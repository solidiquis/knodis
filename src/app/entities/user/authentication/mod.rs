use crate::internal::db::pg::{query_builder, funcs};
use sea_query::{expr::Expr, func::Func};
use sqlx::{Executor, Postgres};
use super::{User, UserIden};

#[cfg(test)]
mod test;

/// Authenticates a user via password, using stored Postgres procedure. See `crate::internal::db::pg::funcs::AuthenticationViaPw`.
pub async fn authenticate_via_pw<T>(conn: &mut T, username: &str, plain_text_pw: &str) -> Option<User>
where
    for<'c> &'c mut T: Executor<'c, Database = Postgres>
{
    let query = {
        let statement = query_builder::Query::select()
            .expr(Expr::table_asterisk(UserIden::Table))
            .from(UserIden::Table)
            .cond_where(Expr::col(UserIden::Username).eq(username))
            .and_where(
                Expr::col(UserIden::Password).eq(
                    Func::cust(funcs::Crypt).args([
                        Expr::val(plain_text_pw),
                        Expr::tbl(UserIden::Table, UserIden::Password)
                    ])
                )
            )
            .to_owned();

        let result = query_builder::build(statement);

        result
    };

    sqlx::query_as::<_, User>(&query)
        .fetch_one(conn)
        .await
        .ok()
}
