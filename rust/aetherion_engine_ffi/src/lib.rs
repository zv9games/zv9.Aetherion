// aetherion_engine_ffi/src/lib.rs
//! Low-level C FFI Bridge for Godot Communication.
//!
//! This module exposes functions to C/Godot that manage the Aetherion runtime lifecycle.

use std::ffi::{CStr, CString};
// NEW IMPORT: Use the actual Phase 2 structural test entry point
use aetherion_generate::start_runtime_placeholder; 
use tracing::info; 
// Removed: use aetherion_generate::run_ai_generation; // No longer exists

// --- 1. Core Lifecycle Functions (Phase 2 Alignment) ---

/// Checks if the engine's asynchronous core is structurally ready.
/// This acts as a simple placeholder for Phase 2 readiness.
#[no_mangle]
pub extern "C" fn aetherion_is_runtime_ready() -> bool {
    // Phase 2: Structural check passes now that Phase 1 validation is complete.
    true
}

/// Initializes the engine (Phase 1 shared data setup).
/// Uses the new structural check instead of the deleted `is_initialized`.
#[no_mangle]
pub extern "C" fn aetherion_initialize_engine() -> bool {
    aetherion_shared::initialize_shared_data();
    aetherion_is_runtime_ready()
}

/// Triggers the structural test of the Conductor: initializes the Tokio runtime
/// and immediately shuts it down gracefully.
///
/// This is the FFI-exposed binding for CLI Menu [4].
#[no_mangle]
pub extern "C" fn aetherion_trigger_runtime_test() {
    info!("FFI Bridge: Received command to trigger Conductor structural test.");
    start_runtime_placeholder();
    info!("FFI Bridge: Conductor test sequence complete.");
}


// --- 2. Utility Functions ---

#[no_mangle]
pub extern "C" fn aetherion_get_status(id: u32) -> *mut std::os::raw::c_char {
    let status = format!(
        "Engine status for id {}: Phase 2 Orchestration Ready (Runtime Ready: {})",
        id,
        aetherion_is_runtime_ready()
    );
    // Convert to CString for C compatibility
    match CString::new(status) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => CString::new("Error: Invalid Status String").unwrap().into_raw(),
    }
}

/// FFI function to free strings allocated by Rust.
#[no_mangle]
pub extern "C" fn aetherion_free_string(s: *mut std::os::raw::c_char) {
    unsafe {
        if s.is_null() { return }
        // Retake ownership of the CString to let it be dropped and free the memory.
        let _ = CString::from_raw(s);
    }
}