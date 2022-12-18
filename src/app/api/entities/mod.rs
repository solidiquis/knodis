use axum::Router;
use super::AppRouter;

/// Services that allow for direct interaction with `User` entity.
pub mod user;

/// Routes for `User` services.
pub fn router() -> AppRouter {
    Router::new()
        .nest("/user", user::router())
}
