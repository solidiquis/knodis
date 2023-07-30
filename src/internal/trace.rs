use std::convert::Into;
use tracing::Level;
use tracing_subscriber::fmt;

/// The default global subscriber that captures spans/events and emits them to a writer.
pub fn init_default_global_subscriber<L: Into<Level>>(level: L) -> fmt::Subscriber {
    fmt::Subscriber::builder()
        .with_ansi(true)
        .with_level(true)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(level.into())
        .finish()
}
