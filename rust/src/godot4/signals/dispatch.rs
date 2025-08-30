use godot::prelude::*;
use godot::global::Error;
use crate::godot4::signals::definitions::AetherionSignals;
use crate::godot4::messaging::messages::EngineMessage;

/// Emits signals from EngineMessage to the connected Godot node.
pub fn emit_from_message(signals_node: &mut Gd<AetherionSignals>, msg: EngineMessage) -> Error {
    match msg {
        EngineMessage::Start => signals_node.emit_signal("build_map_start", &[]),
        EngineMessage::Progress(p) => signals_node.emit_signal("generation_progress", &[p.to_variant()]),
        EngineMessage::Status(s) => signals_node.emit_signal("map_building_status", &[GString::from(s).to_variant()]),
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
