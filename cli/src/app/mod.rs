pub mod commands;

use crate::app::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "composer")] // TODO: Give it a better name
#[command(about = "Composer for PHP in Rust", long_about = None)] // TODO: Give it a better description
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}
