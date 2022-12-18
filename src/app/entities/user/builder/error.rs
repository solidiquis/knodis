use sqlx::Error as SqlxError;
use std::error::Error;
use std::fmt::{self, Display, Debug};
use sea_query::error::Error as SeaQueryError;

#[derive(Debug)]
pub enum UserBuilderError {
    InvalidEmail,
    SeaQueryError(SeaQueryError),
    SqlxError(SqlxError),
    Rollback
}

impl Display for UserBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserBuilderError::InvalidEmail => write!(f, "Invalid format for provided email."),
            UserBuilderError::SeaQueryError(msg) => write!(f, "{}", msg.to_string()),
            UserBuilderError::SqlxError(msg) => write!(f, "{}", msg.to_string()),
            UserBuilderError::Rollback => write!(f, "Transaction rollback.")
        }
    }
}

impl Error for UserBuilderError {}

impl From<SqlxError> for UserBuilderError {
    fn from(e: SqlxError) -> Self {
        UserBuilderError::SqlxError(e)
    }
}

impl Into<UserBuilderError> for SeaQueryError {
    fn into(self) -> UserBuilderError {
        UserBuilderError::SeaQueryError(self)
    }
}
