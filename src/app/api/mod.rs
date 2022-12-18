use axum::routing::{get, Router};
use crate::internal::db::pg::Pg;
use std::sync::Arc;

/// Services that allow for direct interaction with entities i.e. `app::entities`.
pub mod entities;

/// Top-level router.
pub async fn router() -> Router<()> {
    let shared_state = Arc::new(AppState::new().await);

    let router: Router<SharedAppState> = Router::new();

    let service: Router<()> = router
        .nest("/-/entities", entities::router())
        .route("/", get(index))
        .with_state(shared_state);
            
    service
}

/// Stateful router. All nested and merged routers must be of this type.
pub type AppRouter = Router<SharedAppState>;

/// Shared application state.
pub type SharedAppState = Arc<AppState>;

/// Application state.
#[derive(Clone)]
pub struct AppState {
    pub pg_pool: Pg
}

impl AppState {
    pub async fn new() -> Self {
        let pg_pool = Pg::new().await;

        Self { pg_pool }
    }
}

pub async fn index() -> &'static str {
    "Hello, World!"
}
