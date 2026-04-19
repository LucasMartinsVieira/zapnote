use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// A Second Brain helper
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommand,
    /// Only create the note and not run the editor
    #[arg(short, long, global = true)]
    pub no_editor: bool,
    /// Path to a custom config file
    #[arg(short = 'c', long, global = true, value_name = "PATH")]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Create a regular note
    #[command(alias = "n")]
    Note(NoteArgs),
    //Note { template: String, name: String },
    /// Create a journal note
    #[command(alias = "j")]
    Journal(JournalArgs),
}

#[derive(Args)]
pub struct NoteArgs {
    pub template: String,
    pub name: Vec<String>,
}

#[derive(Args)]
pub struct JournalArgs {
    pub name: String,
    #[arg(long)]
    pub date: Option<String>,
    #[arg(long)]
    pub offset: Option<String>,
}
