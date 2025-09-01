use godot::prelude::*;
use godot::global::Error;
use crate::godot4::api::AetherionSignals;
use crate::godot4::messaging::messages::EngineMessage;

/// Emits signals from an `EngineMessage` to the connected Godot node.
/// Returns an Error code from the Godot signal system.
pub fn emit_from_message(signals_node: &mut Gd<AetherionSignals>, msg: EngineMessage) -> Error {
    match msg {
        EngineMessage::Start => signals_node.emit_signal("build_map_start", &[]),

        EngineMessage::Progress(percent) => {
            signals_node.emit_signal("generation_progress", &[percent.to_variant()])
        }

        EngineMessage::Status(status) => {
            signals_node.emit_signal("map_building_status", &[GString::from(status).to_variant()])
        }

        EngineMessage::Complete { width, height, mode, animate, duration } => {
            let mut dict = Dictionary::new();
            dict.insert("width", width);
            dict.insert("height", height);
            dict.insert("mode", mode);
            dict.insert("animate", animate);
            dict.insert("duration", duration);
            signals_node.emit_signal("generation_complete", &[dict.to_variant()])
        }
    }
}
