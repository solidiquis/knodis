pub fn debug() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = crate::internal::runtime::init()?;
    let router = runtime.block_on(async { crate::app::api::router().await });
    println!("{:?}", router);
    Ok(())
}
