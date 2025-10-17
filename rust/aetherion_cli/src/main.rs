// The main entry point for the executable.
// Uses the 'tokio' and 'env_logger' dependencies from the workspace.

use aetherion_shared::AetherionData;
use aetherion_engine_ffi::aetherion_initialize_engine;
use aetherion_tools::validate_data_id;

// NEW IMPORTS for guaranteed logging visibility
use env_logger::Builder;
use log::LevelFilter; 

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging from the 'env_logger' dependency
    // FIX: Set a default log level (INFO) to guarantee output if RUST_LOG is not set.
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
    
    log::info!("--- Starting Aetherion Command-Line Interface ---");

    // 1. Initialize the core engine via FFI
    if aetherion_initialize_engine() {
        log::info!("Aetherion Core Engine initialized successfully.");
    } else {
        log::error!("Failed to initialize Aetherion Core Engine.");
        return Err(anyhow::anyhow!("Initialization failure"));
    }

    // 2. Perform a sample operation
    let sample_data = AetherionData {
        id: 101,
        timestamp: chrono::Utc::now(),
        value: 42.0,
    };

    if validate_data_id(&sample_data) {
        log::info!("Sample data validated. Ready for processing.");
        // Placeholder for calling processing functions
    } else {
        log::warn!("Sample data validation failed.");
    }
    
    log::info!("--- Aetherion CLI finished ---");
    Ok(())
}