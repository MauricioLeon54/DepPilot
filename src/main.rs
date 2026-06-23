mod cli;
mod errors;
mod git;
mod output;
mod package_json;
mod package_manager;
mod project;
mod prompt;
mod update;
mod validation;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Update(args) => update::run(&args),
    }
}
