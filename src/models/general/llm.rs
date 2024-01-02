use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Deserialize, Debug)]
pub struct APIMessage {
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct APIChoice {
    pub message: APIMessage,
}

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}
