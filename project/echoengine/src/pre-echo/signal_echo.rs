// src/signal_echo.rs

use godot::prelude::*;
use godot::builtin::Variant;

/// 🪞 Emits a trace signal for any other signal — for debugging, overlays, or ritual logging.
pub fn log_signal_emission(signal_name: &str, origin: &str) {
    let echo = format!("📡 {} from {}", signal_name, origin);
    SceneTree::singleton()
        .root()
        .emit_signal("signal_echo", &[Variant::from(echo)]);
}

/// 🪞 Emits a trace signal with optional payload preview
pub fn log_signal_with_payload(signal_name: &str, origin: &str, payload: Option<&Variant>) {
    let preview = match payload {
        Some(p) => format!("📡 {} from {} → {:?}", signal_name, origin, p),
        None => format!("📡 {} from {}", signal_name, origin),
    };
    SceneTree::singleton()
        .root()
        .emit_signal("signal_echo", &[Variant::from(preview)]);
}
