use crate::{
    cli::server::env::Environment,
    internal::pg::PgPool,
};
use std::sync::Arc;

/// Public and/or private services exposed to the web
pub mod www;

/// Application state and dependency injection mechanism for handlers across entire app.
pub struct State {
    pg_pool: PgPool,
}

impl State {
    /// Initializes atomic reference counter around [`State`]
    pub async fn init(env: Environment) -> Arc<Self> {
        let pg_pool = PgPool::try_init(env).await
            .expect("Failed to establish a Postgres connection");

        Arc::new(Self { pg_pool })
    }
}
