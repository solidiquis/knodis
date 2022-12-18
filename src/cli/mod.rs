use clap::{Args, Parser, Subcommand};

/// CLI programs.
pub mod programs;

mod utils;

#[derive(Parser)]
#[command(name = "Knodis")]
pub struct Knodis {
    #[command(subcommand)]
    pub command: Option<KnodisCommand> 
}

#[derive(Debug, Subcommand)]
pub enum KnodisCommand {
    /// Display all routes
    Routes,

    /// Runs the app server
    Run,

    /// Postgres utilities
    Pg(programs::pg::Pg)
}
