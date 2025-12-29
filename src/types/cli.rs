use crate::{Command, CommandRunError};
use clap::Parser;
use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    // #[arg(short, long, value_parser = value_parser!(PathBuf))]
    // root: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn run(self) -> Result<(), CliRunError> {
        let Self {
            command,
        } = self;
        command.run().await.map_err(From::from)
    }
}

#[derive(Error, Display, From, Debug)]
pub enum CliRunError {
    CommandRunFailed { source: CommandRunError },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
