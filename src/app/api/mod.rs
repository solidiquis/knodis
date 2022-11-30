use axum::routing::{get, Router};
use crate::internal::db::pg::Pg;
use std::sync::Arc;

pub mod entities;

pub async fn routes() -> Router<()> {
    let shared_state = Arc::new(AppState::new().await);

    let router: Router<SharedAppState> = Router::new();

    let service: Router<()> = router
        .nest("/-/entities", entities::router())
        .route("/", get(index))
        .with_state(shared_state);
            
    service
}

pub type AppRouter = Router<SharedAppState>;

pub type SharedAppState = Arc<AppState>;

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
