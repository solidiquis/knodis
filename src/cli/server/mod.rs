use crate::{
    app::www,
    internal::{runtime::RuntimeConfig, trace},
};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};
use tokio::runtime::Runtime;

/// What environment is the application running in
pub mod env;

/// Log level definitions
pub mod log;

/// Default IP address to listen on.
const DEFAULT_IP_ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

/// Default port number to listen on.
const DEFAULT_PORT: u16 = 6666;

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Which IP address to use
    #[arg(short, long, default_value_t = DEFAULT_IP_ADDR)]
    ipaddr: Ipv4Addr,

    /// Which port to listen on
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,

    /// Log level filter
    #[arg(short, long, value_enum, default_value_t)]
    log_level: log::Level,

    /// Sets the Knodis environment
    #[arg(short, long, value_enum, default_value_t)]
    env: env::Environment,
}

impl Command {
    /// Run the application server. Entry-point for the main application.
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let tracing_sub = trace::init_default_global_subscriber(self.log_level);
        tracing::subscriber::set_global_default(tracing_sub)?;

        let runtime = RuntimeConfig::new().and_then(Runtime::try_from)?;

        runtime.block_on(self.start_application())?;

        Ok(())
    }

    async fn start_application(&self) -> Result<(), Box<dyn Error>> {
        let app_router = www::init_app_router(self.env).await;

        let socket_v4 = SocketAddrV4::new(self.ipaddr, self.port);
        let socket_addr = SocketAddr::V4(socket_v4);

        tracing::info!("Listening on tcp://{socket_addr}");

        axum::Server::bind(&socket_addr)
            .serve(app_router.into_make_service())
            .await?;

        Ok(())
    }
}
