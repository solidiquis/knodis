use axum::routing::{get, Router};
use super::super::AppRouter;

pub fn router() -> AppRouter {
    Router::new()
        .route("/", get(index))
}

pub async fn index() -> &'static str {
    "Hello, World!"
}
