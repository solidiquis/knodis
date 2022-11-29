use sea_query::{query::QueryStatementWriter, backend::PostgresQueryBuilder};

pub use sea_query::query::Query;

pub fn build<S: QueryStatementWriter>(statement: S) -> String {
    statement.to_string(PostgresQueryBuilder)
}
