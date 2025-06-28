use reqwest::Client;
use std::env;
use termimad::crossterm::style::Color::*;
use termimad::*;

use super::request::{ReasoningEffort, RequestBody};
use super::schema::{APIResponse, Message, Usage};
use super::spinner;
use super::stream::stream;
use crate::cli::Cli;
use crate::config::setup as config;
use crate::services::schema::NonStreamingResponse;

fn check_exists(model: &str, models: &APIResponse) -> bool {
    models.data.iter().any(|m| m.id == model)
}

fn load_api_key() -> Result<String, Box<dyn std::error::Error>> {
    env::var("ASK_API_KEY").map_err(|_| "ASK_API_KEY environment variable not set".into())
}

fn select_model(config: &config::Config, reasoning: &ReasoningEffort) -> String {
    if *reasoning != ReasoningEffort::None {
        config.thinking_model.clone()
    } else {
        config.model.clone()
    }
}

fn create_endpoint(legacy_completions: &bool, base_url: &str) -> String {
    let endpoint = match legacy_completions {
        true => "completions",
        false => "chat/completions",
    };

    if base_url.ends_with("/") {
        format!("{}{}", base_url, endpoint)
    } else {
        format!("{}/{}", base_url, endpoint)
    }
}

fn build_request_body(
    model: String,
    prompt: String,
    stream: bool,
    reasoning: ReasoningEffort,
    verbose: bool,
) -> Result<RequestBody, Box<dyn std::error::Error>> {
    Ok(RequestBody::builder()
        .model(model)
        .messages(vec![Message {
            role: Some("user".to_string()),
            content: Some(prompt),
        }])
        .stream(stream)
        .reasoning_effort(reasoning)
        .show_reasoning(verbose)
        .build()?)
}
async fn build_request(
    config: &config::Config,
    api_key: &str,
    body: RequestBody,
    reasoning: &ReasoningEffort,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let client = Client::new();
    let endpoint = create_endpoint(&config.legacy_completions, &config.base_url);
    // Spinner setup
    let spinner = if reasoning == &ReasoningEffort::None {
        spinner::create_api_spinner()
    } else {
        spinner::create_reasoning_spinner()
    };
    let response = client
        .post(&endpoint)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!(body))
        .send()
        .await?;
    spinner.finish();
    // Check response code of the API
    if !response.status().is_success() {
        return Err(format!("API returned an error: {:#?}", response).into());
    }
    Ok(response)
}

async fn handle_response(
    response: reqwest::Response,
    stream_enabled: bool,
) -> Result<Option<Usage>, Box<dyn std::error::Error>> {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.quote_mark.set_fg(Yellow);
    if stream_enabled {
        stream(response, &skin).await
    } else {
        let response_text = response.text().await?;
        let response_json: NonStreamingResponse = serde_json::from_str(&response_text)?;
        let response_string = response_json.choices[0]
            .message
            .content
            .as_ref()
            .expect("No content in response");
        println!("{}", skin.inline(response_string));
        Ok(Some(response_json.usage))
    }
}
pub async fn check_models(
    base_url: &str,
    api_key: &str,
    model: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Make the GET request
    let client = Client::new();
    let body: APIResponse = client
        .get(format!("{}/models", base_url))
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;

    // Check if the model exists
    let model_exists = check_exists(model, &body);

    if !model_exists {
        return Err(format!(
            "Model {} does not exist, available models are: {:?}",
            model, body.data
        )
        .into());
    }
    Ok(())
}

pub async fn chat(prompt: String, args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let config: config::Config = config::Config::load()?;
    let api_key = load_api_key()?;
    let model = select_model(&config, &args.reasoning);
    let body = build_request_body(model, prompt, config.stream, args.reasoning, args.verbose)?;

    // dbg the body as a json string if the DEBUG environment variable is set
    if let Ok(debug) = env::var("DEBUG") {
        if debug == "1" || debug == "true" {
            println!("Request Body: {:#?}", body);
        }
    }

    let response = build_request(&config, &api_key, body, &args.reasoning).await?;

    // If the DEBUG environment variable is set, print the response
    if let Ok(debug) = env::var("DEBUG") {
        if debug == "1" || debug == "true" {
            println!("{:?}", response);
        }
    }
    let usage = handle_response(response, config.stream).await?;
    if args.verbose {
        if let Some(usage_info) = usage {
            println!(
                "\n\nUsage: prompt_tokens: {}, completion_tokens: {}, total_tokens: {}",
                usage_info.prompt_tokens, usage_info.completion_tokens, usage_info.total_tokens
            );
        } else {
            println!("No usage information available.");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::schema::{APIResponse, Model};
    use std::env; // Import Model and APIResponse for tests

    #[tokio::test]
    #[ignore = "requires real API key and network"]
    async fn test_check_models() {
        let base_url = "https://api.openai.com/v1/";
        let api_key = env::var("ASK_API_KEY").expect("ASK_API_KEY not set");
        let model = "gpt-3.5-turbo";
        assert!(check_models(base_url, &api_key, model).await.is_ok());
    }

    #[tokio::test]
    async fn check_invalid_model() {
        let api_response = APIResponse {
            data: vec![
                Model {
                    id: "model-id-0".to_string(),
                },
                Model {
                    id: "model-id-1".to_string(),
                },
                Model {
                    id: "model-id-2".to_string(),
                },
            ],
        };
        let model = "gpt-3.5-turbo";
        assert!(!check_exists(model, &api_response));
    }

    #[test]
    fn test_serde_openai() {
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
        // Since APIResponse and Model are now in src/services/schema, we need to import them for the test
        use crate::services::schema::APIResponse; // Only APIResponse is directly used here
        let parsed: APIResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(parsed.data.len(), 3);
    }
}
