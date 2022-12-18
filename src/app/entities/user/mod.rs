use sea_query::enum_def;
use sqlx::types::chrono::{DateTime, Utc};

/// Various methods to authenticate an existing user.
pub mod authentication;

/// Utility to construct a new "user" entry in SQL.
pub mod builder;

/// Corresponds to "user" SQL table.
#[derive(Debug, sqlx::FromRow)]
#[enum_def]
pub struct User {
    pub id: i32,    
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
