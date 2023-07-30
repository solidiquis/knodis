use super::error::Error;
use crate::cli::server::env::Environment;
use config::Config;
use std::{fmt, ops::Deref};

/// Represents the Postgres connection URI.
pub struct Uri(String);

impl Uri {
    /// Attempts to initialize with provided [`Config`] and [`Environment`].
    pub fn try_init(conf: &Config, env: &Environment) -> Result<Self, Error> {
        let table_name = env.as_ref();

        let table = conf
            .cache
            .clone()
            .into_table()?
            .remove(table_name)
            .ok_or_else(|| Error::MissingTomlTable(format!("{env}")))
            .unwrap();

        let mut config = table.into_table()?;

        macro_rules! extract_or_err {
            ($e:expr) => {
                config
                    .remove($e)
                    .ok_or_else(|| Error::MissingRequiredField($e))?
            };
        }

        let host = extract_or_err!("host").into_string()?;
        let port = extract_or_err!("port").into_uint()?;
        let dbname = extract_or_err!("dbname").into_string()?;
        let user = extract_or_err!("user").into_string()?;
        let password = extract_or_err!("password").into_string()?;
        let sslmode = extract_or_err!("sslmode").into_string()?;

        let uri =
            format!("postgresql://{user}:{password}@{host}:{port}/{dbname}?sslmode={sslmode}");

        Ok(Self(uri))
    }
}

impl Deref for Uri {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
