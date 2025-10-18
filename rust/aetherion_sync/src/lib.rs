//! aetherion_sync/src/lib.rs
//!
//! Provides the core synchronization and communication channels for the engine,
//! allowing data exchange between the main thread, Godot, and worker threads.

use crossbeam_channel::{unbounded, Receiver, Sender};
// We use the tracing framework (configured in Cargo.toml) for logging.
use tracing::info;

// NOTE: We are using String as a temporary placeholder for the complex
// AetherionData struct, which will be defined later in the project.

/// Creates an unbounded crossbeam channel pair.
///
/// This channel will eventually be used to send complex Aetherion data structs
/// between the engine threads.
pub fn create_sync_channel() -> (Sender<String>, Receiver<String>) {
    unbounded()
}

/// Starts the main synchronization worker task.
///
/// In a real application, this would launch an asynchronous task, likely using tokio.
pub fn start_sync_worker() {
    info!("Aetherion Synchronization Worker placeholder started.");
}
