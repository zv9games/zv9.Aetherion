//! Engine configuration structs and logic.
//! Defines runtime parameters for tick rate, threading, and diagnostics.

use serde::{Deserialize, Serialize};

/// Configuration for the Aetherion engine runtime.
/// Can be loaded from external files or constructed programmatically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Target tick rate (frames per second).
    pub tick_rate: u32,

    /// Maximum number of threads for procedural generation.
    pub max_threads: usize,

    /// Enables verbose logging and diagnostics.
    pub enable_logging: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            tick_rate: 60,
            max_threads: 4,
            enable_logging: true,
        }
    }
}

impl EngineConfig {
    /// Returns the tick duration in seconds.
    pub fn tick_interval(&self) -> f64 {
        1.0 / self.tick_rate as f64
    }

    /// Returns true if multithreading is enabled.
    pub fn is_multithreaded(&self) -> bool {
        self.max_threads > 1
    }
}
