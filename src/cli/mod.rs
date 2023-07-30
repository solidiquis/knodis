use clap::Parser;
use std::convert::From;

/// Rules about the `server` subcommand.
pub mod server;

/// Defines the CLI.
#[derive(Parser, Debug)]
#[command(name = "knodis")]
#[command(author = "Benjamin Nguyen. <benjamin.van.nguyen@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Interface to Knodis and all of its auxiliary programs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

/// All of the subcommands of the CLI.
#[derive(clap::Subcommand, Debug)]
pub enum Subcommand {
    /// Runs the application server
    Server(server::Command),
}
