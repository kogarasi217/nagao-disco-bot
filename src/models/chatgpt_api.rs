use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<HashMap<String, String>>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub choices: Vec<Choices>,
    pub usage: Usage,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Choices {
    pub index: u32,
    pub message: HashMap<String, String>,
    pub finish_reason: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
