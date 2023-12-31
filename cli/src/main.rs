use crate::app::handlers::require::require_handler;
use crate::app::{commands::Commands, App};
use anyhow::Result;
use clap::Parser;
use composer::config::Config;

mod app;

fn main() -> Result<()> {
    let args = App::parse();
    let config = Config::build()?;

    let _ = match args.command {
        Commands::Require(args) => require_handler(args),
        _ => todo!(),
    };

    Ok(())
}
