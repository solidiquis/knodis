use clap::Parser;
use dotenv::dotenv;
use std::error::Error;

/// Application business logic.
mod app;

/// CLI to run app or utility programs.
mod cli;

/// Internal library code.
mod internal;

/// App metadata
mod metadata;

fn main() -> Result<(), Box<dyn Error>> {
    use cli::KnodisCommand;
    use cli::programs::pg::{
        bootstrap::bootstrap,
        migrate::migrate,
        Pg, PgCommand,
    };

    dotenv().ok();

    let cli = cli::Knodis::parse();

    match cli.command {
        None => cli::programs::server::run(),
        Some(cmd) => match cmd {
            KnodisCommand::Run => cli::programs::server::run(),
            KnodisCommand::Routes => cli::programs::routes::debug(),
            KnodisCommand::Pg(Pg { command }) => match command {
                PgCommand::Bootstrap(args) => Ok(bootstrap(args)?),
                PgCommand::Migrate(args) => Ok(migrate(args)?),
            }
        }
    }
}
