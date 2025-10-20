//! Low-level C FFI Bridge for Godot Communication.
//!
//! This module exposes functions to C/Godot that manage the Aetherion runtime lifecycle
//! and facilitate the transfer of generated chunk data.

use std::ffi::CString;
use std::sync::OnceLock;
use std::ptr;

// --- EXTERNAL CRATE DEPENDENCIES ---
use aetherion_generate::{Conductor, start_runtime_placeholder};
use aetherion_math::Vec2i;
use aetherion_shared::chunk_data::CHUNK_SIZE;
use tracing::info;

// --- 0. FFI-SAFE DATA STRUCTURES ---

/// C-compatible structure to transfer a generated chunk's tile map.
/// This structure holds a pointer to a flat array of raw u8 tile types.
#[repr(C)]
pub struct FFITileTypeArray {
    pub tiles_ptr: *mut u8,
    pub size: usize, // Total number of tiles (CHUNK_SIZE * CHUNK_SIZE)
}

// --- 1. Conductor Management (Singleton State) ---

/// A thread-safe, lazily initialized global instance of the Conductor.
static CONDUCTOR: OnceLock<Conductor> = OnceLock::new();

/// Initializes the Aetherion Runtime (Conductor) and stores it globally.
/// This is the canonical start function for the engine core.
#[no_mangle]
pub extern "C" fn aetherion_start_runtime() -> bool {
    if CONDUCTOR.get().is_some() {
        info!("FFI Bridge: Runtime already running.");
        return true;
    }

    // Call the Conductor initializer
    match Conductor::new() {
        Ok((conductor, _state)) => {
            if CONDUCTOR.set(conductor).is_err() {
                // Should not happen if the lock is successfully acquired
                return false;
            }
            info!("FFI Bridge: Conductor Runtime started successfully.");
            true
        }
        Err(e) => {
            tracing::error!("FFI Bridge: Failed to initialize Conductor: {:?}", e);
            false
        }
    }
}

/// Gracefully shuts down the Conductor by signaling its internal state.
/// The Conductor instance remains in the OnceLock to be reused or queried.
#[no_mangle]
pub extern "C" fn aetherion_shutdown_runtime() {
    // RESOLVED: We now retrieve an immutable reference and call the non-consuming
    // `signal_shutdown_graceful` method (implemented in aetherion_generate/src/conductor.rs).
    if let Some(conductor) = CONDUCTOR.get() {
        conductor.signal_shutdown_graceful();
        info!("FFI Bridge: Conductor Runtime signalled for shutdown.");
    }
}

/// Checks if the engine's asynchronous core has been successfully initialized.
#[no_mangle]
pub extern "C" fn aetherion_is_runtime_ready() -> bool {
    CONDUCTOR.get().is_some()
}

// --- 2. Generation Bridge ---

/// Generates a single chunk at the given 2D coordinates using the active algorithm.
///
/// Returns a pointer to an FFITileTypeArray containing the tile data.
/// The caller is responsible for freeing this memory using `aetherion_free_chunk_result`.
#[no_mangle]
pub extern "C" fn aetherion_generate_chunk_2d(chunk_x: i32, chunk_y: i32) -> *mut FFITileTypeArray {
    let conductor = match CONDUCTOR.get() {
        Some(c) => c,
        None => {
            tracing::error!("FFI Bridge: Cannot generate chunk. Conductor is not running.");
            return ptr::null_mut();
        }
    };

    let coords = Vec2i { x: chunk_x, y: chunk_y };
    let chunk_data = conductor.generate_single_chunk(coords);

    let total_size = (CHUNK_SIZE * CHUNK_SIZE) as usize;
    let mut tile_array: Vec<u8> = Vec::with_capacity(total_size);

    // Convert TileData structs to a flat array of raw u8 tile types
    for tile in chunk_data.tiles {
        // ASSUMPTION: TileType is a simple enum safely cast to u8
        tile_array.push(tile.tile_type as u8);
    }
    
    // Box the vector and then leak the memory, giving ownership to the C side
    let mut boxed_slice = tile_array.into_boxed_slice();
    let tiles_ptr = boxed_slice.as_mut_ptr();
    // Prevent the Box from being dropped and freeing the memory
    std::mem::forget(boxed_slice);

    // Allocate the result struct on the heap
    let result = Box::new(FFITileTypeArray {
        tiles_ptr,
        size: total_size,
    });

    Box::into_raw(result)
}

/// Frees the memory allocated for the FFITileTypeArray (both the struct and the data buffer).
/// This must be called by the C side after consuming the chunk data.
#[no_mangle]
pub extern "C" fn aetherion_free_chunk_result(result_ptr: *mut FFITileTypeArray) {
    if result_ptr.is_null() {
        return;
    }
    
    // Safety: Retake ownership of the Box<FFITileTypeArray>
    // This frees the memory allocated for the FFI structure itself.
    let result = unsafe { Box::from_raw(result_ptr) };
    
    // Safety: Recreate the Box<[u8]> from the raw components
    // This allows the Rust memory allocator to reclaim the linear tile data array.
    let _ = unsafe { 
        Vec::from_raw_parts(result.tiles_ptr, result.size, result.size).into_boxed_slice()
    };
    
    // When `result` goes out of scope, the Box<FFITileTypeArray> is dropped, freeing the struct memory.
    info!("FFI Bridge: Freed chunk result memory.");
}

// --- 3. Compatibility and Utility Functions ---

/// Triggers the structural test of the Conductor (used by CLI menu).
#[no_mangle]
pub extern "C" fn aetherion_trigger_runtime_test() {
    info!("FFI Bridge: Received command to trigger Conductor structural test.");
    // Calls the standalone Conductor::new/shutdown pair
    start_runtime_placeholder();
    info!("FFI Bridge: Conductor test sequence complete.");
}

/// The original `aetherion_initialize_engine` is now superseded by `aetherion_start_runtime`,
/// but is kept for compatibility. It now calls the new runtime start.
#[no_mangle]
pub extern "C" fn aetherion_initialize_engine() -> bool {
    // NOTE: This call to initialize_shared_data assumes it is idempotent and safe.
    aetherion_shared::initialize_shared_data();
    aetherion_start_runtime()
}


#[no_mangle]
pub extern "C" fn aetherion_get_status(id: u32) -> *mut std::os::raw::c_char {
    let status = format!(
        "Engine status for id {}: Runtime Running: {}",
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