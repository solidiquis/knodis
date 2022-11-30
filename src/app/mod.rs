use axum::Extension;
use crate::internal::db::pg::Pg;
use dotenv;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;

mod entities;
mod api;

const DEFAULT_IP_ADDR: &'static str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;

pub async fn run() {
    let ip = dotenv::var("IP_ADDR").map_or_else(
        |_e| Ipv4Addr::from_str(DEFAULT_IP_ADDR).unwrap(),
        |ip| Ipv4Addr::from_str(&ip).unwrap()
    );

    let port = dotenv::var("PORT").map_or_else(
        |_e| DEFAULT_PORT,
        |p| p.parse::<u16>().unwrap()
    );

    let socket_addr = SocketAddr::from(SocketAddrV4::new(ip, port));

    let app = api::routes().await;

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start Knodis server")
}
