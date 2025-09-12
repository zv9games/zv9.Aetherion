/// ✅ Suggestions for godot4/messaging/dispatch.rs

// 🔧 Expand `json_to_variant()` support:
//     - Handle arrays (`Value::Array`) and objects (`Value::Object`) recursively
//     - Enables richer custom event payloads and structured data

// 🧩 Add signal emission logging or tracing:
//     - Log signal name and payload for debugging or profiling
//     - Could integrate with a diagnostics overlay or telemetry system

// 🚦 Add fallback or retry logic for failed emissions:
//     - If `emit_signal()` returns non-OK, optionally retry or queue for later
//     - Prevents silent failures in high-load scenarios

// 📚 Document signal mappings clearly:
//     - Include a table or comment block mapping `EngineMessage` variants to signal names
//     - Improves maintainability and onboarding for contributors

// 🧪 Add unit tests for dispatch logic:
//     - Validate correct signal emission for each `EngineMessage` variant
//     - Ensure `json_to_variant()` handles edge cases and malformed input

// 🧼 Optional: Add helper for batch dispatch:
//     - `fn emit_all(signals_node: &mut Gd<AetherionSignals>, messages: &[EngineMessage])`
//     - Useful for draining queues or replaying events

// 🚀 Future: Add signal filtering or routing:
//     - e.g. `fn emit_filtered(...)` with predicate or category
//     - Enables selective dispatch based on runtime state

// 🧠 Consider exposing dispatch metadata:
//     - e.g. `fn describe_message(msg: &EngineMessage) -> String`
//     - Useful for debugging, logging, or UI overlays


use godot::prelude::*;
use godot::global::Error;
use crate::godot4::api::AetherionSignals;
use crate::godot4::messaging::messages::EngineMessage;
use serde_json::Value;

/// Emits signals from an `EngineMessage` to the connected Godot node.
/// Returns an Error code from the Godot signal system.
pub fn emit_from_message(signals_node: &mut Gd<AetherionSignals>, msg: EngineMessage) -> Error {
    match msg {
        // ✅ Core generation
        EngineMessage::Start => signals_node.emit_signal("build_map_start", &[]),

        EngineMessage::Progress(percent) => {
            signals_node.emit_signal("generation_progress", &[percent.to_variant()])
        }

        EngineMessage::Status(status) => {
            signals_node.emit_signal("map_building_status", &[GString::from(status).to_variant()])
        }

        EngineMessage::Complete { width, height, mode, animate, duration } => {
            let mut dict = Dictionary::new();
            let _ = dict.insert("width", width);
            let _ = dict.insert("height", height);
            let _ = dict.insert("mode", mode);
            let _ = dict.insert("animate", animate);
            let _ = dict.insert("duration", duration);
            signals_node.emit_signal("generation_complete", &[dict.to_variant()])
        }

        EngineMessage::MapChunkReady => signals_node.emit_signal("map_chunk_ready", &[]),

        // 🔁 Chunk delivery
        EngineMessage::ChunkReady(chunk) => {
            let dict = chunk.to_dictionary();
            signals_node.emit_signal("chunk_ready", &[dict.to_variant()])
        }

        // 🧠 Lifecycle
        EngineMessage::Cancelled => signals_node.emit_signal("map_build_cancelled", &[]),

        EngineMessage::Diagnostics { memory_usage, thread_count, tick_rate } => {
            signals_node.emit_signal("diagnostics", &[
                memory_usage.to_variant(),
                (thread_count as i32).to_variant(),
                tick_rate.to_variant(),
            ])
        }

        // ⚠️ Error & warning
        EngineMessage::Error(msg) => {
            signals_node.emit_signal("rust_error", &[GString::from(msg).to_variant()])
        }

        EngineMessage::Warning(msg) => {
            signals_node.emit_signal("rust_warning", &[GString::from(msg).to_variant()])
        }

        // 🧪 Custom hook
        EngineMessage::Custom { name, payload } => {
            signals_node.emit_signal("custom_event", &[
                GString::from(name).to_variant(),
                json_to_variant(payload),
            ])
        }

        // 🧭 New lifecycle variants
        EngineMessage::Paused => signals_node.emit_signal("engine_paused", &[]),
        EngineMessage::Resumed => signals_node.emit_signal("engine_resumed", &[]),
        EngineMessage::Retry => signals_node.emit_signal("engine_retry", &[]),
    }
}


/// Converts a serde_json::Value into a Godot Variant.
/// Expand this as needed to support arrays and objects.
pub fn json_to_variant(value: Value) -> Variant {
    match value {
        Value::String(s) => GString::from(s).to_variant(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                (i as i32).to_variant()
            } else if let Some(f) = n.as_f64() {
                f.to_variant()
            } else {
                Variant::nil()
            }
        }
        Value::Bool(b) => b.to_variant(),
        Value::Null => Variant::nil(),
        _ => Variant::nil(), // Expand later for arrays/objects
    }
}
