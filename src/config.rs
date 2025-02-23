// This rs file is meant to serialize and deserialize configurations
use serde::{Deserialize, Serialize};
use cliclack::{confirm, input, intro, outro};
use std::env;
use std::path::PathBuf;

use crate::api;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub legacy_completions: bool,
    pub model: String,
    pub system_prompt: String,
    pub system_role: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base_url: "https://api.openai.com/v1/".to_string(),
            legacy_completions: false,
            model: "gpt-4o-mini".to_string(),
            system_prompt: "You are a helpful assistant, answer concisely. The user will be asking questions via a terminal, so keep the answers brief.".to_string(),
            system_role: "system".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = get_askconfig_path();
        let config = std::fs::read_to_string(&path)?;
        let config: Config = serde_json::from_str(&config)?;
        Ok(config)
    }
}

pub fn get_askconfig_path() -> PathBuf {
    // Check if the environment variable "ASKCONFIG_PATH" exists.
    match env::var("ASKCONFIG_PATH") {
        Ok(path) => {
            // If it exists, return its value as a PathBuf.
            PathBuf::from(path)
        }
        Err(_) => {
            // If it doesn't exist, default to using $HOME/.askconfig.
            match env::var("HOME") {
                Ok(home_dir) => {
                    let mut path = PathBuf::from(home_dir);
                    path.push(".askconfig");
                    path
                }
                Err(_) => {
                    // If $HOME is not set (why?), return a default path (e.g., "./.askconfig").
                    PathBuf::from("./.askconfig")
                }
            }
        }
    }
}

pub fn configure () -> Result<(), Box<dyn std::error::Error>> {

    intro("Welcome to the configuration mode")?;

    // check if there is a current configuration
    let config = if get_askconfig_path().exists() {
            let config = Config::load()?;
            println!("Here is the current configuration: {:?}", config);
            config
        }
        else {
            Config::default()
        };
    

    let base_url: String = input("What is the base_url?")
        .default_input(&config.base_url)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Base URL cannot be empty")
            } else if !input.starts_with("http") {
                Err("Path should be a valid URL")
            }
            else {
                Ok(())
            }
        }).interact()?;

    let model: String = input("What model do you want to use? Smaller models are recommended: ")
       .default_input(&config.model)
       .interact()?;

    let system_prompt: String = input("What is the system prompt? ")
        .default_input(&config.system_prompt)
        .interact()?;

    let system_role: String = input("What is the system role? ")
        .default_input(&config.system_role)
        .interact()?;

    let legacy_completions: bool = confirm("Do you want to use legacy completions?")
        .initial_value(false)
        .interact()?;
        
    // Allow skipping because I cannot figure out if all endpoints have the models endpoint
    let skip_validate: bool = confirm("Do you want to skip model validation? ")
        .initial_value(true)
        .interact()?;

    // check if the user wants to skip validation
    if !skip_validate {
        // Validate the model

        let api_key = env::var("API_KEY")?;
        api::check_models(&base_url, &api_key, &model)?;
    }

    let config = Config {
        base_url,
        legacy_completions,
        model,
        system_prompt,
        system_role,
    };

    let path = get_askconfig_path();

    // Serialize the configuration to a file
    serde_json::to_writer(std::fs::File::create(&path)?, &config)?;
    
    outro("Configuration complete")?;
    Ok(())
}