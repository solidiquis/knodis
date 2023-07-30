use std::{convert::TryFrom, io, thread};

/// Runtime configurations
#[derive(Debug)]
pub struct RuntimeConfig {
    max_blocking_threads: usize,
    worker_threads: usize,
}

impl RuntimeConfig {
    /// Default runtime configurations
    pub fn new() -> io::Result<Self> {
        let worker_threads = thread::available_parallelism()?.get();

        Ok(Self {
            max_blocking_threads: 512,
            worker_threads,
        })
    }
}

impl TryFrom<RuntimeConfig> for tokio::runtime::Runtime {
    type Error = io::Error;

    fn try_from(config: RuntimeConfig) -> Result<Self, Self::Error> {
        tracing::info!("Initializing runtime: {config:?}");

        let RuntimeConfig {
            worker_threads,
            max_blocking_threads,
        } = config;

        tokio::runtime::Builder::new_multi_thread()
            .max_blocking_threads(max_blocking_threads)
            .worker_threads(worker_threads)
            .enable_time()
            .enable_io()
            .build()
    }
}
