use super::error::Error;
use crate::cli::server::env::Environment;
use sqlx::{pool::PoolOptions, Postgres};
use std::{convert::TryFrom, thread, time::Duration};

/// Postgres pool configuration options
#[derive(Debug)]
pub struct Config {
    /// Maximum amount of time waiting for a connection.
    acquire_timeout: Duration,

    /// How long any given connection is allowed to live before it is replaced. Avoids the
    /// memory/resource leaks associated with long-lasting connections.
    max_lifetime: Duration,

    /// How long a connection is allowed to be idle before it is cleaned up.
    idle_timeout: Duration,

    /// The maximum amount of connections the pool is allowed to maintain.
    max_connections: u32,

    /// How many connections to maintain at all times per CPU. If a connection is removed due to
    /// `idle_timeout` or `max_lifetime` then a new one will almost immediately replace it.
    min_connections_per_cpu: u32,
}

impl Config {
    /// Attempts to initialize pool options from [`super::TOML_NAME`]. If `env` is the
    /// `Development` variant, then this will attempt to load the [development.pool_options]
    /// sub-table in the toml file.
    pub fn try_init(conf: &config::Config, env: &Environment) -> Result<Self, Error> {
        let mut pool_options = conf
            .cache
            .clone()
            .into_table()?
            .remove("development")
            .ok_or_else(|| Error::MissingTomlTable(format!("{env}.pool_options")))
            .unwrap()
            .into_table()?
            .remove("pool_options")
            .ok_or_else(|| Error::MissingTomlTable(format!("{env}.pool_options")))
            .unwrap()
            .into_table()?;

        macro_rules! extract_or_err {
            ($e:expr) => {
                pool_options
                    .remove($e)
                    .ok_or_else(|| Error::MissingRequiredField($e))?
            };
        }

        let acquire_timeout = extract_or_err!("acquire_timeout")
            .into_uint()
            .map(Duration::from_secs)?;

        let max_lifetime = extract_or_err!("max_lifetime")
            .into_uint()
            .map(Duration::from_secs)?;

        let idle_timeout = extract_or_err!("idle_timeout")
            .into_uint()
            .map(Duration::from_secs)?;

        let max_connections = u32::try_from(extract_or_err!("max_connections").into_uint()?)
            .expect("Invalid 'max_connections' provided");

        let min_connections_per_cpu =
            u32::try_from(extract_or_err!("min_connections_per_cpu").into_uint()?)
                .expect("Invalid 'min_connections_per_cpu' provided");

        Ok(Self {
            acquire_timeout,
            max_lifetime,
            idle_timeout,
            max_connections,
            min_connections_per_cpu,
        })
    }

    /// Returns the minimum amount of connections to maintain at all times as a function of the
    /// number of logical CPU cores.
    fn min_connections(&self) -> Result<u32, Error> {
        let num_cpus = u32::try_from(thread::available_parallelism()?.get())?;
        Ok(num_cpus * self.min_connections_per_cpu)
    }
}

impl TryFrom<Config> for PoolOptions<Postgres> {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let options = PoolOptions::new()
            .acquire_timeout(config.acquire_timeout)
            .min_connections(config.min_connections()?)
            .max_connections(config.max_connections)
            .idle_timeout(config.idle_timeout)
            .max_lifetime(config.max_lifetime);

        Ok(options)
    }
}
