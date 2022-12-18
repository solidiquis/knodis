use futures::future::BoxFuture;
use num_cpus;
use sqlx::{
    Connection,
    Error as SqlxError,
    pool::PoolConnection,
    Pool,
    Postgres,
    postgres::PgPoolOptions,
    Transaction
};
use std::time::Duration;
use crate::box_fut;

pub mod funcs;
pub mod query_builder;

#[cfg(not(test))]
const MAX_CONNECTIONS: u32 = 100;

#[cfg(test)]
const MAX_CONNECTIONS: u32 = 2;

const MIN_CONNS_PER_WORKER: u32 = 5;
const ACQUIRE_CONN_TIMEOUT: Duration = Duration::from_secs(5);
const IDLE_TIMEOUT: Duration = Duration::from_secs(10 * 60);
const MAX_LIFETIME: Duration = Duration::from_secs(30 * 60);

#[cfg(test)]
const DB_URL_ENV_VAR: &'static str = "DATABASE_URL_TEST";

#[cfg(not(test))]
const DB_URL_ENV_VAR: &'static str = "DATABASE_URL";

pub type PgPool = Pool<Postgres>;
pub type PgConn = PoolConnection<Postgres>;

#[derive(Clone)]
pub struct Pg {
    pool: Pool<Postgres>
}

impl Pg {
    pub async fn new() -> Self {
        let url = dotenv::var(DB_URL_ENV_VAR).expect("Missing 'DB_URL_ENV_VAR'");

        let pool = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .min_connections(Self::min_connections())
            .acquire_timeout(ACQUIRE_CONN_TIMEOUT)
            .idle_timeout(Some(IDLE_TIMEOUT))
            .max_lifetime(Some(MAX_LIFETIME))
            .test_before_acquire(true)
            .connect(&url)
            .await
            .expect(&format!("Failed to connect to {url}"));

        Self { pool }
    }

    pub async fn acquire(&self) -> Result<PgConn, SqlxError> {
        self.pool.acquire().await
    }

    pub fn inner_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub async fn transaction<F, T, E>(&self, op: F) -> Result<T, E>
    where
        T: Send,
        E: From<SqlxError> + Send,
        for<'c> F: FnOnce(&'c mut Transaction<'_, Postgres>) -> BoxFuture<'c, Result<T, E>> + Send + Sync,
    {
        let mut conn = self.acquire().await?;
        conn.transaction(op).await
    }

    pub async fn with_conn<F, T, E>(&self, op: F) -> Result<T, E>
    where
        T: Send, 
        E: From<SqlxError> + Send,
        for<'c >F: FnOnce(&'c mut PgConn) -> BoxFuture<'c, Result<T, E>> + Send + Sync
    {
        let mut conn = self.acquire().await?;
        op(&mut conn).await
    }

    #[cfg(not(test))]
    fn min_connections() -> u32 {
        let logical_cpus = num_cpus::get() as u32;
        logical_cpus * MIN_CONNS_PER_WORKER
    }

    #[cfg(test)]
    fn min_connections() -> u32 {
        1
    }
}
