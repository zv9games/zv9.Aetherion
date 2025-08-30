use godot::prelude::*;
use godot::classes::TileMap;
use godot::global::Error;
use crate::godot4::signals::AetherionSignals;
use crate::godot4::messaging::{GodotSync, EngineMessage};
use crate::aetherion::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

/// The main Godot-facing engine node.
/// Exposes procedural generation and signal dispatch to GDScript.
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
        godot_print!("?? AetherionEngine is ready.");
        self.base.to_init_gd().set_process(true);
    }

    fn process(&mut self, _delta: f64) {
        let chunks = self.sync.drain_chunks();
        let signals = self.sync.drain_signals();

        if let Some(mut tilemap) = self.target_tilemap.as_mut() {
            for chunk in chunks {
                for (pos, tile_info) in chunk.chunk {
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
    pub fn build_custom_map(
        &mut self,
        width: i32,
        height: i32,
        seed: u64,
        mode: String,
        animate: bool,
        black: SerializableVector2i,
        blue: SerializableVector2i,
    ) {
        let sync_clone = self.sync.clone();

        rayon::spawn(move || {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let total_tiles = (width as u64) * (height as u64);
            let batch_size = 1_000;
            let mut chunk = MapDataChunk::new();
            let mut placed = 0;

            sync_clone.add_signal(EngineMessage::Start);
            sync_clone.add_signal(EngineMessage::Status("Building map".into()));

            for y in 0..height {
                for x in 0..width {
                    let pos = SerializableVector2i { x, y };
                    let atlas = match mode.as_str() {
                        "checkerboard" => if (x + y) % 2 == 0 { black.clone() } else { blue.clone() },
                        "clustered" => if rng.gen_bool(0.7) { black.clone() } else { blue.clone() },
                        "noise" => if rng.gen_bool(0.5) { black.clone() } else { blue.clone() },
                        _ => if rng.gen_bool(0.5) { black.clone() } else { blue.clone() },
                    };

                    chunk.insert(pos, TileInfo {
                        source_id: 0,
                        atlas_coords: atlas,
                        alternate_id: 0,
                    });

                    placed += 1;
                    if placed % batch_size == 0 || placed as u64 == total_tiles {
                        let percent = ((placed as u64 * 100) / total_tiles) as i32;
                        sync_clone.add_signal(EngineMessage::Progress(percent));
                        sync_clone.add_chunk(chunk);
                        chunk = MapDataChunk::new();
                    }
                }
            }

            if !chunk.chunk.is_empty() {
                sync_clone.add_chunk(chunk);
            }

            sync_clone.add_signal(EngineMessage::Progress(100));
            let duration = std::time::Instant::now().elapsed().as_secs_f64();
            sync_clone.add_signal(EngineMessage::Status("Terrain generation complete".into()));
            sync_clone.add_signal(EngineMessage::Complete {
                width,
                height,
                mode,
                animate,
                duration,
            });
        });
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
