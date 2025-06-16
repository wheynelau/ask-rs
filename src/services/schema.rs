use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Response {
    pub choices: Vec<Choice>,
}
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub delta: Message,
}

#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Serialize, Debug)]
pub struct RequestBody {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    reasoning_effort: Option<String>,
    extra_body: serde_json::Value,
}
#[allow(non_snake_case)]
impl RequestBody {
    pub fn new(
        model: String,
        messages: Vec<Message>,
        stream: bool,
        reasoning_effort: Option<String>,
        extra_body: serde_json::Value,
    ) -> Self {
        RequestBody {
            model,
            messages,
            stream,
            reasoning_effort,
            extra_body,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReasoningEffort {
    /// No reasoning (0)
    None = 0,
    /// Low reasoning effort (1)
    Low = 1,
    /// Medium reasoning effort (2)
    Medium = 2,
    /// High reasoning effort (3)
    High = 3,
}

impl std::str::FromStr for ReasoningEffort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ReasoningEffort::None),
            "1" => Ok(ReasoningEffort::Low),
            "2" => Ok(ReasoningEffort::Medium),
            "3" => Ok(ReasoningEffort::High),
            _ => Err(format!("Invalid reasoning effort: {}", s)),
        }
    }
}

impl ReasoningEffort {
    pub fn to_option_string(&self) -> Option<String> {
        match self {
            ReasoningEffort::None => None,
            ReasoningEffort::Low => Some("low".to_string()),
            ReasoningEffort::Medium => Some("medium".to_string()),
            ReasoningEffort::High => Some("high".to_string()),
        }
    }
}
