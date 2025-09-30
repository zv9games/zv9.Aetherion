//! 🛠️ Helpers Module
//! Utility functions and ergonomic shortcuts for EchoEngine.
//!
//! Used across audit, overlay, lifecycle, and signal modules.
//! Designed for clarity, alignment, and introspection support.
//!
//! Contributors may invoke these helpers to simplify timestamping, formatting, and overlay logic.

use std::time::{SystemTime, UNIX_EPOCH};

/// 🕰️ Get current timestamp in milliseconds since UNIX epoch
/// Used for audit logs, overlay glyphs, and tick introspection
pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// 🔧 Clamp a value between min and max bounds
/// Used for overlay sizing, tick pacing, and debug glyphs
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// 📐 Pad a string to fixed width for overlay alignment
/// Used in manifest rendering and debug observatory
pub fn pad(s: &str, width: usize) -> String {
    let len = s.len();
    if len >= width {
        s.to_string()
    } else {
        format!("{:width$}", s, width = width)
    }
}

/// ✅❌ Convert boolean flag to emoji glyph
/// Used for toggle overlays, audit trails, and contributor feedback
pub fn bool_emoji(flag: bool) -> &'static str {
    if flag { "✅" } else { "❌" }
}
