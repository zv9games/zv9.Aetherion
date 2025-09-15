//C:/ZV9/zv9.aetherion/rust/src/util/logging.rs

// ✅ Suggestions for util/logging.rs

// 🔧 Add dynamic log level control:
//     - `fn set_log_level(level: LevelFilter)`
//     - Enables runtime adjustment for debugging or performance tuning

// 🧩 Add structured logging support:
//     - Include optional fields like `component`, `context`, or `trace_id`
//     - Improves observability and integration with external tools

// 🚦 Improve initialization resilience:
//     - Check if logger is already initialized to prevent panics
//     - Useful in multi-module or plugin environments

// 📚 Document logging conventions:
//     - Clarify tag usage and emoji semantics (e.g. ⚠️ for warnings)
//     - Note that `env_logger` respects `RUST_LOG` environment variable

// 🧪 Add tests for log formatting:
//     - Validate output structure and tag rendering
//     - Ensure consistent behavior across log levels

// 🧼 Optional: Add log capture or export:
//     - e.g. `fn capture_logs_to_file(path: &str)`
//     - Useful for diagnostics, CI, or runtime audits

// 🚀 Future: Add integration with Trailkeeper:
//     - Automatically convert logs into `LogEntry` records
//     - Enables unified diagnostics and historical tracking

// 🧠 Consider exposing logging to GDScript:
//     - e.g. `log_info_gd(tag: String, message: String)`
//     - Useful for runtime feedback and editor integration



// Logging utilities for Aetherion.
// Provides structured logging for diagnostics, debugging, and runtime feedback.

use log::{info, warn, error, debug, LevelFilter};
#[allow(unused_imports)]
use crate::zv9_prelude::*;

// Initializes the logging system using `env_logger`.
// Should be called once during engine startup.
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

// Logs a warning with context.
pub fn log_warn(tag: &str, message: &str) {
    warn!("[{}] ⚠️ {}", tag, message);
}

// Logs an error with context.
pub fn log_error(tag: &str, message: &str) {
    error!("[{}] ❌ {}", tag, message);
}

// Logs an info-level message with context.
pub fn log_info(tag: &str, message: &str) {
    info!("[{}] {}", tag, message);
}
//end logging.rs