#[macro_export]
macro_rules! box_fut {
    ($block:block) => ({
        std::boxed::Box::pin(async move { $block })
    })
}
