use clap::{Parser, Subcommand};

/// A Second Brain helper
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Create a regular note
    #[command(alias = "n")]
    Note { template: String, name: String },
    /// Create a journal note
    #[command(alias = "j")]
    Journal,
}

#[derive(Subcommand)]
pub enum Note {
    /// Templalte
    Template,
}
