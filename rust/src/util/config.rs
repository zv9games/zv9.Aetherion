//C:/ZV9/zv9.aetherion/rust/src/util/config.rs

/// ✅ Suggestions for util/config.rs

// 🔧 Add file loading and saving utilities:
//     - `fn load_from(path: &str) -> Result<Self, ConfigError>`
//     - `fn save_to(&self, path: &str) -> Result<(), ConfigError>`
//     - Enables persistence and external configuration

// 🧩 Add validation logic:
//     - `fn validate(&self) -> Result<(), String>`
//     - Ensures tick rate and thread count are within safe bounds

// 🚦 Improve diagnostics integration:
//     - Add `diagnostics_level: Option<String>` or enum
//     - Enables fine-grained control over logging verbosity

// 📚 Document runtime impact:
//     - Clarify how `tick_rate` affects simulation pacing
//     - Note that `max_threads` influences procedural workload distribution

// 🧪 Add unit tests for `tick_interval` and `is_multithreaded`:
//     - Validate correctness across edge cases and default config

// 🧼 Optional: Add feature flags or toggles:
//     - e.g. `enable_profiling`, `use_gpu_acceleration`
//     - Useful for runtime tuning and platform-specific behavior

// 🚀 Future: Add config hot-reloading support:
//     - e.g. `fn watch(path: &str)` with callback on change
//     - Enables dynamic reconfiguration without restart

// 🧠 Consider exposing config to GDScript:
//     - Wrap in a Godot-friendly resource or singleton
//     - Useful for editor tweaking and runtime inspection



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
//end config.rs