use crate::cli::{parse_cli, ListTarget, SubCommand};
use crate::journal::*;
use crate::note::*;
use clap_complete::aot::generate;
use config::Config;
use std::{env, io, process};
use utils::casing::convert_case;
use utils::template::{journal_entries, template_entries};

mod cli;
mod config;
mod errors;
mod journal;
mod note;
mod utils;

fn main() {
    let exit_code = match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    };

    process::exit(exit_code);
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = match parse_cli() {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };

    if let Some(config_path) = &cli.config {
        if !config_path.exists() {
            return Err(format!("config file not found: {}", config_path.display()).into());
        }
        env::set_var("ZAPNOTE_CONFIG_PATH", config_path);
    } else {
        Config::load();
    }

    if cli.no_editor {
        env::set_var("ZAPNOTE_NO_EDITOR", String::from("true"));
    } else {
        env::set_var("ZAPNOTE_NO_EDITOR", String::from("false"));
    }

    match &cli.subcommand {
        SubCommand::Note(args) => {
            let note_name = args.name.join(" ");
            let case_converted_title = convert_case(note_name);

            let path = handle_note_command(&args.template, case_converted_title)?;
            if cli.no_editor {
                println!("{path}");
            }
        }
        SubCommand::Journal(args) => {
            let offset = args.offset_value();
            let path = handle_journal_command(&args.name, args.date.as_deref(), offset.as_deref())?;
            if cli.no_editor {
                println!("{path}");
            }
        }
        SubCommand::Completion(args) => {
            let mut cmd = cli::build_cli();
            let command_name = cmd.get_name().to_string();
            generate(args.shell, &mut cmd, command_name, &mut io::stdout());
        }
        SubCommand::List(args) => match &args.target {
            ListTarget::Templates(output) => {
                let templates = template_entries()?;
                if output.json {
                    println!("{}", serde_json::to_string_pretty(&templates)?);
                } else {
                    templates
                        .iter()
                        .for_each(|entry| println!("{}", entry.name));
                }
            }
            ListTarget::Journals(output) => {
                let journals = journal_entries()?;
                if output.json {
                    println!("{}", serde_json::to_string_pretty(&journals)?);
                } else {
                    journals.iter().for_each(|entry| println!("{}", entry.name));
                }
            }
        },
    }

    Ok(())
}
