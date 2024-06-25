use crate::cli::{Cli, UserSubCommand};
use clap::Parser;

mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        UserSubCommand::Note { template } => {
            println!("You're accesing this template: {}", template)
        }
        UserSubCommand::Journal => {
            println!("You're acessing Journal")
        }
    }
    println!("Working...");
}
