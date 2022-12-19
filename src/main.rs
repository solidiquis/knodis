use clap::Parser;
use cli::KnodisCommand;
use cli::programs::pg::{
    bootstrap::bootstrap,
    migrate::migrate,
    Pg, PgCommand,
};
use dotenv::dotenv;
use std::str::FromStr;
use tracing_subscriber::{
    filter::LevelFilter,
    Layer,
    layer::SubscriberExt,
};

/// Application business logic.
mod app;

/// CLI to run app or utility programs.
mod cli;

/// Internal library code.
mod internal;

/// App metadata
mod metadata;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // The more layers added for sophisticated diagnostics will eventually warrant moving
    // subscriber stuff into the `internal` module.
    let log_level = {
        let lvl = dotenv::var("LOG_LEVEL").or::<String>(Ok("debug".to_owned()))?;
        LevelFilter::from_str(&lvl)?
    };
    
    let general_layer = tracing_subscriber::fmt::Layer::default()
        .with_level(true)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_filter(log_level);

    let subscriber = tracing_subscriber::Registry::default()
        .with(general_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

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
