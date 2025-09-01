// common.rs

use godot::prelude::*;

/// Shared constants for testing and diagnostics
pub const DEFAULT_PROGRESS: i32 = 50;
pub const DEFAULT_DURATION: f64 = 1.23;

/// Helper to create a dummy Dictionary for signal testing
pub fn dummy_results() -> Dictionary {
    let mut dict = Dictionary::new();
    dict.insert("status", "success");
    dict.insert("chunks", 42);
    dict
}

/// Helper to create a GString from &str
pub fn gstr(text: &str) -> GString {
    GString::from(text)
}

/// Mock signal receiver node for integration tests
#[derive(GodotClass)]
#[class(base = Node)]
pub struct SignalReceiver {
    #[base]
    base: Base<Node>,
    pub last_signal: Option<GString>,
}

#[godot_api]
impl INode for SignalReceiver {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            last_signal: None,
        }
    }
}

#[godot_api]
impl SignalReceiver {
    #[func]
    fn on_progress(&mut self, percent: i32) {
        godot_print!("Received progress: {}", percent);
        self.last_signal = Some(GString::from(format!("progress: {}", percent)));
    }

    #[func]
    fn on_status(&mut self, status: GString) {
        godot_print!("Received status: {}", status);
        self.last_signal = Some(status);
    }
}
