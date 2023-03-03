pub mod chatgpt;
pub mod feishu;

use std::collections::HashMap;
use std::{collections::VecDeque, sync::RwLock};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Msg {
    role: String,
    content: String,
}

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
pub static DB: Lazy<RwLock<HashMap<String, VecDeque<Msg>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
