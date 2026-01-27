//! Logging setup using tracing
//!
//! Configures file-based logging with level control

use std::path::Path;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize logging to file
pub fn init_logging(log_path: &Path, debug: bool) -> anyhow::Result<()> {
    let log_file = std::fs::File::create(log_path)?;

    let level = if debug { "debug" } else { "info" };

    let file_layer = fmt::layer()
        .with_writer(log_file)
        .with_ansi(false)
        .with_target(false);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .init();

    tracing::info!("Logging initialized to {:?} at level {}", log_path, level);
    Ok(())
}
