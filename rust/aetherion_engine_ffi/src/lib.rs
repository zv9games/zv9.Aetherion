use aetherion_generate::run_ai_generation;
use std::ffi::{CStr, CString};

// IMPORTANT: The `extern "C"` block is essential for FFI
#[no_mangle]
pub extern "C" fn aetherion_initialize_engine() -> bool {
    aetherion_shared::initialize_shared_data();
    aetherion_generate::is_initialized()
}

#[no_mangle]
pub extern "C" fn aetherion_get_status(id: u32) -> *mut std::os::raw::c_char {
    let status = format!("Engine status for id {}: Running", id);
    // Convert to CString for C compatibility
    CString::new(status).unwrap().into_raw()
}

// Function to free the memory allocated by Rust in C
#[no_mangle]
pub extern "C" fn aetherion_free_string(s: *mut std::os::raw::c_char) {
    unsafe {
        if s.is_null() { return }
        _ = CString::from_raw(s);
    }
}