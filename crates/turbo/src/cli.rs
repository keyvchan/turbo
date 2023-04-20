use std::path::PathBuf;

use clap::Parser;
use tracing::debug;

#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    pub mode: Option<String>,
}

impl Cli {
    /// parse the command line arguments and return a new App instance
    pub fn new() -> Cli {
        let cli = Cli::parse();

        if let Some(config_path) = cli.config.as_deref() {
            debug!("Value for config: {}", config_path.display());
        }

        // You can see how many times a particular flag or argument occurred
        // Note, only flags can have multiple occurrences
        match cli.debug {
            0 => debug!("Debug mode is off"),
            1 => debug!("Debug mode is kind of on"),
            2 => debug!("Debug mode is on"),
            _ => debug!("Don't be crazy"),
        }
        cli
    }
}
