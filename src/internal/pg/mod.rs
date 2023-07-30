use crate::cli::server::env::Environment;
use config::{Config, File};
use sqlx::{
    pool::{Pool, PoolOptions},
    Postgres,
};
use std::{env, ops::Deref};

/// Concerned with generating a Postgres URI connection string.
pub mod conn;

/// Pool configurations
pub mod pool;

/// Errors related to Postgres pool initialization.
pub mod error;
use error::Error;

/// Name of the toml file containing the Postgres connection information expected to be in the
/// project root.
const TOML_NAME: &str = "pg.toml";

/// A wrapper around a [`Pool`] that can be deref coerced into the underlying [`Pool`] while also
/// providing additional behavior.
pub struct PgPool(Pool<Postgres>);

impl PgPool {
    /// Initializes [`PgPool`] using the appropriate configurations for the provided `env` provided
    /// the config toml file.
    pub async fn try_init(env: Environment) -> Result<Self, Error> {
        let toml = Self::load_toml()?;

        let uri = conn::Uri::try_init(&toml, &env)?;
        tracing::info!("Postgres URI: {uri}");

        let pool_config = pool::Config::try_init(&toml, &env)?;
        tracing::info!("Postgres pool opts: {pool_config:?}");

        let pool = PoolOptions::try_from(pool_config)?.connect(&uri).await?;

        Ok(Self(pool))
    }

    /// Loads in the file with the name [`TOML_NAME`] expected to be in the project root.
    fn load_toml() -> Result<Config, Error> {
        let path = env::current_dir().map(|p| p.join(TOML_NAME).to_string_lossy().into_owned())?;

        let name = path
            .strip_suffix(".toml")
            .ok_or(Error::File)
            .map(File::with_name)?;

        let conf = Config::builder().add_source(name).build()?;

        Ok(conf)
    }
}

impl Deref for PgPool {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
