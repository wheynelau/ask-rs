use super::additional_config::gemini_config;
use super::schema::Message;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RequestBody {
    model: String,
    messages: Vec<Message>,
    stream: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,

    extra_body: serde_json::Value,
}
impl RequestBody {
    pub fn builder() -> RequestBodyBuilder {
        RequestBodyBuilder::new()
    }
}

#[derive(Default, Clone)]
pub struct RequestBodyBuilder {
    pub(super) model: Option<String>,
    pub(super) messages: Option<Vec<Message>>,
    pub(super) stream: Option<bool>,
    pub(super) reasoning_effort: Option<String>,
    pub(super) show_reasoning: bool,
    pub(super) extra_body: Option<serde_json::Value>,
}

impl RequestBodyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = Some(messages);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn reasoning_effort(mut self, reasoning_effort: ReasoningEffort) -> Self {
        self.reasoning_effort = reasoning_effort.as_option_string();
        self
    }
    // This is not used yet, just a placeholder for future use
    #[allow(dead_code)]
    pub fn extra_body(mut self, extra_body: serde_json::Value) -> Self {
        self.extra_body = Some(extra_body);
        self
    }

    pub fn show_reasoning(mut self, show_reasoning: bool) -> Self {
        self.show_reasoning = show_reasoning;
        self
    }

    fn validate(mut self) -> Self {
        // This handles additional configurations or validations if needed
        if let Some(model) = &self.model {
            if model.starts_with("gemini") {
                // gemini cannot accept both reasoning effort and extra body
                self = gemini_config(self);
            }
        }
        self
    }

    pub fn build(self) -> Result<RequestBody, String> {
        let builder = self.validate();
        let model = builder.model.ok_or("model must be set")?;
        let messages = builder.messages.ok_or("messages must be set")?;
        let stream = builder.stream.unwrap_or(false); // Default to false if not set
        let reasoning_effort = builder.reasoning_effort;

        let stream_options = if stream {
            Some(serde_json::json!({"include_usage": true}))
        } else {
            None
        };

        let extra_body = builder
            .extra_body
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new())); // Default to empty object

        Ok(RequestBody {
            model,
            messages,
            stream,
            stream_options,
            reasoning_effort,
            extra_body,
        })
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
    pub fn as_option_string(&self) -> Option<String> {
        match self {
            ReasoningEffort::None => None,
            ReasoningEffort::Low => Some("low".to_string()),
            ReasoningEffort::Medium => Some("medium".to_string()),
            ReasoningEffort::High => Some("high".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_gemini_model_validation() {
        let messages = vec![Message {
            role: Some("user".to_string()),
            content: Some("Hello".to_string()),
        }];

        let request = RequestBodyBuilder::new()
            .model("gemini-pro".to_string())
            .messages(messages.clone())
            .reasoning_effort(ReasoningEffort::Medium)
            .build()
            .unwrap();

        assert_eq!(request.reasoning_effort, Some("medium".to_string()));

        // When showing reasoning, it should not have extra body
        let par_request = RequestBodyBuilder::new()
            .model("gemini-pro".to_string())
            .messages(messages)
            .show_reasoning(true)
            .reasoning_effort(ReasoningEffort::High);

        let request = par_request.clone().build().unwrap();

        assert_eq!(request.reasoning_effort, None);
        assert_eq!(
            request.extra_body,
            json!({
                "google": {
                    "thinking_config": {
                        "thinkingBudget": 24576,
                        "include_thoughts": true
                    }
                }
            })
        );

        let request = par_request
            .clone()
            .reasoning_effort(ReasoningEffort::Medium)
            .build()
            .unwrap();

        assert_eq!(
            request.extra_body,
            json!({
                "google": {
                    "thinking_config": {
                        "thinkingBudget": 8192,
                        "include_thoughts": true
                    }
                }
            })
        );

        let request = par_request
            .clone()
            .reasoning_effort(ReasoningEffort::Low)
            .build()
            .unwrap();

        assert_eq!(
            request.extra_body,
            json!({
                "google": {
                    "thinking_config": {
                        "thinkingBudget": 512,
                        "include_thoughts": true
                    }
                }
            })
        );
    }
}
