//! Logging utilities for Aetherion.
//! Provides structured logging for diagnostics, debugging, and runtime feedback.

use log::{info, warn, error, debug};

/// Initializes the logging system using `env_logger`.
/// Should be called once during engine startup.
pub fn init_logging() {
    env_logger::init();
    info!("📝 Logging initialized.");
}

/// Logs a structured debug message with a custom tag.
///
/// # Example
/// ```
/// log_debug("MapBuilder", "Chunk generation started.");
/// ```
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
