pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = crate::internal::runtime::init()?;
    runtime.block_on(async { crate::app::run().await });
    Ok(())
}
