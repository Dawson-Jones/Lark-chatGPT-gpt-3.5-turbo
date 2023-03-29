use crate::{Msg, CONFIG, DB};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, str};

#[derive(Serialize, Deserialize, Debug)]
struct GptBody {
    model: String,
    messages: VecDeque<Msg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GptResp {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub message: String,
    pub r#type: String,
    pub param: String,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GptError {
    #[serde(rename = "error")]
    pub errinfo: ErrorInfo
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum GptRespError {
    Resp(GptResp),
    Err(GptError)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub index: i64,
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

pub async fn request_for_chatgpt(
    user_id: String,
    question: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let trimed_question = question.trim();

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
    let trimed_anwser = anwser.trim();

    let mut map = DB.write().unwrap();
    let context = map.get_mut(&user_id).unwrap();
    context.push_back(Msg {
        role: "assistant".to_string(),
        content: trimed_anwser.to_string(),
    });

    while context.len() - 1 > CONFIG.gpt.context_limit * 2 {
        context.swap(0, 2);
        context.pop_front();
        context.pop_front();
    }

    log::info!("context: {:#?}", context);
    Ok(trimed_anwser.to_string())
}

async fn request_to_gpt(user_id: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let gpt_body = GptBody {
        model: CONFIG.gpt.body.model.clone(),
        messages: DB.read().unwrap().get(user_id).unwrap().clone(),
        temperature: CONFIG.gpt.body.temperature,
        max_tokens: CONFIG.gpt.body.max_tokens,
    };

    let obj: GptRespError = reqwest::Client::new()
        .post(&CONFIG.gpt.url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", &CONFIG.gpt.api_key))
        .json(&gpt_body)
        .send()
        .await?
        .json()
        .await?;

    log::info!("json: {:#?}", obj);
    match obj {
        GptRespError::Resp(resp) => {
            Ok(resp.choices[0].message.content.clone())
        },
        GptRespError::Err(err) => {
            log::error!("respose error: {:#?}", err);
            Ok(err.errinfo.message)
        },
    }
}
