use clap::Parser;
use std::{error::Error, process::ExitCode};

/// Application code: Web APIs, models, and so on.
mod app;

/// The interface of this application and all of its auxiliary programs is a CLI. This module
/// contains all its rules and definitions.
mod cli;
use cli::{Cli, Subcommand};

/// Isolated standalone modules that should be completely decoupled from `app`.
mod internal;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.subcommand {
        Subcommand::Server(cmd) => cmd.run()?,
    }

    Ok(())
}
