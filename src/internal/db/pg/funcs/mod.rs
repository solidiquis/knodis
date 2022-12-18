use sea_query::Iden;

/// Identifier for stored Postgres stored procedure, `authentication_via_pw`.
/// See `migrations/20221121004517_authenticate_via_pw.up.sql`.
#[derive(Iden)]
pub struct AuthenticateViaPw;

/// Identifier for stored Postgres stored procedure, `crypt`.
/// See `pgcrypto` Postgres extension for more info on `crypt`.
#[derive(Iden)]
pub struct Crypt;
