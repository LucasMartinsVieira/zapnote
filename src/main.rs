use crate::cli::{Cli, SubCommand};
use crate::note::*;
use clap::Parser;

mod cli;
mod config;
mod note;

fn main() {
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCommand::Note { template, name } => {
            handle_note_command(template, name);
        }
        SubCommand::Journal => {
            todo!()
        }
    }
    println!("Working...");
}
