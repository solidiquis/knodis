use super::TOML_NAME;
use config::ConfigError;
use std::{io, num::TryFromIntError};

/// Errors related to Postgres connection pool initialization.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Config(#[from] ConfigError),

    #[error("{0}")]
    Connect(#[from] sqlx::Error),

    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    IntConversion(#[from] TryFromIntError),

    #[error("Failed to find pg.toml in the project root")]
    File,

    #[error("Failed to find table '{0}' in {TOML_NAME}")]
    MissingTomlTable(String),

    #[error("Missing required field '{0}' from {TOML_NAME}")]
    MissingRequiredField(&'static str),
}
