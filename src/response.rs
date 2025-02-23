/// Helper structs for creating and parsing the response from the API
/// 
/// Some notes: Response is always in the format Vec of Choices, where each choice has a delta
/// The delta is a message, which has a role and content
///
/// RequestBody is the struct that is used to send the request to the API
/// It has a model, messages and a stream field
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
    pub content: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
}

#[derive(Deserialize)]
pub struct APIResponse {
    pub data: Vec<Model>,
}

#[derive(Serialize)]
pub struct RequestBody {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

impl RequestBody {
    pub fn new(model: String, messages: Vec<Message>, stream: bool) -> Self {
        RequestBody {
            model,
            messages,
            stream,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde () {
        // sample taken from openai
        let json_response = r#"
            {
                "object": "list",
                "data": [
                    {
                    "id": "model-id-0",
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "organization-owner"
                    },
                    {
                    "id": "model-id-1",
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "organization-owner"
                    },
                    {
                    "id": "model-id-2",
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "openai"
                    }
                ],
                "object": "list"
            }
        "#;
        let parsed: APIResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(parsed.data.len(), 3);
    }

}