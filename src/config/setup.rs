use cliclack::{confirm, input, intro, outro};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

use crate::services::api::check_models;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "Config::default_base_url")]
    pub base_url: String,

    #[serde(default)]
    pub legacy_completions: bool,

    #[serde(default = "Config::default_model")]
    pub model: String,

    #[serde(default = "Config::default_thinking_model")]
    pub thinking_model: String,

    #[serde(default = "Config::default_system_prompt")]
    pub system_prompt: String,

    #[serde(default = "Config::default_system_role")]
    pub system_role: String,

    #[serde(default = "Config::default_stream")]
    pub stream: bool,
}

impl Config {
    // Associated functions for defaults (cleaner than free functions)
    fn default_base_url() -> String {
        "https://generativelanguage.googleapis.com/v1beta/openai/".to_string()
    }

    fn default_model() -> String {
        "gemini-2.0-flash".to_string()
    }

    fn default_thinking_model() -> String {
        "gemini-2.5-flash-preview-05-20".to_string()
    }

    fn default_system_prompt() -> String {
        "The user will be asking questions via a terminal, so there is no need for markdown formatting.".to_string()
    }

    fn default_system_role() -> String {
        "system".to_string()
    }
    fn default_stream() -> bool {
        true
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = get_askconfig_path();
        let config_str = std::fs::read_to_string(&path)?;
        let config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = get_askconfig_path();
        let file = std::fs::File::create(&path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: Self::default_base_url(),
            legacy_completions: false,
            model: Self::default_model(),
            thinking_model: Self::default_thinking_model(),
            system_prompt: Self::default_system_prompt(),
            system_role: Self::default_system_role(),
            stream: Self::default_stream(),
        }
    }
}

pub fn get_askconfig_path() -> PathBuf {
    env::var("ASKCONFIG_PATH")
        .map(PathBuf::from)
        .or_else(|_| {
            env::var("HOME").map(|home| {
                let mut path = PathBuf::from(home);
                path.push(".askconfig");
                path
            })
        })
        .unwrap_or_else(|_| PathBuf::from("./.askconfig"))
}

fn load_existing_config() -> Config {
    get_askconfig_path()
        .exists()
        .then(|| Config::load().ok())
        .flatten()
        .unwrap_or_default()
}

fn prompt_base_url(current: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input("What is the base_url?")
        .default_input(current)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Base URL cannot be empty")
            } else if !input.starts_with("http") {
                Err("Path should be a valid URL")
            } else {
                Ok(())
            }
        })
        .interact()?)
}

fn prompt_string(prompt: &str, current: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input(prompt).default_input(current).interact()?)
}

fn prompt_bool(prompt: &str, default: bool) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(confirm(prompt).initial_value(default).interact()?)
}

async fn validate_model_if_requested(
    base_url: &str,
    model: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let skip_validate = prompt_bool(
        "Do you want to skip model validation? This defaults to true due to the different formats of the models endpoint.",
        true
    )?;

    if !skip_validate {
        let api_key = env::var("ASK_API_KEY")?;
        check_models(base_url, &api_key, model).await?;
    }

    Ok(())
}

pub async fn configure() -> Result<(), Box<dyn std::error::Error>> {
    intro("Welcome to the configuration mode")?;

    let current_config = load_existing_config();
    if get_askconfig_path().exists() {
        println!("Here is the current configuration: {:#?}", current_config);
    }

    let base_url = prompt_base_url(&current_config.base_url)?;
    let model = prompt_string(
        "What model do you want to use? Smaller models are recommended: ",
        &current_config.model,
    )?;
    let thinking_model = prompt_string(
        "What is the thinking model? (leave empty if you don't want to use thinking) ",
        &current_config.thinking_model,
    )?;
    let system_prompt =
        prompt_string("What is the system prompt? ", &current_config.system_prompt)?;
    let system_role = prompt_string("What is the system role? ", &current_config.system_role)?;
    let legacy_completions = prompt_bool(
        "Do you want to use legacy completions?",
        current_config.legacy_completions,
    )?;
    let stream = prompt_bool("Do you want to enable streaming?", current_config.stream)?;

    validate_model_if_requested(&base_url, &model).await?;

    let new_config = Config {
        base_url,
        legacy_completions,
        model,
        thinking_model,
        system_prompt,
        system_role,
        stream,
    };

    new_config.save()?;
    outro("Configuration complete")?;
    Ok(())
}
