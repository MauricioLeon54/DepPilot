use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "deppilot",
    about = "Dependency update assistant for JavaScript/TypeScript projects",
    long_about = None,
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Update one or more dependencies and create a commit per update
    Update(UpdateArgs),
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Packages to update (e.g. axios  axios@^1.18.1  vue@latest).
    /// Omit to update all dependencies.
    pub packages: Vec<String>,

    /// Restrict updates to 'deps' (production) or 'dev' (devDependencies)
    #[arg(long, value_name = "TYPE", value_parser = ["deps", "dev"])]
    pub only: Option<String>,

    /// Accept all prompts automatically
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Update dependencies without creating git commits
    #[arg(long)]
    pub no_commit: bool,

    /// Show what would happen without modifying any files
    #[arg(long)]
    pub dry_run: bool,

    /// Shell command to run after each update to validate it
    #[arg(long, value_name = "COMMAND")]
    pub check: Option<String>,

    /// Keep updating even when the validation command fails
    #[arg(long)]
    pub continue_on_error: bool,

    /// Override the auto-detected package manager
    #[arg(long, value_name = "PM", value_parser = ["yarn", "pnpm", "npm"])]
    pub package_manager: Option<String>,

    /// Template for commit messages ({name} and {version} are available as placeholders)
    #[arg(long, value_name = "TEMPLATE", default_value = "chore: update {name}")]
    pub commit_template: String,

    /// Continue even when the package manager exits with a non-zero code (e.g. peer-dep warnings)
    #[arg(long)]
    pub force: bool,
}
