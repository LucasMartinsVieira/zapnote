use std::{env, ffi::OsStr, path::PathBuf};

use clap::{
    builder::PossibleValuesParser, Args, Command, CommandFactory, FromArgMatches, Parser,
    Subcommand,
};
use clap_complete::{
    aot::Shell,
    engine::{ArgValueCompleter, CompletionCandidate},
};

use crate::utils::template::{journal_entries, template_entries};

/// A Second Brain helper
#[derive(Parser)]
#[command(name = "zn", version, about, long_about = None)]
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
    /// Create a journal note
    #[command(alias = "j")]
    Journal(JournalArgs),
    /// Generate shell completion scripts
    Completion(CompletionArgs),
    /// List metadata for templates and journals
    List(ListArgs),
}

#[derive(Args)]
pub struct NoteArgs {
    #[arg(index = 1)]
    pub template: String,
    #[arg(index = 2, required = true, num_args = 1..)]
    pub name: Vec<String>,
}

#[derive(Args)]
pub struct JournalArgs {
    #[arg(index = 1)]
    pub name: String,
    #[arg(long)]
    pub date: Option<String>,
    #[arg(long, num_args = 1..=2, allow_hyphen_values = true)]
    pub offset: Option<Vec<String>>,
}

#[derive(Args)]
pub struct CompletionArgs {
    pub shell: Shell,
}

#[derive(Args)]
pub struct ListArgs {
    #[command(subcommand)]
    pub target: ListTarget,
}

#[derive(Subcommand)]
pub enum ListTarget {
    /// List available note templates
    Templates(ListOutputArgs),
    /// List configured journal definitions
    Journals(ListOutputArgs),
}

#[derive(Args)]
pub struct ListOutputArgs {
    #[arg(long)]
    pub json: bool,
}

impl JournalArgs {
    pub fn offset_value(&self) -> Option<String> {
        self.offset.as_ref().map(|parts| parts.join(" "))
    }
}

pub fn parse_cli() -> Result<Cli, clap::Error> {
    if let Some(config_path) = config_path_from_args() {
        env::set_var("ZAPNOTE_CONFIG_PATH", config_path);
    }

    let matches = build_cli().try_get_matches()?;
    Cli::from_arg_matches(&matches)
}

pub fn build_cli() -> Command {
    let template_candidates = note_template_candidates();
    let journal_candidates = journal_name_candidates();

    let mut command = Cli::command();

    command = command.mut_subcommand("note", |cmd| {
        cmd.mut_arg("template", |arg| {
            let arg = arg.add(ArgValueCompleter::new(complete_note_templates));
            if template_candidates.is_empty() {
                arg
            } else {
                arg.value_parser(PossibleValuesParser::new(leak_candidates(
                    &template_candidates,
                )))
            }
        })
    });

    command.mut_subcommand("journal", |cmd| {
        cmd.mut_arg("name", |arg| {
            let arg = arg.add(ArgValueCompleter::new(complete_journal_names));
            if journal_candidates.is_empty() {
                arg
            } else {
                arg.value_parser(PossibleValuesParser::new(leak_candidates(
                    &journal_candidates,
                )))
            }
        })
    })
}

fn note_template_candidates() -> Vec<String> {
    template_entries()
        .unwrap_or_default()
        .into_iter()
        .map(|entry| entry.name)
        .collect()
}

fn journal_name_candidates() -> Vec<String> {
    journal_entries()
        .unwrap_or_default()
        .into_iter()
        .map(|entry| entry.name)
        .collect()
}

fn filter_candidates(current: &OsStr, candidates: &[String]) -> Vec<CompletionCandidate> {
    let Some(current) = current.to_str() else {
        return vec![];
    };

    candidates
        .iter()
        .filter(|candidate| candidate.starts_with(current))
        .map(|candidate| CompletionCandidate::new(candidate.clone()))
        .collect()
}

fn leak_candidates(candidates: &[String]) -> Vec<&'static str> {
    candidates
        .iter()
        .cloned()
        .map(|candidate| Box::leak(candidate.into_boxed_str()) as &'static str)
        .collect()
}

fn complete_note_templates(current: &OsStr) -> Vec<CompletionCandidate> {
    filter_candidates(current, &note_template_candidates())
}

fn complete_journal_names(current: &OsStr) -> Vec<CompletionCandidate> {
    filter_candidates(current, &journal_name_candidates())
}

fn config_path_from_args() -> Option<PathBuf> {
    let mut args = env::args_os().skip(1).peekable();

    while let Some(arg) = args.next() {
        if arg == "--config" || arg == "-c" {
            return args.next().map(PathBuf::from);
        }

        if let Some(arg_str) = arg.to_str() {
            if let Some(path) = arg_str.strip_prefix("--config=") {
                return Some(PathBuf::from(path));
            }

            if let Some(path) = arg_str.strip_prefix("-c") {
                if !path.is_empty() {
                    return Some(PathBuf::from(path));
                }
            }
        }
    }

    None
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

    #[test]
    fn parses_completion_subcommand() {
        let cli = Cli::try_parse_from(["zn", "completion", "bash"]).unwrap();

        let SubCommand::Completion(args) = cli.subcommand else {
            panic!("expected completion subcommand");
        };

        assert!(matches!(args.shell, Shell::Bash));
    }

    #[test]
    fn parses_list_templates_json() {
        let cli = Cli::try_parse_from(["zn", "list", "templates", "--json"]).unwrap();

        let SubCommand::List(args) = cli.subcommand else {
            panic!("expected list subcommand");
        };

        let ListTarget::Templates(list_args) = args.target else {
            panic!("expected templates target");
        };

        assert!(list_args.json);
    }
}
