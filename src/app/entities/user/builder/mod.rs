use crate::internal::db::pg::query_builder;
use email_address::EmailAddress;
use sea_query::error::Error as SeaQueryError;
use sqlx::{Error as SqlxError, Executor, Postgres};
use super::{User, UserIden};
use error::UserBuilderError;

pub mod error;

#[cfg(test)]
mod test;

pub struct UserBuilder<'a> {
    username: Option<&'a str>,
    email: Option<&'a str>,
    password: Option<&'a str>, 
}

impl Default for UserBuilder<'_> {
    fn default() -> Self {
        Self { username: None, email: None, password: None }
    }
}

impl<'a> UserBuilder<'a> {
    pub fn new(username: Option<&'a str>, email: Option<&'a str>, password: Option<&'a str>) -> Self {
        Self { username, email, password }
    }

    pub fn username(mut self, username: &'a str) -> Self {
        self.username = Some(username);
        self
    }

    pub fn email(mut self, email: &'a str) -> Result<Self, UserBuilderError> {
        if EmailAddress::is_valid(email) {
            self.email = Some(email);
            return Ok(self);
        }

        Err(UserBuilderError::InvalidEmail)
    }

    pub fn password(mut self, password: &'a str) -> Self {
        self.password = Some(password);
        self
    }

    pub async fn build<T>(self, conn: &mut T) -> Result<User, UserBuilderError>
    where
        for<'c> &'c mut T: Executor<'c, Database = Postgres>
    {
        let query = {
            let statement = query_builder::Query::insert()
                .into_table(UserIden::Table)
                .columns([UserIden::Username, UserIden::Password, UserIden::Email])
                .values([
                    self.username.into(),
                    self.password.into(),
                    self.email.into()
                ])
                .map_err(|e| <SeaQueryError as Into<UserBuilderError>>::into(e))?
                .returning_all()
                .to_owned();

            query_builder::build(statement)
        };

        sqlx::query_as::<_, User>(&query)
            .fetch_one(conn)
            .await
            .map_err(|e| <SqlxError as Into<UserBuilderError>>::into(e))
    }
}
