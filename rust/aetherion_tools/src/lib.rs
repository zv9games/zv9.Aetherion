// aetherion_tools/src/lib.rs
//! Core utilities for configuration, asset management, and data validation.

use once_cell::sync::Lazy;
use regex::Regex;
use tracing::info;

// --- CRATE DEPENDENCIES ---
// NOTE: Assuming AetherionData is defined in aetherion_shared/src/lib.rs
use aetherion_shared::AetherionData; 

// --- CONFIGURATION CONSTANTS ---
// Default settings used if no external config file is loaded.
const DEFAULT_GENERATOR: &str = "cellular_automata_basic";
const DEFAULT_CA_RULESET: u8 = 0; // 0: Basic Cave (B45/S1234567)

// -----------------------------------------------------------------------------
// AETHERION CONFIGURATION UTILITIES
// -----------------------------------------------------------------------------

/// 🔧 Stores all global, static configuration settings for the Aetherion Engine.
pub struct AetherionConfig {
    default_generator_id: String,
    ca_default_ruleset: u8,
}

impl AetherionConfig {
    /// Private function to load/initialize the config with default or file values.
    fn load() -> Self {
        info!("AetherionConfig: Loading default configuration settings...");
        
        AetherionConfig {
            default_generator_id: DEFAULT_GENERATOR.to_string(),
            ca_default_ruleset: DEFAULT_CA_RULESET,
        }
    }

    /// Accessor for the default generator ID.
    pub fn get_default_generator_id(&self) -> &str {
        &self.default_generator_id
    }

    /// Accessor for the Cellular Automata default ruleset ID.
    pub fn get_ca_default_ruleset(&self) -> u8 {
        self.ca_default_ruleset
    }
}

/// Provides thread-safe, static access to the global configuration instance.
static CONFIG: Lazy<AetherionConfig> = Lazy::new(AetherionConfig::load);

/// Public function to retrieve a reference to the global configuration.
pub fn get_config() -> &'static AetherionConfig {
    &CONFIG
}

// -----------------------------------------------------------------------------
// DATA VALIDATION UTILITIES
// -----------------------------------------------------------------------------

/// Provides a thread-safe, lazily initialized Regex instance for data ID validation.
static ID_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Requires IDs to be numeric strings (e.g., for compatibility with file names or database keys).
    Regex::new(r"^\d+$").expect("Failed to compile ID validation regex")
});

/// Validates the ID field of an AetherionData primitive against a standard regex pattern.
/// 
/// This is a crucial **data integrity check** before processing or caching data.
pub fn validate_data_id(data: &AetherionData) -> bool {
    // Assumes AetherionData::id is accessible and implements ToString (e.g., u64).
    ID_REGEX.is_match(&data.id.to_string())
}


// -----------------------------------------------------------------------------
// CRATE ENTRY
// -----------------------------------------------------------------------------

/// Initializes the `aetherion_tools` crate.
pub fn initialize() {
    // Force initialization of the configuration and the static regex on crate load.
    let _ = get_config(); 
    let _ = &*ID_REGEX; // Access the Lazy static to ensure compilation
    info!("Aetherion Tools: Configuration and data validation utilities initialized.");
}