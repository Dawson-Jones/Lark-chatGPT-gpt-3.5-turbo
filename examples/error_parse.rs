
use serde::{Deserialize, Serialize};

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GptResp {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<Choice>,
    pub usage: Usage,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum GptRespError {
    Resp(GptResp),
    Err(GptError)
}


fn main() {
    let json_str = r#"{ "error": { "message": "This model's maximum context length is 4097 tokens. However, you requested 4189 tokens (2141 in the messages, 2048 in the completion). Please reduce the length of the messages or completion.", "type": "invalid_request_error", "param": "messages", "code": "context_length_exceeded" } }"#;

    let response: GptRespError = serde_json::from_str(json_str).unwrap();


    // let json_str = r#"
    // {"id":"chatcmpl-6z0LxA1AKJItZMm8VSDaIuUojHx9A","object":"chat.completion","created":1679997385,"model":"gpt-3.5-turbo-0301","usage":{"prompt_tokens":530,"completion_tokens":9,"total_tokens":539},"choices":[{"message":{"role":"assistant","content":"Hello! How can I help you today?"},"finish_reason":"stop","index":0}]}
    // "#;

    println!("{:#?}", response);
}