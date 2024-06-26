use crate::cli::{Cli, SubCommand};
use crate::note::*;
use clap::Parser;
use config::Config;

mod cli;
mod config;
mod note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCommand::Note { template, name } => {
            handle_note_command(template, name);
        }
        SubCommand::Journal => {
            todo!()
        }
    }

    Config::load_config();

    Ok(())
}
