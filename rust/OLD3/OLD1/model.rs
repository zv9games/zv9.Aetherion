// src/model.rs

use godot::prelude::*;

// This is the struct for data that is safe to send between threads.
// It is a simple Rust struct, NOT a GodotClass.
// It is `Clone` and `Send` for safe use in the message channel.
#[derive(Debug, Clone, Send)]
pub struct ThreadSafeDebugInfo {
    pub message: String,
}

// This is the struct for data that is exposed to Godot as a class.
// It is a GodotClass and lives on the main thread.
#[derive(Debug, GodotClass)]
#[class(base=RefCounted)]
pub struct DebugInfo {
    #[base]
    base: Base<RefCounted>,
    
    // The `#[export]` macro makes this property visible in Godot.
    #[export]
    pub message: GString,
}

#[godot_api]
impl DebugInfo {
    // A proper `init` constructor is required for GodotClass.
    #[func]
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            message: GString::new(),
        }
    }

    // This method helps populate the Godot object from our thread-safe struct.
    pub fn from_thread_safe(info: ThreadSafeDebugInfo) -> Gd<Self> {
        let mut new_info = DebugInfo::new_gd();
        new_info.bind_mut().message = GString::from(info.message);
        new_info
    }
}


// This enum represents a message from the thread to the main thread.
// It now holds the simple Rust struct which is `Send`.
pub enum Message {
    DebugInfo(ThreadSafeDebugInfo),
    MapLoadComplete,
}
