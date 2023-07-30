use crate::{app, cli::server::env::Environment};
use axum::{routing::get, Router};

/// Application router wherewith to register all sub-services
pub async fn init_app_router(env: Environment) -> Router {
    let app_state = app::State::init(env).await;

    Router::new()
        .route("/", get(root))
        .with_state(app_state)
}

async fn root() -> &'static str {
    "Hello, World!"
}
