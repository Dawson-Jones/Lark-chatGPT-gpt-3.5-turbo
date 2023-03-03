const LARK_HOST: &str = "https://open.feishu.cn";
const Verification_Token: &str = "iiTAghdddddddL0uvgMOnxSdGOnC";
const APP_ID: &str = "cli_a48a68fdjs7b8500d";
const APP_SECRET: &str = "Yj8SWiGsoxGk0yj2txYHmcjofdsOSj";

use std::{net::SocketAddr};

use crate::chatgpt::request_for_chatgpt;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub token: String,
    #[serde(rename = "create_time")]
    pub create_time: String,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
    #[serde(rename = "app_id")]
    pub app_id: String,
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
    if msg.header.event_type != "im.message.receive_v1" {
        return (StatusCode::BAD_REQUEST, format!("bad request"));
    }

    tokio::spawn(async move {
        if msg.event.message.chat_type == "p2p" {
            if msg.event.message.message_type != "text" {
                log::warn!("not support");
                return;
            }

            let content = &msg.event.message.content;
            log::info!("{}", content);
            if let Ok(obj) = serde_json::from_str::<serde_json::Value>(content) {
                match make_response(msg, obj["text"].to_string()).await {
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
        } else {
            log::warn!("not p2p");
            return;
        }
    });

    return (StatusCode::OK, format!("Ok"));
}

async fn make_response(msg: ReceiveMessage, question: String,) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_id = msg.event.sender.sender_id.user_id;
    // let event_id = msg.header.event_id;
    let msg_id = msg.event.message.message_id;
    let open_id = msg.event.sender.sender_id.open_id;

    let tenant_access_token = get_authorization().await?;
    let answer = request_for_chatgpt(user_id, question).await?;
    let msg_uri = format!("{}/open-apis/im/v1/messages/{}/reply", LARK_HOST, msg_id);
    // let msg_uri = format!("{}/open-apis/im/v1/messages?receive_id_type={}", LARK_HOST, "open_id");

    let content = Content {
        text: answer
    };

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
    let cli = reqwest::Client::new();
    let TENANT_ACCESS_TOKEN_URI = "/open-apis/auth/v3/tenant_access_token/internal";
    let url = format!("{}{}", LARK_HOST, TENANT_ACCESS_TOKEN_URI);

    let res: serde_json::Value = cli
        .post(url)
        .json(&serde_json::json!({
            "app_id": APP_ID,
            "app_secret": APP_SECRET,
        }))
        .send()
        .await?
        .json()
        .await?;

    log::info!("tenant_access_token: {:#?}", res);
    let pat: &[_] = &['"'];
    Ok(res["tenant_access_token"].to_string().trim_matches(pat).to_string())
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
