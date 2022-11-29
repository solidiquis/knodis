use axum::Extension;
use crate::internal::db::pg::Pg;
use dotenv;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;

mod entities;
mod handlers;
mod routes;

const DEFAULT_IP_ADDR: &'static str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;

pub struct AppState {
    pub pg_pool: Pg
}

impl AppState {
    pub async fn new() -> Self {
        let pg_pool = Pg::new().await;

        Self { pg_pool }
    }
}

pub type SharedState = Extension<Arc<AppState>>;

pub async fn run() {
    let shared_state = Arc::new(AppState::new().await);

    let ip = dotenv::var("IP_ADDR").map_or_else(
        |_e| Ipv4Addr::from_str(DEFAULT_IP_ADDR).unwrap(),
        |ip| Ipv4Addr::from_str(&ip).unwrap()
    );

    let port = dotenv::var("PORT").map_or_else(
        |_e| DEFAULT_PORT,
        |p| p.parse::<u16>().unwrap()
    );

    let socket_addr = SocketAddr::from(SocketAddrV4::new(ip, port));

    let app = routes::compile()
        .layer(Extension(shared_state));

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start Knodis server")
}
