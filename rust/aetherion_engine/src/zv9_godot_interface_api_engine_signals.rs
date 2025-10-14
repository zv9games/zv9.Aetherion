use godot::prelude::*;
use godot::global::Error;
//use godot::classes::TileMap;

use crate::zv9_godot_interface_map_ext::MapDataChunkExt;
use crate::zv9_godot_interface_api_engine_core::AetherionEngine;
use aetherion_shared::shared::EngineMessage;

/// 📡 Emits all pending signals from engine and conductor into Godot.
pub fn emit_pending_signals(engine: &mut AetherionEngine) {
    if let Some(signals_node) = engine.signals_node.as_mut() {
        let mut signals = engine.sync.drain_signals();

        if let Some(conductor) = engine.conductor.as_mut() {
            let conductor_signals = conductor.streamer_mut().delivery_mut().sync.drain_signals();
            signals.extend(conductor_signals);
        }

        for signal_msg in signals {
            let result = match signal_msg {
                EngineMessage::Status(status) => {
                    engine.current_status = status.clone();
                    godot_print!("📡 Emitting 'map_building_status' → {}", status);
                    signals_node.emit_signal("map_building_status", &[GString::from(&status).to_variant()])

                }
                EngineMessage::Progress(percent) => {
                    godot_print!("📡 Emitting 'generation_progress' → {}%", percent);
                    signals_node.emit_signal("generation_progress", &[percent.to_variant()])
                }
                EngineMessage::Complete { width, height, mode, animate, duration } => {
                    godot_print!("📡 Emitting 'generation_complete' → {}x{}, mode={}, animate={}, duration={}", width, height, mode, animate, duration);
                    let mut dict = Dictionary::new();
                    let _ = dict.insert("width", width);
                    let _ = dict.insert("height", height);
                    let _ = dict.insert("mode", mode);
                    let _ = dict.insert("animate", animate);
                    let _ = dict.insert("duration", duration);
                    signals_node.emit_signal("generation_complete", &[dict.to_variant()])
                }
                EngineMessage::ChunkReady(chunk) => {
                    godot_print!("📡 Emitting 'chunk_ready'");
                    let dict = chunk.to_dictionary();
                    signals_node.emit_signal("chunk_ready", &[dict.to_variant()])
                }
                _ => Error::OK,
            };

            if result != Error::OK {
                godot_warn!("⚠️ Signal emission failed: {:?}", result);
            }
        }
    } else {
        godot_warn!("⚠️ No signals_node assigned. Cannot emit signals.");
    }
}

/// 🧱 Applies drained chunks to the assigned TileMap.
pub fn apply_chunks_to_tilemap(engine: &mut AetherionEngine) {
    if let Some(tilemap) = engine.target_tilemap.as_mut() {
        for chunk in engine.sync.drain_chunks() {
            godot_print!("🧱 Applying chunk with {} tiles", chunk.tiles.len());
            for (pos, tile_info) in chunk.tiles {
                tilemap.set_cell_ex(0, Vector2i::new(pos.x, pos.y))
                    .source_id(tile_info.source_id)
                    .atlas_coords(Vector2i::new(tile_info.atlas_coords.x, tile_info.atlas_coords.y))
                    .alternative_tile(tile_info.alternate_id)
                    .done();
            }
        }
    } else {
        godot_warn!("⚠️ No TileMap assigned. Cannot apply chunks.");
    }
}

/// 🧪 Places a debug tile at the given coordinates.

pub fn debug_place_tile(engine: &mut AetherionEngine, x: i32, y: i32) {
    if let Some(tilemap) = engine.target_tilemap.as_mut() {
        godot_print!("🧪 Placing debug tile at ({}, {})", x, y);
        tilemap.set_cell_ex(0, Vector2i::new(x, y))
            .source_id(0)
            .atlas_coords(Vector2i::new(14, 13))
            .alternative_tile(0)
            .done();
    } else {
        godot_warn!("⚠️ No TileMap assigned. Cannot place debug tile.");
    }
}


