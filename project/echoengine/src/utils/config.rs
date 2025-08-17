// config.rs

//! 🧩 Config Module
//! Defines tunable parameters for EchoEngine.
//! Used by audit, overlay, lifecycle, and dimension modules.

use std::env;

/// Engine configuration
#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub tick_rate: u64,         // Ticks per second
    pub debug_overlay: bool,    // Enable visual introspection
    pub dimension_mode: String, // "2D", "3D", or "abstract"
    pub audit_level: AuditLevel,
}

/// Audit verbosity levels
#[derive(Clone, Debug)]
pub enum AuditLevel {
    Silent,
    Minimal,
    Verbose,
    Ritual,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            tick_rate: 60,
            debug_overlay: true,
            dimension_mode: "2D".to_string(),
            audit_level: AuditLevel::Minimal,
        }
    }
}

/// Load config from environment or fallback to default
pub fn load_config() -> EngineConfig {
    EngineConfig {
        tick_rate: env::var("ECHO_TICK_RATE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60),
        debug_overlay: env::var("ECHO_DEBUG_OVERLAY")
            .map(|v| v == "true")
            .unwrap_or(true),
        dimension_mode: env::var("ECHO_DIMENSION_MODE")
            .unwrap_or_else(|_| "2D".to_string()),
        audit_level: match env::var("ECHO_AUDIT_LEVEL").as_deref() {
            Ok("silent") => AuditLevel::Silent,
            Ok("verbose") => AuditLevel::Verbose,
            Ok("ritual") => AuditLevel::Ritual,
            _ => AuditLevel::Minimal,
        },
    }
}
