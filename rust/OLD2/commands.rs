use godot::prelude::*;
use godot::classes::{TileMap, TileSetAtlasSource};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use std::sync::{Arc, Mutex};
use crate::godot_bridge::godotsync::{GodotSync, EngineMessage, TilePlacementMessage, TileInfo};
use crate::godot_bridge::signals::AetherionSignals;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct AetherionEngine {
    #[base]
    base: Base<Node>,
    tilemap: Option<Gd<TileMap>>,
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
        
        let mut base_gd = self.base.to_gd();
        
        // This is now handled by Godot's main loop.
        // Calling set_process(true) on a Node ensures the `_process` function is called every frame.
        base_gd.set_process(true); 

        // Let's remove the timer logic here. The `_process` loop is the standard and correct way.
        // The original code was using a timer to trigger a process function, which is
        // not how Godot's `_process` is designed to be used. The `set_process(true)` call above
        // handles this automatically.
    }

    #[func]
    fn _process(&mut self, _delta: f64) {
        godot_print!("🔄 AetherionEngine _process called, is_generating = {}", self.is_generating);
        if !self.is_generating {
            return;
        }

        let Some(tilemap) = self.tilemap.as_ref() else {
            godot_error!("⚠️ TileMap not set.");
            self.is_generating = false;
            return;
        };

        if !tilemap.is_inside_tree() || tilemap.get_tileset().is_none() {
            godot_error!("⚠️ TileMap invalid: not in tree or no TileSet.");
            self.is_generating = false;
            return;
        }

        let tiles = self.sync.drain_tiles();
        godot_print!("🛠️ Drained {} tiles from sync", tiles.len());
        let tiles_per_frame = 5000;
        for tile_msg in tiles.iter().take(tiles_per_frame) {
            let cell_args = &[
                Variant::from(0),
                Variant::from(tile_msg.position),
                Variant::from(tile_msg.info.source_id),
                Variant::from(tile_msg.info.atlas_coords),
                Variant::from(tile_msg.info.alternate_id),
            ];
            let mut tilemap_mut = tilemap.clone();
            let result = tilemap_mut.call_deferred("set_cell", cell_args);
            godot_print!("✅ Tile queued at x: {}, y: {} with args {:?}, result: {:?}", 
                tile_msg.position.x, tile_msg.position.y, cell_args, result);
            tilemap_mut.call_deferred("force_update", &[]);
            godot_print!("🛠️ Forced TileMap update after set_cell");
            let used_rect = tilemap.get_used_rect();
            godot_print!("🛠️ TileMap used rect after set_cell: {:?}", used_rect);
        }

        if tiles.len() > tiles_per_frame {
            godot_print!("🛠️ Re-adding {} remaining tiles to sync", tiles.len() - tiles_per_frame);
            self.sync.add_tiles(tiles.into_iter().skip(tiles_per_frame).collect());
        } else if tiles.is_empty() {
            godot_print!("🛠️ No more tiles, checking signals");
            let signals = self.sync.drain_signals();
            godot_print!("🛠️ Drained {} signals from sync", signals.len());
            if signals.is_empty() {
                godot_print!("🛠️ No more tiles or signals, stopping generation");
                let mut tilemap_mut = tilemap.clone();
                tilemap_mut.call_deferred("force_update", &[]);
                self.is_generating = false;
            }
        }

        let signals = self.sync.drain_signals();
        godot_print!("🛠️ Drained {} signals from sync", signals.len());
        if let Some(signals_node) = self.signals_node.as_ref() {
            let mut signals_node_mut = signals_node.clone();
            for msg in signals {
                match msg {
                    EngineMessage::Progress(percent) => {
                        let result = signals_node_mut.call_deferred("emit_generation_progress", &[Variant::from(percent)]);
                        godot_print!("🛠️ Emitted progress signal: {}%, result: {:?}", percent, result);
                    }
                    EngineMessage::Status(status_message) => {
                        let result = signals_node_mut.call_deferred("emit_map_building_status", &[Variant::from(status_message.clone())]);
                        godot_print!("🛠️ Emitted status signal: {}, result: {:?}", status_message, result);
                    }
                    EngineMessage::Complete { width, height, mode, animate, duration } => {
                        let mut dict = Dictionary::new();
                        dict.insert("width", width);
                        dict.insert("height", height);
                        dict.insert("mode", mode);
                        dict.insert("animate", animate);
                        dict.insert("duration", duration);
                        let result = signals_node_mut.call_deferred("emit_generation_complete", &[dict.to_variant()]);
                        godot_print!("✅ Emitted complete signal in {:.2}s, result: {:?}", duration, result);
                    }
                }
            }
        }
    }

    #[func]
    pub fn set_signals_node(&mut self, signals: Gd<AetherionSignals>) {
        self.signals_node = Some(signals);
        godot_print!("🔗 Signals node connected.");
    }

    #[func]
    pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.tilemap = Some(tilemap);
        godot_print!("🗺️ TileMap set in AetherionEngine.");
    }

    #[func]
    pub fn build_custom_map(
        &mut self,
        tilemap: Gd<TileMap>,
        width: i32,
        height: i32,
        seed: u64,
        black: Vector2i,
        blue: Vector2i,
        animate: bool,
        mode: String,
    ) {
        self.tilemap = Some(tilemap);
        self.is_generating = true;
        let sync = &self.sync;
        let mode = mode.clone();

        let Some(signals_node) = self.signals_node.as_ref() else {
            godot_error!("⚠️ Signals node missing.");
            self.is_generating = false;
            return;
        };
        let mut signals = signals_node.clone();

        signals.call_deferred("emit_map_building_status", &[Variant::from("Map generation started")]);
        signals.call_deferred("emit_build_map_start", &[]);
        godot_print!("🗺️ Starting build_custom_map: {}x{}", width, height);

        let mut rng = StdRng::seed_from_u64(seed);
        let start = std::time::Instant::now();
        let total_tiles = (width as u64) * (height as u64);
        let batch_size = 10_000;
        let mut tiles = Vec::with_capacity(batch_size as usize);
        let mut placed = 0;

        for y in 0..height {
            for x in 0..width {
                let pos = Vector2i::new(x, y);
                let atlas = match mode.as_str() {
                    "checkerboard" => {
                        if (x + y) % 2 == 0 { black } else { blue }
                    }
                    "clustered" => {
                        if rng.gen_bool(0.7) { black } else { blue }
                    }
                    "noise" => {
                        if rng.gen_bool(0.5) { black } else { blue }
                    }
                    _ => {
                        godot_warn!("⚠️ Unknown mode '{}', defaulting to noise.", mode);
                        if rng.gen_bool(0.5) { black } else { blue }
                    }
                };

                tiles.push(TilePlacementMessage {
                    position: pos,
                    info: TileInfo {
                        source_id: 0,
                        atlas_coords: atlas,
                        alternate_id: 0,
                    },
                });

                placed += 1;
                if placed % batch_size == 0 || placed == total_tiles {
                    let percent = (placed * 100) / total_tiles;
                    sync.add_signal(EngineMessage::Progress(percent as i32));
                    sync.add_tiles(tiles);
                    godot_print!("📊 Generated {} tiles, {}%", placed, percent);
                    tiles = Vec::with_capacity(batch_size as usize);
                }
            }
        }

        if !tiles.is_empty() {
            let remaining = tiles.len();
            sync.add_tiles(tiles);
            godot_print!("🛠️ Added final batch of {} tiles", remaining);
        }

        godot_print!("🛠️ Ensuring tile sync");
        let duration = start.elapsed().as_secs_f64();
        sync.add_signal(EngineMessage::Status("Terrain generation complete".to_string()));
        sync.add_signal(EngineMessage::Complete {
            width,
            height,
            mode,
            animate,
            duration,
        });
        godot_print!("✅ Generation finished: {} tiles in {:.2}s", placed, duration);
    }

    #[func]
    fn debug_tilemap(&mut self) {
        let Some(tilemap) = self.tilemap.as_ref() else {
            godot_error!("No TileMap set.");
            return;
        };
        godot_print!("TileMap in tree: {}", tilemap.is_inside_tree());
        godot_print!("TileMap visible: {}", tilemap.is_visible());
        godot_print!("TileMap tileset: {:?}", tilemap.get_tileset());
        godot_print!("TileMap used rect: {:?}", tilemap.get_used_rect());
        godot_print!("TileMap layer enabled: {}", tilemap.is_layer_enabled(0));
        godot_print!("TileMap position: {:?}", tilemap.get_global_position());
        godot_print!("TileMap scale: {:?}", tilemap.get_scale());
        godot_print!("TileMap modulate: {:?}", tilemap.get_modulate());
        if let Some(tileset) = tilemap.get_tileset() {
            godot_print!("TileSet source count: {}", tileset.get_source_count());
            godot_print!("TileSet tile size: {:?}", tileset.get_tile_size());
            if tileset.get_source_count() > 0 {
                if let Some(source) = tileset.get_source(0) {
                    godot_print!("TileSet source 0 valid: {}", source.is_instance_valid());
                    if source.is_class("TileSetAtlasSource") {
                        let atlas_source = source.cast::<TileSetAtlasSource>();
                        let black_coords = Vector2i::new(0, 0);
                        let blue_coords = Vector2i::new(1, 0);
                        godot_print!("TileSet source 0 is atlas: {}", atlas_source.is_instance_valid());
                        godot_print!("Tile (0, 0) exists: {}", atlas_source.has_tile(black_coords));
                        godot_print!("Tile (1, 0) exists: {}", atlas_source.has_tile(blue_coords));
                        if atlas_source.has_tile(black_coords) {
                            godot_print!("Tile (0, 0) data: {:?}", atlas_source.get_tile_data(black_coords, 0));
                        } else {
                            godot_error!("Tile (0, 0) not found in atlas.");
                        }
                        if atlas_source.has_tile(blue_coords) {
                            godot_print!("Tile (1, 0) data: {:?}", atlas_source.get_tile_data(blue_coords, 0));
                        } else {
                            godot_error!("Tile (1, 0) not found in atlas.");
                        }
                        let atlas_size = atlas_source.get_atlas_grid_size();
                        godot_print!("TileSet atlas grid size: {:?}", atlas_size);
                    } else {
                        godot_error!("TileSet source 0 is not a TileSetAtlasSource, found: {}", source.get_class());
                    }
                } else {
                    godot_error!("TileSet source 0 is null.");
                }
            } else {
                godot_error!("TileSet has no sources.");
            }
        }
    }

    #[func]
    fn debug_place_tile(&mut self, pos_x: i32, pos_y: i32) {
        let Some(tilemap) = self.tilemap.as_ref() else {
            godot_error!("No TileMap set.");
            return;
        };
        let mut tilemap_mut = tilemap.clone();
        let cell_args = &[
            Variant::from(0),
            Variant::from(Vector2i::new(pos_x, pos_y)),
            Variant::from(0),
            Variant::from(Vector2i::new(0, 0)),
            Variant::from(0),
        ];
        let result = tilemap_mut.call_deferred("set_cell", cell_args);
        godot_print!("🧪 Debug tile queued at x: {}, y: {}, result: {:?}", pos_x, pos_y, result);
        tilemap_mut.call_deferred("force_update", &[]);
    }

    #[func]
    fn debug_sync_tiles(&self) {
        let tiles = self.sync.drain_tiles();
        godot_print!("🛠️ Debug: Sync contains {} tiles", tiles.len());
        let mut count = 0;
        for tile in tiles.iter() {
            if count < 100 {
                godot_print!("🛠️ Tile at x: {}, y: {}", tile.position.x, tile.position.y);
                count += 1;
            } else {
                godot_print!("🛠️ Truncated debug output at 100 tiles");
                break;
            }
        }
        self.sync.add_tiles(tiles);
    }
}