use clap::Parser;
use rust_cli_app_template::{Cli, Outcome};
use std::io::{stderr, stdin, stdout};

#[tokio::main]
async fn main() -> Outcome {
    let args = Cli::parse();
    let mut stdin = stdin();
    let mut stdout = stdout();
    let mut stderr = stderr();
    args.run(&mut stdin, &mut stdout, &mut stderr).await
}
