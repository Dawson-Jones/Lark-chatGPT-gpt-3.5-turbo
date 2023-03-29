use crate::{chatgpt, CONFIG};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

const TENANT_ACCESS_TOKEN_URI: &str = "/open-apis/auth/v3/tenant_access_token/internal";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum RootBody {
    UrlVerification(UrlVerification),
    RecieveMessage(ReceiveMessage),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct UrlVerification {
    challenge: String,
    token: String,
    r#type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct UrlVerificationResp {
    challenge: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiveMessage {
    pub schema: String,
    pub header: Header,
    pub event: Event,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "create_time")]
    pub create_time: String,
    pub token: String,
    #[serde(rename = "app_id")]
    pub app_id: String,
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub message: Message,
    pub sender: Sender,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "chat_id")]
    pub chat_id: String,
    #[serde(rename = "chat_type")]
    pub chat_type: String,
    pub content: String,
    #[serde(rename = "create_time")]
    pub create_time: String,
    #[serde(rename = "message_id")]
    pub message_id: String,
    #[serde(rename = "message_type")]
    pub message_type: String,
    #[serde(rename = "parent_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "root_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mention>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub key: String,
    pub id: Id,
    pub name: String,
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    #[serde(rename = "union_id")]
    pub union_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "open_id")]
    pub open_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    #[serde(rename = "sender_id")]
    pub sender_id: SenderId,
    #[serde(rename = "sender_type")]
    pub sender_type: String,
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SenderId {
    #[serde(rename = "open_id")]
    pub open_id: String,
    #[serde(rename = "union_id")]
    pub union_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Content {
    text: String,
}

async fn url_verification(verified: UrlVerification) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(UrlVerificationResp {
            challenge: verified.challenge,
        }),
    )
}

async fn handle_msg(msg: ReceiveMessage) -> impl IntoResponse {
    log::info!("{}", serde_json::to_string(&msg).unwrap());

    if msg.header.event_type != "im.message.receive_v1" {
        return (StatusCode::BAD_REQUEST, format!("bad request"));
    }

    tokio::spawn(async move {
        if msg.event.message.message_type != "text" {
            log::warn!(
                "unsupported message type: {}",
                msg.event.message.message_type
            );
            return;
        }

        if msg.event.message.chat_type == "p2p" {
            let content = &msg.event.message.content;
            log::info!("{}", content);

            // if let Ok(obj) = serde_json::from_str::<serde_json::Value>(content) {
            if let Ok(obj) = serde_json::from_str::<Content>(content) {
                // match make_response(msg, obj["text"].to_string()).await {
                match make_response(msg, obj.text).await {
                    Ok(_) => return,
                    Err(e) => {
                        log::error!("make response: {}", e.to_string());
                        return;
                    }
                }
            } else {
                log::warn!("parse json error");
                return;
            }
        } else if msg.event.message.chat_type == "group" {
            if let Some(mentions) = &msg.event.message.mentions {
                if mentions.len() <= 0 {
                    log::warn!(
                        "mention length is {}, {}",
                        mentions.len(),
                        serde_json::to_string(mentions).unwrap()
                    );
                    return;
                }

                if mentions[0].name != CONFIG.feishu.bot_name {
                    log::info!("bot name is not the first mentioin name");
                    return;
                }

                let content = &msg.event.message.content;
                if let Ok(obj) = serde_json::from_str::<Content>(content) {
                    let text = obj.text;
                    log::info!("text: {}", text);

                    let text = text.replace("@_user_1", "");
                    match make_response(msg, text).await {
                        Ok(_) => return,
                        Err(e) => {
                            log::error!("make response: {}", e.to_string());
                            return;
                        }
                    }
                } else {
                    log::warn!("parse json error: {}", content);
                    return;
                }
            } else {
                log::info!("not mention gptbot");
                return;
            }
        } else {
            log::warn!("unsupported chat_type: {}", msg.event.message.chat_type);
            return;
        }
    });

    return (StatusCode::OK, format!("Ok"));
}

async fn make_response(
    msg: ReceiveMessage,
    question: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_id = msg.event.sender.sender_id.user_id;
    // let event_id = msg.header.event_id;
    let msg_id = msg.event.message.message_id;
    let open_id = msg.event.sender.sender_id.open_id;

    let tenant_access_token = get_authorization().await?;
    let answer = chatgpt::request_for_chatgpt(user_id, question).await?;
    let msg_uri = format!(
        "{}/open-apis/im/v1/messages/{}/reply",
        &CONFIG.feishu.lark_host, msg_id
    );
    // let msg_uri = format!("{}/open-apis/im/v1/messages?receive_id_type={}", LARK_HOST, "open_id");

    let content = Content { text: answer };

    let content = serde_json::to_string(&content).unwrap();

    let body = serde_json::json!({
        "receive_id": open_id,
        "content": content.to_string(),
        "msg_type": "text",
        "uuid": Uuid::new_v4().to_string()
    });
    log::info!("body: {}", body);

    let resp: serde_json::Value = reqwest::Client::new()
        .post(msg_uri)
        .header("Authorization", format!("Bearer {}", tenant_access_token))
        .header("Content-Type", "application/json; charset=utf-8")
        .body(body.to_string())
        // .json(&serde_json::json!())
        .send()
        .await?
        .json()
        .await?;

    if resp["code"] != 0 {
        log::error!("request response error: {:#?}", resp);
    }

    log::info!("response succ");
    Ok(())
}

async fn get_authorization() -> Result<String, reqwest::Error> {
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TenantResp {
        pub code: i64,
        pub msg: String,
        #[serde(rename = "tenant_access_token")]
        pub tenant_access_token: String,
        pub expire: i64,
    }

    let cli = reqwest::Client::new();
    let url = format!("{}{}", &CONFIG.feishu.lark_host, TENANT_ACCESS_TOKEN_URI);

    let res: TenantResp = cli
        .post(url)
        .json(&serde_json::json!({
            "app_id": &CONFIG.feishu.app_id,
            "app_secret": &CONFIG.feishu.app_secret,
        }))
        .send()
        .await?
        .json()
        .await?;

    log::info!("tenant_access_token: {:#?}", res);
    Ok(res.tenant_access_token)
}

async fn root_handler(Json(body): Json<RootBody>) -> Response {
    match body {
        RootBody::UrlVerification(verified) => url_verification(verified).await.into_response(),
        RootBody::RecieveMessage(msg) => handle_msg(msg).await.into_response(),
    }
}

pub fn route_init() -> Router {
    Router::new().route("/", post(root_handler))
}

pub async fn run(app: Router, addr: SocketAddr) {
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
