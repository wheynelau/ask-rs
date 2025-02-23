/// Handles the API calling
use serde_json;
use std::env;

use reqwest::blocking::{Client, Response};
use std::io::{BufRead, BufReader};

use crate::config;
use crate::response::{self,RequestBody, Message, APIResponse};

fn check_exists (model:&str , models: &APIResponse) -> bool {
    models.data.iter().any(|m| m.id == model)
}

fn create_endpoint (legacy_completions: &bool, base_url: &str) -> String {
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

pub fn check_models (base_url: &str, api_key: &str, model: &str) -> Result<(), Box<dyn std::error::Error>> {

    // Make the GET request
    let client = Client::new();
    let body: APIResponse = client
        .get(format!("{}/models", base_url))
        .bearer_auth(api_key)
        .send()?
        .json()?;

    // Check if the model exists
    let model_exists = check_exists(model, &body);
    
    if !model_exists {
        return Err(format!("Model {} does not exist, available models are: {:?}", model, body.data).into());
    }
    Ok(())

}

pub fn chat(prompt: String) -> Result<(), Box<dyn std::error::Error>> {

    let config: config::Config = config::Config::load()?;

    let api_key = env::var("API_KEY")?;
    let body = RequestBody::new(config.model, 
        vec![
        Message {
            role: Some(config.system_role),
            content: Some(config.system_prompt),
        },
        Message {
            role: Some("user".to_string()),
            content: Some(prompt),
        }
    ],
    true);

    let client = Client::new();
    let endpoint = create_endpoint(&config.legacy_completions, &config.base_url);
    let response: Response = client
        .post(&endpoint)
        .bearer_auth(&api_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!(body))
        .send()?;

    // Check response code of the API
    if !response.status().is_success() {
        return Err(format!("API returned an error: {:#?}", response).into());
    }

    // If the DEBUG environment variable is set, print the response
    if let Ok(debug) = env::var("DEBUG") {
        if debug == "1" || debug == "true" {
            println!("{:?}", response);
        }
    }
    // Process the streamed response synchronously
    let reader = BufReader::new(response);
    for line in reader.lines() {
        match line {
            Ok(chunk) => {
                
                if chunk.contains("[DONE]") {
                    return Ok(());
                }
                if chunk.starts_with("data: ") {
                    let chunk = chunk.replace("data: ", "");
                    let chunk: response::Response = serde_json::from_str(&chunk)?;
                    match chunk.choices[0].delta.content.as_ref() {
                        Some(content) => print!("{}", content),
                        None => println!(),
                    }
                }
            }
            Err(err) => eprintln!("Error reading chunk: {}", err),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_check_models() {
        let base_url = "https://api.openai.com/v1/";
        let api_key = env::var("API_KEY").expect("API_KEY not set");
        let model = "gpt-3.5-turbo";
        assert!(check_models(base_url, &api_key, model).is_ok());
    }

    #[test]
    fn check_invalid_model() {
        let api_response = APIResponse {
            data: vec![
                response::Model { id: "model-id-0".to_string() },
                response::Model { id: "model-id-1".to_string() },
                response::Model { id: "model-id-2".to_string() },
            ]
        };
        let model = "gpt-3.5-turbo";
        assert!(!check_exists(model, &api_response));
    }

}