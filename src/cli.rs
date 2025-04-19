use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Increase the log verbosity
    #[arg(short, long)]
    verbose: bool,

    /// Supress all non-error outputs
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show locally‑cached templates
    List {},
    /// Scaffold a project from a template
    Init { template: String },
}
