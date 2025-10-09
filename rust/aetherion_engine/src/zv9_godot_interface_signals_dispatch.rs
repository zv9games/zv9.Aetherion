use godot::prelude::*;
use godot::global::Error;

use crate::zv9_prelude::*;
use crate::zv9_godot_interface_map_ext::MapDataChunkExt;
use aetherion_core::shared::EngineMessage;
use serde_json::Value;

/// 📡 Emits signals from an `EngineMessage` to the connected Godot node.
/// Returns an Error code from the Godot signal system.
pub fn emit_from_message(signals_node: &mut Gd<AetherionSignals>, msg: EngineMessage) -> Error {
    match msg {
        // ✅ Core generation
        EngineMessage::Start => {
            godot_print!("📡 Dispatch → build_map_start");
            signals_node.emit_signal("build_map_start", &[])
        }

        EngineMessage::Progress(percent) => {
            godot_print!("📡 Dispatch → generation_progress: {}%", percent);
            signals_node.emit_signal("generation_progress", &[percent.to_variant()])
        }

        EngineMessage::Status(status) => {
            godot_print!("📡 Dispatch → map_building_status: {}", status);
            signals_node.emit_signal("map_building_status", &[GString::from(status.as_str()).to_variant()])
        }

        EngineMessage::Complete { width, height, mode, animate, duration } => {
            godot_print!(
                "📡 Dispatch → generation_complete: {}x{}, mode={}, animate={}, duration={}",
                width, height, mode, animate, duration
            );
            let mut dict = Dictionary::new();
            let _ = dict.insert("width", width);
            let _ = dict.insert("height", height);
            let _ = dict.insert("mode", mode);
            let _ = dict.insert("animate", animate);
            let _ = dict.insert("duration", duration);
            signals_node.emit_signal("generation_complete", &[dict.to_variant()])
        }

        EngineMessage::MapChunkReady => {
            godot_print!("📡 Dispatch → map_chunk_ready");
            signals_node.emit_signal("map_chunk_ready", &[])
        }

        // 🔁 Chunk delivery
        EngineMessage::ChunkReady(chunk) => {
            godot_print!("📡 Dispatch → chunk_ready");
            let dict = chunk.to_dictionary();
            signals_node.emit_signal("chunk_ready", &[dict.to_variant()])
        }

        // 🧠 Lifecycle
        EngineMessage::Cancelled => {
            godot_print!("📡 Dispatch → map_build_cancelled");
            signals_node.emit_signal("map_build_cancelled", &[])
        }

        EngineMessage::Diagnostics { memory_usage, thread_count, tick_rate } => {
            godot_print!(
                "📡 Dispatch → diagnostics: memory={}MB, threads={}, tick_rate={}",
                memory_usage, thread_count, tick_rate
            );
            signals_node.emit_signal("diagnostics", &[
                memory_usage.to_variant(),
                (thread_count as i32).to_variant(),
                tick_rate.to_variant(),
            ])
        }

        // ⚠️ Error & warning
        EngineMessage::Error(msg) => {
            godot_warn!("🚨 Dispatch → rust_error: {}", msg);
            signals_node.emit_signal("rust_error", &[GString::from(msg.as_str()).to_variant()])
        }

        EngineMessage::Warning(msg) => {
            godot_print!("⚠️ Dispatch → rust_warning: {}", msg);
            signals_node.emit_signal("rust_warning", &[GString::from(msg.as_str()).to_variant()])
        }

        // 🧪 Custom hook
        EngineMessage::Custom { name, payload } => {
            godot_print!("📡 Dispatch → custom_event: {}", name);
            signals_node.emit_signal("custom_event", &[
                GString::from(name.as_str()).to_variant(),
                json_to_variant(payload),
            ])
        }

        // 🧭 New lifecycle variants
        EngineMessage::Paused => {
            godot_print!("📡 Dispatch → engine_paused");
            signals_node.emit_signal("engine_paused", &[])
        }

        EngineMessage::Resumed => {
            godot_print!("📡 Dispatch → engine_resumed");
            signals_node.emit_signal("engine_resumed", &[])
        }

        EngineMessage::Retry => {
            godot_print!("📡 Dispatch → engine_retry");
            signals_node.emit_signal("engine_retry", &[])
        }
    }
}

/// 🔄 Converts a serde_json::Value into a Godot Variant.
/// Expand this as needed to support arrays and objects.
pub fn json_to_variant(value: Value) -> Variant {
    match value {
        Value::String(s) => GString::from(s.as_str()).to_variant(),
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
