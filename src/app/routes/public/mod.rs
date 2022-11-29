use axum::routing::Router;

pub mod user;

pub fn router() -> Router {
    Router::new()
        .nest("/user", user::router())
}
