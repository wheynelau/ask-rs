use clap::Parser;
use std::io::{self, Read};

pub mod api;
pub mod args;
pub mod config;
pub mod response;
pub mod prompt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Cli::parse();

    if args.configure {
        // This is configuration mode
        config::configure().await?;
    } else {
        // check if the configuration file exists
        if !config::get_askconfig_path().exists() {
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

        let user_question = args.input.clone();

        let config: config::Config = config::Config::load()?;

        let prompt = prompt::format_prompt(
            &config.system_prompt,
            stdin_content.as_deref(),
            &user_question
        );

        api::chat(prompt).await?;
    }

    Ok(())
}
