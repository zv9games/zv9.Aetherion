// helpers.rs

//! 🛠️ Helpers Module
//! Contains utility functions and ergonomic shortcuts for EchoEngine.
//! Used across audit, overlay, lifecycle, and signal modules.

use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in milliseconds
pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Pad a string to fixed width (for overlay alignment)
pub fn pad(s: &str, width: usize) -> String {
    let len = s.len();
    if len >= width {
        s.to_string()
    } else {
        format!("{:width$}", s, width = width)
    }
}

/// Convert bool to emoji (for overlay toggles)
pub fn bool_emoji(flag: bool) -> &'static str {
    if flag { "✅" } else { "❌" }
}
