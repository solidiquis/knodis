use dotenv::dotenv;
use std::error::Error;

/// Application business logic.
mod app;

/// Internal library code.
mod internal;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let runtime = internal::runtime::init()?;

    runtime.block_on(async { app::run().await });

    Ok(())
}
