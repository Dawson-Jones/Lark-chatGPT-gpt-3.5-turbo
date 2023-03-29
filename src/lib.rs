pub mod chatgpt;
pub mod feishu;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::{collections::VecDeque, sync::RwLock};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Msg {
    role: String,
    content: String,
}

pub static DB: Lazy<RwLock<HashMap<String, VecDeque<Msg>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

const FILE_NAME: &str = "config.toml";
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let contents = fs::read_to_string(FILE_NAME).unwrap();
    let config = toml::from_str(&contents).unwrap();

    log::info!("{:#?}", config);
    config
});

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub feishu: Feishu,
    pub gpt: Gpt,
    #[serde(rename = "Host")]
    pub host: Host,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feishu {
    #[serde(rename = "LARK_HOST")]
    pub lark_host: String,
    #[serde(rename = "Verification_Token")]
    pub verification_token: String,
    #[serde(rename = "APP_ID")]
    pub app_id: String,
    #[serde(rename = "APP_SECRET")]
    pub app_secret: String,
    #[serde(rename = "BOT_NAME")]
    pub bot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gpt {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "API_KEY")]
    pub api_key: String,
    #[serde(rename = "CONTEXT_LIMIT")]
    pub context_limit: usize,
    pub body: Body,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(rename = "max_tokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    #[serde(rename = "IP")]
    pub ip: String,
    #[serde(rename = "Port")]
    pub port: u16,
}
