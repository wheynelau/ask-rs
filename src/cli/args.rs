use crate::services::schema::ReasoningEffort;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable configuration mode
    #[arg(long, help = "Configure the application")]
    pub configure: bool,

    #[arg(
        short = 'r',
        long,
        help = "Reasoning effort level (0-3).",
        long_help = "Set the reasoning effort level:\n\
                     0 - No reasoning\n\
                     1 - Low reasoning effort\n\
                     2 - Medium reasoning effort\n\
                     3 - High reasoning effort",
        default_value = "0"
    )]
    pub reasoning: ReasoningEffort,

    pub input: Option<String>,
}
