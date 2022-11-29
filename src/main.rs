use dotenv::dotenv;
use std::error::Error;

mod app;
mod internal;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let runtime = internal::runtime::init()?;

    runtime.block_on(async { app::run().await });

    Ok(())
}
