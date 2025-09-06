//C:/ZV9/zv9.aetherion/rust/src/util/logging.rs
//! Logging utilities for Aetherion.
//! Provides structured logging for diagnostics, debugging, and runtime feedback.

use log::{info, warn, error, debug, LevelFilter};

/// Initializes the logging system using `env_logger`.
/// Should be called once during engine startup.
use env_logger::Builder;
use std::io::Write;

pub fn init_logging() {
    Builder::new()
        .filter_level(LevelFilter::Info) // You can change this to Debug, Warn, etc.
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {}: {}",
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    log::info!("📝 Logging initialized.");
}

pub fn log_debug(tag: &str, message: &str) {
    debug!("[{}] {}", tag, message);
}

/// Logs a warning with context.
pub fn log_warn(tag: &str, message: &str) {
    warn!("[{}] ⚠️ {}", tag, message);
}

/// Logs an error with context.
pub fn log_error(tag: &str, message: &str) {
    error!("[{}] ❌ {}", tag, message);
}

/// Logs an info-level message with context.
pub fn log_info(tag: &str, message: &str) {
    info!("[{}] {}", tag, message);
}
//end logging.rs