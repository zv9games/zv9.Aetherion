//! Engine configuration structs and logic

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub tick_rate: u32,
    pub max_threads: usize,
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
