// In src/interface/data_bridge.rs

use parking_lot::Mutex;
use once_cell::sync::Lazy;

// This is our global singleton. It holds a single message,
// which the Godot main thread can take ownership of.
static DATA_MAILBOX: Lazy<Mutex<Option<String>>> = Lazy::new(|| {
    Mutex::new(None)
});

// A function for the background thread to push a new message into the mailbox.
pub fn push_message(message: String) {
    let mut mailbox = DATA_MAILBOX.lock();
    *mailbox = Some(message);
}

// A function for the main thread to take ownership of the message,
// leaving the mailbox empty. This is an atomic "poll".
pub fn take_message() -> Option<String> {
    if let Some(mut mailbox) = DATA_MAILBOX.try_lock() {
        // ⚠️ NEW DEBUG LOG: Check if we actually acquired the lock
        godot::prelude::godot_print!("Rust `data_bridge`: Successfully acquired mutex lock!");
        mailbox.take()
    } else {
        None
    }
}
