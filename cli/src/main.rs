mod app;

use crate::app::handlers::require::require_handler;
use crate::app::{commands::Commands, App};
use clap::Parser;

fn main() {
    let args = App::parse();

    let _ = match args.command {
        Commands::Require(args) => require_handler(args),
        _ => todo!(),
    };
}
