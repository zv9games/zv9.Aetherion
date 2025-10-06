// zv9_godot_interface_api_engine_signals.rs

use godot::prelude::*;
use godot::global::Error;
use crate::zv9_godot_interface_map_ext::MapDataChunkExt;
use crate::zv9_godot_interface_api_engine_core::AetherionEngine;

#[func]
pub fn emit_pending_signals(engine: &mut AetherionEngine) {
    if let Some(signals_node) = engine.signals_node.as_mut() {
        let mut signals = engine.sync.drain_signals();

        if let Some(conductor) = engine.conductor.as_mut() {
            signals.extend(conductor.streamer_mut().delivery_mut().sync.drain_signals());
        }

        for signal_msg in signals {
            let result = match signal_msg {
                EngineMessage::Status(status) => {
                    engine.current_status = status.clone();
                    signals_node.emit_signal("map_building_status", &[GString::from(status).to_variant()])
                }
                EngineMessage::Progress(percent) => signals_node.emit_signal("generation_progress", &[percent.to_variant()]),
                EngineMessage::Complete { width, height, mode, animate, duration } => {
                    let mut dict = Dictionary::new();
                    dict.insert("width", width);
                    dict.insert("height", height);
                    dict.insert("mode", mode);
                    dict.insert("animate", animate);
                    dict.insert("duration", duration);
                    signals_node.emit_signal("generation_complete", &[dict.to_variant()])
                }
                EngineMessage::ChunkReady(chunk) => {
                    let dict = chunk.to_dictionary();
                    signals_node.emit_signal("chunk_ready", &[dict.to_variant()])
                }
                _ => Error::OK,
            };

            if result != Error::OK {
                godot_warn!("⚠️ Signal emission failed: {:?}", result);
            }
        }
    }
}

#[godot_api]
impl AetherionEngine {
    #[func]
    pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.target_tilemap = Some(tilemap);
    }

    #[func]
    pub fn apply_chunks_to_tilemap(&mut self) {
        if let Some(tilemap) = self.target_tilemap.as_mut() {
            for chunk in self.sync.drain_chunks() {
                for (pos, tile_info) in chunk.tiles {
                    tilemap.set_cell_ex(0, Vector2i::new(pos.x, pos.y))
                        .source_id(tile_info.source_id)
                        .atlas_coords(Vector2i::new(tile_info.atlas_coords.x, tile_info.atlas_coords.y))
                        .alternative_tile(tile_info.alternate_id)
                        .done();
                }
            }
        }
    }

    #[func]
    pub fn debug_place_tile(&mut self, x: i32, y: i32) {
        if let Some(tilemap) = self.target_tilemap.as_mut() {
            tilemap.set_cell_ex(0, Vector2i::new(x, y))
                .source_id(0)
                .atlas_coords(Vector2i::new(14, 13))
                .alternative_tile(0)
                .done();
        }
    }
}
