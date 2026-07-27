//! Version metadata.

/// Crate version string (Cargo package version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Human-readable version line for logs and CLI.
pub fn version_string() -> String {
    format!("aetherion {VERSION}")
}
