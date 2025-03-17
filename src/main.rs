use clap::Parser;
use rust_cli_app_template::{Cli, Outcome};

#[tokio::main]
async fn main() -> Outcome {
    let args = Cli::parse();
    args.run().await
}
