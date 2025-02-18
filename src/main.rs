use crate::cli::{Cli, SubCommand};
use crate::journal::*;
use crate::note::*;
use clap::Parser;
use config::Config;
use std::env;
use utils::casing::convert_case;

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
            let note_name = args.name.join(" ");
            let case_converted_title = convert_case(note_name);

            env::set_var("ZAPNOTE_NOTE_TITLE", &case_converted_title);

            handle_note_command(&args.template, case_converted_title);
        }
        SubCommand::Journal { name } => {
            handle_journal_commmand(name);
        }
    }

    Ok(())
}
