use clap::Parser;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

use cli::{Cli, Commands};
use list::list_templates;

mod cli;
mod list;
mod template;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .compact()
        .without_time()
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting the default subscriber failed for logging");

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List {}) => list_templates(),
        Some(Commands::Init { template }) => {
            println!("Initializing template: {:?}", template)
        }
        None => error!("no command given"),
    }
}
