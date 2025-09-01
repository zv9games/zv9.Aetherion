use godot::prelude::*;
use godot::classes::TileMap;
use godot::global::Error;

use crate::godot4::api::AetherionSignals;
use crate::godot4::messaging::{GodotSync, EngineMessage};
use crate::aetherion::pipeline::data::{MapDataChunk, TileInfo, MapBuildOptions};
use crate::aetherion::pipeline::builder::threaded::spawn_builder_thread;

/// Godot-facing engine node for procedural generation and signal dispatch.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionEngine {
    #[base]
    base: Base<Node>,
    sync: GodotSync,

    #[export]
    signals_node: Option<Gd<AetherionSignals>>,

    #[export]
    target_tilemap: Option<Gd<TileMap>>,
}

#[godot_api]
impl AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            sync: GodotSync::init(),
            signals_node: None,
            target_tilemap: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("📡 AetherionEngine is ready.");
        self.base.to_init_gd().set_process(true);
    }

    fn process(&mut self, _delta: f64) {
        let chunks = self.sync.drain_chunks();
        let signals = self.sync.drain_signals();

        if let Some(mut tilemap) = self.target_tilemap.as_mut() {
            for chunk in chunks {
                for (pos, tile_info) in chunk.tiles {
                    tilemap.set_cell_ex(0, pos.into())
                        .source_id(tile_info.source_id)
                        .atlas_coords(tile_info.atlas_coords.into())
                        .alternative_tile(tile_info.alternate_id)
                        .done();
                }
            }
        }

        if let Some(mut signals_node) = self.signals_node.as_mut() {
            for signal_msg in signals {
                let result = match signal_msg {
                    EngineMessage::Start => signals_node.emit_signal("build_map_start", &[]),
                    EngineMessage::Progress(percent) => signals_node.emit_signal("generation_progress", &[percent.to_variant()]),
                    EngineMessage::Status(status) => signals_node.emit_signal("map_building_status", &[GString::from(status).to_variant()]),
                    EngineMessage::Complete { width, height, mode, animate, duration } => {
                        let mut dict = Dictionary::new();
                        dict.insert("width", width);
                        dict.insert("height", height);
                        dict.insert("mode", mode);
                        dict.insert("animate", animate);
                        dict.insert("duration", duration);
                        signals_node.emit_signal("generation_complete", &[dict.to_variant()])
                    }
                };

                if result != Error::OK {
                    godot_warn!("Signal emission failed: {:?}", result);
                }
            }
        }
    }

    #[func]
    pub fn aetherionoracle(&mut self) {
        self.process(0.0);
    }

    #[func]
    fn build_map(
        &mut self,
        width: i32,
        height: i32,
        seed: i64,
        mode: String,
        animate: bool,
        black: Vector2i,
        blue: Vector2i,
    ) {
        let config = MapBuildOptions {
            width,
            height,
            seed,
            mode: match mode.as_str() {
                "automata" => "automata".into(),
                "basic" => "basic".into(),
                _ => "basic".into(),
            },
            animate,
            black,
            blue,
        };

        spawn_builder_thread(self.sync.clone(), config);
    }

    #[func]
    pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.target_tilemap = Some(tilemap);
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
