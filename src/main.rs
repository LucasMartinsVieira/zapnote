use crate::cli::{Cli, SubCommand};
use crate::journal::*;
use crate::note::*;
use clap::Parser;
use config::Config;
use std::env;

mod cli;
mod config;
mod errors;
mod journal;
mod note;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    Config::load();

    if cli.no_editor {
        env::set_var("ZAPNOTE_NO_EDITOR", String::from("true"));
    } else {
        env::set_var("ZAPNOTE_NO_EDITOR", String::from("false"));
    }

    match &cli.subcommand {
        // SubCommand::Note { template, name } => {
        SubCommand::Note(args) => {
            // TODO: Substitute this
            env::set_var("ZAPNOTE_NOTE_TITLE", args.name.join(" "));
            handle_note_command(&args.template, &args.name);
        }
        SubCommand::Journal { name } => {
            handle_journal_commmand(name);
        }
    }

    Ok(())
}
