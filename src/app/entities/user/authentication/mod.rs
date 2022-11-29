use crate::internal::db::pg::{query_builder, funcs};
use sea_query::{expr::Expr, func::Func, Iden};
use sqlx::{Executor, Postgres};
use super::User;

#[cfg(test)]
mod test;

pub async fn authenticate_via_pw<T>(conn: &mut T, username: &str, plain_text_pw: &str) -> Option<User>
where
    for<'c> &'c mut T: Executor<'c, Database = Postgres>
{
    let query = {
        let statement = query_builder::Query::select()
        .expr(Func::cust(funcs::AuthenticationViaPw).args([Expr::val(username), Expr::val(plain_text_pw)]))
        .to_owned();

        query_builder::build(statement)
    };

    sqlx::query_as::<_, User>(&query)
        .fetch_one(conn)
        .await
        .ok()
}
