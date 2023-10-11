mod app;

use crate::app::{commands::Commands, App};
use clap::Parser;

fn main() {
    let args = App::parse();

    match args.command {
        Commands::Require(args) => {
            dbg!(args.packages);
        }
        _ => todo!(),
    }
}
