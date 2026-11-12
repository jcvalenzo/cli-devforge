use std::process::ExitCode;

use clap::Parser;
use devforge::cli::{Cli, Command};
use devforge::commands;

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let use_stderr = error.use_stderr();
            let _ = error.print();
            return if use_stderr {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            };
        }
    };

    let result = match cli.command {
        Some(Command::Create(args)) => commands::create::run(args),
        Some(Command::List) => commands::list::run(),
        Some(Command::Version) => commands::version::run(),
        None => commands::home::run(),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}
