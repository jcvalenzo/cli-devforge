use clap::{Args, Parser, Subcommand};

use crate::VERSION;

#[derive(Debug, Parser)]
#[command(
    name = "devforge",
    version = VERSION,
    about = "Project scaffold generator based on naming conventions",
    long_about = "devforge creates project scaffolds based on prefix-based naming conventions.",
    override_usage = "devforge <COMMAND>"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new project scaffold
    #[command(
        long_about = "Create a new project scaffold based on prefix naming convention.

Examples:
  devforge create api-catalog --lang go
  devforge create ms-auth --lang java
  devforge create web-admin --lang node
  devforge create worker-email --lang python"
    )]
    Create(CreateArgs),

    /// List all available prefixes
    List,

    /// Print the version number
    Version,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Project name in <prefix>-<name> form
    pub name: String,

    /// Language for the project (default: prefix default)
    #[arg(short, long, default_value = "")]
    pub lang: String,

    /// Author name
    #[arg(short, long, default_value = "")]
    pub author: String,

    /// Go module path (for Go projects)
    #[arg(long = "module", default_value = "")]
    pub module: String,

    /// Project license
    #[arg(long, default_value = "MIT")]
    pub license: String,

    /// Output directory
    #[arg(short, long, default_value = ".")]
    pub output: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn no_args_means_home() {
        let cli = Cli::try_parse_from(["devforge"]).expect("parse");
        assert!(cli.command.is_none());
    }
}
