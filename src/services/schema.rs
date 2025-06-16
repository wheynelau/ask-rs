use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Response {
    pub choices: Vec<Choice>,
}
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub delta: Message,
}

//TODO: Refactor the non streaming and streaming responses so they can be shared
#[derive(Debug, Deserialize)]
pub struct NonStreamingResponse {
    pub choices: Vec<NonStreamingChoice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct NonStreamingChoice {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
}

#[derive(Deserialize)]
pub struct APIResponse {
    pub data: Vec<Model>,
}
