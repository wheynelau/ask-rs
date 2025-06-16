use clap::Parser;
use std::io::{self, Read};
pub mod cli;
pub mod config;
pub mod models;
pub mod services;

use config::setup as config_setup;
use config::setup::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();

    if args.configure {
        // This is configuration mode
        config_setup::configure().await?;
    } else {
        // check if the configuration file exists
        if !config_setup::get_askconfig_path().exists() {
            eprintln!("Configuration file does not exist. Please run the application in configuration mode first.");
            std::process::exit(1);
        }

        let stdin_content = if !atty::is(atty::Stream::Stdin) {
            let mut piped_input = String::new();
            io::stdin().read_to_string(&mut piped_input)?;
            Some(piped_input)
        } else {
            None
        };

        let user_question = match &args.input {
            Some(input) => input,
            None => {
                eprintln!("Please enter your question.");
                std::process::exit(1);
            }
        };

        let config: Config = Config::load()?;

        let prompt = models::prompt::format_prompt(
            &config.system_prompt,
            stdin_content.as_deref(),
            user_question,
        );

        services::api::chat(prompt, args).await?;
    }

    Ok(())
}
