use clap::Parser;

pub mod args;
pub mod config;
pub mod api;
pub mod response;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Cli::parse();

    if args.configure {
        // This is configuration mode
        config::configure()?;
    } 
    else {
        // check if the configuration file exists
        if !config::get_askconfig_path().exists() {
            eprintln!("Configuration file does not exist. Please run the application in configuration mode first.");
            std::process::exit(1);
        }
        let input_string = args.inputs;

        api::chat(input_string.join(" "))?;
    }

    Ok(())
}