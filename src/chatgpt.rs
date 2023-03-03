use crate::{Msg, DB};
use serde::{Deserialize, Serialize};
use std::{collections::{VecDeque}, str};

const URL: &str = "https://api.openai.com/v1/chat/completions";
const API_KEY: &str = "xxxxxxxxxx";
const CONTEXT_LIMIT: usize = 5;

pub async fn request_for_chatgpt(
    user_id: String,
    question: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // let trimed_question = question.trim();
    let pat: &[_] = &['"', '\\', '\r', '\n', ' '];
    let trimed_question = question.trim_matches(pat);
    {
        let mut map = DB.write().unwrap();
        if let Some(context) = map.get_mut(&user_id) {
            context.push_back(Msg {
                role: "user".to_string(),
                content: trimed_question.to_string(),
            });
        } else {
            map.insert(
                user_id.clone(),
                VecDeque::from(vec![
                    Msg {
                        role: "system".to_string(),
                        content: "You are a helpful assistant.".to_string(),
                    },
                    Msg {
                        role: "user".to_string(),
                        content: trimed_question.to_string(),
                    },
                ]),
            );
        }
    }

    let anwser = request_to_gpt(&user_id).await?;
    // let trimed_anwser = anwser.trim();
    let trimed_anwser = anwser.trim_matches(pat);

    let mut map = DB.write().unwrap();
    let context = map.get_mut(&user_id).unwrap();
    context.push_back(Msg {
        role: "assistant".to_string(),
        content: trimed_anwser.to_string(),
    });

    while context.len() > CONTEXT_LIMIT * 2 {
        context.swap(0, 2);
        context.pop_front();
        context.pop_front();
    }

    // println!("context: {:#?}", context);

    Ok(trimed_anwser.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct GptBody {
    model: String,
    messages: VecDeque<Msg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

async fn request_to_gpt(user_id: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let gpt_body = GptBody {
        model: "gpt-3.5-turbo".to_string(),
        messages: DB.read().unwrap().get(user_id).unwrap().clone(),
        temperature: Some(0.8),
        max_tokens: Some(2048),
    };

    // println!("body:--------------");
    // println!("{}", serde_json::to_string(&gpt_body).unwrap());

    let resp: serde_json::Value = reqwest::Client::new()
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .json(&gpt_body)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp["choices"][0]["message"]["content"].to_string())
}
