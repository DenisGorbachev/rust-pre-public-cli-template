use crate::Outcome;
use clap::{value_parser, Parser};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
pub struct PrintCommand {
    #[arg(short, long, value_parser = value_parser!(PathBuf))]
    path: PathBuf,
}

impl PrintCommand {
    pub async fn run(self, _stdin: &mut impl Read, _stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        // let Self {
        //     path,
        // } = self;

        // let mut input = String::new();
        // _stdin.read_to_string(&mut input)?;
        // writeln!(_stdout, "{}", input)?;

        // writeln!(_stdout, "{}", "Test stdout")?;

        // writeln!(_stderr, "{}", "Test stderr")?;

        Ok(())
    }
}
