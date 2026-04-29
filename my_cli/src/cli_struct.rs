use crate::enums_file::{Commands};
use clap::Parser;


#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

