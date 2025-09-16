//C:/ZV9/zv9.aetherion/rust/src/util/logging.rs

use log::{info, warn, error, debug, LevelFilter};
use env_logger::Builder;
use std::io::Write;
#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 📝 Initializes the logging system using `env_logger`.
/// Should be called once during engine startup.
pub fn init_logging() {
    Builder::new()
        .filter_level(LevelFilter::Info) // Change to Debug, Warn, etc. as needed
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

    info!("📝 Logging initialized.");
}

/// Logs a debug-level message with a tag.
pub fn log_debug(tag: &str, message: &str) {
    debug!("[{}] {}", tag, message);
}

/// Logs a warning-level message with a tag.
pub fn log_warn(tag: &str, message: &str) {
    warn!("[{}] ⚠️ {}", tag, message);
}

/// Logs an error-level message with a tag.
pub fn log_error(tag: &str, message: &str) {
    error!("[{}] ❌ {}", tag, message);
}

/// Logs an info-level message with a tag.
pub fn log_info(tag: &str, message: &str) {
    info!("[{}] {}", tag, message);
}



#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_log_info_output() {
        init_logging();
        log_info("test", "This is an info message");
    }

    #[test]
    fn stress_log_debug_output() {
        init_logging();
        log_debug("debugger", "Debugging subsystem initialized");
    }

    #[test]
    fn stress_log_warn_output() {
        init_logging();
        log_warn("config", "Missing optional config file");
    }

    #[test]
    fn stress_log_error_output() {
        init_logging();
        log_error("engine", "Failed to initialize engine core");
    }

    #[test]
    fn stress_multiple_log_levels() {
        init_logging();
        log_info("multi", "Info level active");
        log_debug("multi", "Debug level active");
        log_warn("multi", "Warning level active");
        log_error("multi", "Error level active");
    }
}


//end logging.rs