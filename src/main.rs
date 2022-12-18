use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::error::Error;

/// Application business logic.
mod app;

/// Internal library code.
mod internal;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let cli = Knodis::parse();

    match cli.command {
        None => commands::server::run(),
        Some(cmd) => match cmd {
            Command::Run => commands::server::run(),
            Command::Routes => commands::routes::debug()
        }
    }
}

#[derive(Parser)]
#[command(name = "Knodis")]
#[command(about = "Collection of Knodis utilities.", long_about = None)]
struct Knodis {
    #[command(subcommand)]
    command: Option<Command> 
}

#[derive(Subcommand)]
enum Command {
    /// Display all routes.
    Routes,

    /// Runs the app server.
    Run
}

/// CLI subcommands.
pub mod commands {
    /// Run the application server.
    pub mod server {
        pub fn run() -> Result<(), Box<dyn std::error::Error>> {
            let runtime = crate::internal::runtime::init()?;
            runtime.block_on(async { crate::app::run().await });
            Ok(())
        }
    }

    /// Debug-print the application router.
    pub mod routes {
        pub fn debug() -> Result<(), Box<dyn std::error::Error>> {
            let runtime = crate::internal::runtime::init()?;
            let router = runtime.block_on(async { crate::app::api::router().await });
            println!("{:?}", router);
            Ok(())
        }
    }
}

