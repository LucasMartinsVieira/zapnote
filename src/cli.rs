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
    #[arg(long, num_args = 1..=2, allow_hyphen_values = true)]
    pub offset: Option<Vec<String>>,
}

impl JournalArgs {
    pub fn offset_value(&self) -> Option<String> {
        self.offset.as_ref().map(|parts| parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unquoted_negative_offset() {
        let cli = Cli::try_parse_from(["zn", "journal", "day", "--offset", "-1", "day"]).unwrap();

        let SubCommand::Journal(args) = cli.subcommand else {
            panic!("expected journal subcommand");
        };

        assert_eq!(args.offset_value().as_deref(), Some("-1 day"));
    }

    #[test]
    fn parses_unquoted_positive_offset() {
        let cli = Cli::try_parse_from(["zn", "journal", "day", "--offset", "+1", "day"]).unwrap();

        let SubCommand::Journal(args) = cli.subcommand else {
            panic!("expected journal subcommand");
        };

        assert_eq!(args.offset_value().as_deref(), Some("+1 day"));
    }

    #[test]
    fn parses_quoted_offset() {
        let cli = Cli::try_parse_from(["zn", "journal", "day", "--offset", "+1 day"]).unwrap();

        let SubCommand::Journal(args) = cli.subcommand else {
            panic!("expected journal subcommand");
        };

        assert_eq!(args.offset_value().as_deref(), Some("+1 day"));
    }
}
