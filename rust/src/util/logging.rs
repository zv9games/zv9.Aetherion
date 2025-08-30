//! Logging utilities for Aetherion

use log::{info, warn, error, debug};

pub fn init_logging() {
    env_logger::init();
    info!("Logging initialized.");
}

/// Log a structured debug message
pub fn log_debug(tag: &str, message: &str) {
    debug!("[{}] {}", tag, message);
}
