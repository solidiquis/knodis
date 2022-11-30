use axum::Router;
use super::AppRouter;

pub mod user;

pub fn router() -> AppRouter {
    Router::new()
        .nest("/user", user::router())
}
