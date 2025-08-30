use godot::prelude::*;
use godot::classes::{TileMap, TileSetAtlasSource, Node, INode};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use std::sync::{Arc, Mutex};
use crate::godot::messages::{GodotSync, EngineMessage, TilePlacementMessage, TileInfo};
use crate::godot::api::AetherionSignals;
use rayon::spawn;

// Note: I've moved the `GodotSync` related types to a new `messages.rs` file
// within the `godot` module to comply with our proposed structure.
// You'll need to create that file next.

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct AetherionEngine {
    #[base]
    base: Base<Node>,
    #[export]
    tilemap: Option<Gd<TileMap>>,
    #[export]
    signals_node: Option<Gd<AetherionSignals>>,
    sync: GodotSync,
    is_generating: bool,
}

#[godot_api]
impl AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            tilemap: None,
            signals_node: None,
            sync: GodotSync::init(),
            is_generating: false,
        }
    }

    #[func]
    fn _ready(&mut self) {
        godot_print!("🧠 AetherionEngine is ready.");
        self.base.to_gd().set_process(true);
    }

    #[func]
    fn _process(&mut self, _delta: f64) {
        if !self.is_generating {
            return;
        }

        // DRAIN AND APPLY TILES
        let tiles_to_place = self.sync.drain_tiles();
        if let Some(mut tilemap) = self.tilemap.as_mut() {
            for tile_msg in tiles_to_place {
                tilemap.set_cell(
                    0,
                    tile_msg.position,
                    tile_msg.info.source_id,
                    tile_msg.info.atlas_coords,
                    tile_msg.info.alternate_id,
                );
            }
        }
        
        // DRAIN AND EMIT SIGNALS
        if let Some(mut signals_node) = self.signals_node.as_mut() {
            let signals = self.sync.drain_signals();
            for msg in signals {
                match msg {
                    EngineMessage::Progress(percent) => {
                        signals_node.emit_signal("emit_generation_progress".into(), &[Variant::from(percent)]);
                    }
                    EngineMessage::Status(status) => {
                        signals_node.emit_signal("emit_map_building_status".into(), &[Variant::from(status)]);
                    }
                    EngineMessage::Complete { width, height, mode, animate, duration } => {
                        let mut dict = Dictionary::new();
                        dict.insert("width", width);
                        dict.insert("height", height);
                        dict.insert("mode", mode);
                        dict.insert("animate", animate);
                        dict.insert("duration", duration);
                        signals_node.emit_signal("emit_generation_complete".into(), &[dict.to_variant()]);
                        self.is_generating = false; // Stop processing after completion
                    }
                    EngineMessage::Start => {
                        signals_node.emit_signal("emit_build_map_start".into(), &[]);
                    }
                }
            }
        }
    }

    // This function now only SPAWNS the generation process, it does not block!
    #[func]
    pub fn build_custom_map(
        &mut self,
        width: i32,
        height: i32,
        seed: u64,
        black: Vector2i,
        blue: Vector2i,
        animate: bool,
        mode: String,
    ) {
        if self.is_generating {
            godot_warn!("⚠️ Generation is already in progress.");
            return;
        }

        self.is_generating = true;
        
        // Send a start signal immediately to Godot
        self.sync.add_signal(EngineMessage::Start);
        self.sync.add_signal(EngineMessage::Status("Map generation started".to_string()));
        
        godot_print!("🗺️ Starting build_custom_map: {}x{}", width, height);
        
        // This is the core change: we move the generation to a separate thread.
        let sync_clone = self.sync.clone();
        
        spawn(move || {
            let mut rng = StdRng::seed_from_u64(seed);
            let start = std::time::Instant::now();
            let total_tiles = (width as u64) * (height as u64);
            let mut placed = 0;

            for y in 0..height {
                for x in 0..width {
                    let pos = Vector2i::new(x, y);
                    let atlas = match mode.as_str() {
                        "checkerboard" => if (x + y) % 2 == 0 { black } else { blue },
                        "clustered" => if rng.gen_bool(0.7) { black } else { blue },
                        "noise" => if rng.gen_bool(0.5) { black } else { blue },
                        _ => if rng.gen_bool(0.5) { black } else { blue },
                    };

                    let tile_msg = TilePlacementMessage {
                        position: pos,
                        info: TileInfo {
                            source_id: 0,
                            atlas_coords: atlas,
                            alternate_id: 0,
                        },
                    };
                    
                    sync_clone.add_tile(tile_msg);
                    placed += 1;
                    
                    // Report progress every 10%
                    if placed % (total_tiles / 10) == 0 {
                        let percent = (placed * 100) / total_tiles;
                        sync_clone.add_signal(EngineMessage::Progress(percent as i32));
                    }
                }
            }
            
            let duration = start.elapsed().as_secs_f64();
            sync_clone.add_signal(EngineMessage::Status("Terrain generation complete".to_string()));
            sync_clone.add_signal(EngineMessage::Complete {
                width,
                height,
                mode,
                animate,
                duration,
            });
            godot_print!("✅ Generation finished in {:.2}s", duration);
        });
    }

    // Add back your helper functions
    #[func]
    pub fn set_signals_node(&mut self, signals: Gd<AetherionSignals>) {
        self.signals_node = Some(signals);
        godot_print!("🔗 Signals node connected.");
    }
    // ... all your other helper and debug functions ...
}