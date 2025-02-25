use clap::Parser;
// A simple CLI application

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable configuration mode
    #[arg(long,
        help = "Configure the application")]
    pub configure: bool,

    /// Input string
    #[arg(num_args = 2..)]
    pub inputs: Vec<String>,
}