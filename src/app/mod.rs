use dotenv;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;

/// Models that may or may not be an in-memory representation of an entity on a database.
pub mod entities;

/// Services that are meant to be exposed directly to the web. Top-level sub-modules of `api` each contain
/// their own routers, set of handlers, and middleware, all of which are meant to be merged and/or nested with the base router,
/// `api::router()`. The base router registers the shared application state.
pub mod api;

const DEFAULT_IP_ADDR: &'static str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;

/// Starts the app server.
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

    let app = api::router().await;

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start Knodis server")
}
