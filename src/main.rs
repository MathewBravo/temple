use clap::Parser;

use cli::{Cli, Commands};

mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List {}) => println!("Listing templates..."),
        Some(Commands::Init { template }) => {
            println!("Initializing template: {:?}", template)
        }
        None => println!("Debug: No Command Given!"),
    }
}
