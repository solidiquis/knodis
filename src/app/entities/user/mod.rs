use sea_query::enum_def;
use sqlx::types::chrono::{DateTime, Utc};

pub mod authentication;
pub mod builder;

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
