use crate::Outcome;
use clap::Parser;
use Command::*;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    Print(PrintCommand),
}

impl Command {
    pub async fn run(self) -> Outcome {
        match self {
            Print(command) => command.run().await,
        }
    }
}

mod print_command;

pub use print_command::*;
