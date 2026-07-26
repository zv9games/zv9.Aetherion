//! Aetherion FFI Bridge — The C Gateway to Infinite Realms
//!
//! Low-level C interface for Godot integration.
//! Manages runtime lifecycle and string-safe status reporting.
//! For the hopeless wanderers who speak in C and dream in Rust.

use std::ffi::CString;
use std::sync::OnceLock;

use aetherion_generate::Conductor;
use aetherion_shared::initialize_shared_data;
use tracing::{info, error};

// ── Global Conductor (Thread-Safe Singleton) ───────────────────────────────
static CONDUCTOR: OnceLock<Conductor> = OnceLock::new();

// ── Runtime Lifecycle ──────────────────────────────────────────────────────

/// Starts the Aetherion async engine. Idempotent.
#[no_mangle]
pub extern "C" fn aetherion_start_runtime() -> bool {
    initialize_shared_data();

    if CONDUCTOR.get().is_some() {
        info!("FFI: Runtime already active.");
        return true;
    }

    match Conductor::new(None) {
        Ok((conductor, _state, _rx)) => {
            if CONDUCTOR.set(conductor).is_ok() {
                info!("FFI: Conductor initialized.");
                true
            } else {
                error!("FFI: Failed to store Conductor in OnceLock.");
                false
            }
        }
        Err(e) => {
            error!("FFI: Conductor init failed: {:?}", e);
            false
        }
    }
}

/// Signals graceful shutdown of the runtime.
#[no_mangle]
pub extern "C" fn aetherion_shutdown_runtime() {
    if let Some(conductor) = CONDUCTOR.get() {
        conductor.signal_shutdown_graceful();
        info!("FFI: Shutdown signal sent.");
    }
}

/// Checks if the engine is ready.
#[no_mangle]
pub extern "C" fn aetherion_is_runtime_ready() -> bool {
    CONDUCTOR.get().is_some()
}

// ── Compatibility & Debug ───────────────────────────────────────────────────

/// Legacy: Triggers placeholder test (CLI use).
#[no_mangle]
pub extern "C" fn aetherion_trigger_runtime_test() {
    info!("FFI: Runtime test triggered (placeholder).");
}

/// Alias for `aetherion_start_runtime`.
#[no_mangle]
pub extern "C" fn aetherion_initialize_engine() -> bool {
    aetherion_start_runtime()
}

// ── String-Safe Status Reporting ───────────────────────────────────────────

/// Returns a C-string with engine status. Caller must free with `aetherion_free_string`.
#[no_mangle]
pub extern "C" fn aetherion_get_status(id: u32) -> *mut std::os::raw::c_char {
    let status = format!(
        "Aetherion Engine [ID: {}] — Runtime: {}",
        id,
        if CONDUCTOR.get().is_some() { "ACTIVE" } else { "INACTIVE" }
    );

    match CString::new(status) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => {
            let fallback = CString::new("ERROR: Invalid status string").unwrap();
            fallback.into_raw()
        }
    }
}

/// Frees a string allocated by Rust FFI functions.
#[no_mangle]
pub extern "C" fn aetherion_free_string(s: *mut std::os::raw::c_char) {
    if s.is_null() {
        return;
    }
    unsafe { let _ = CString::from_raw(s); }
}