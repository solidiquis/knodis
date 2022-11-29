use axum::routing::{get, Router};
use super::handlers::index;

pub mod public;

pub fn compile() -> Router {
    Router::new()
        .nest("/-", public::router())
        .route("/", get(index))
}
