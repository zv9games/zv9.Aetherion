use godot::prelude::*;
use godot::classes::TileMap;
use godot::global::Error;

use crate::godot4::api::AetherionSignals;
use crate::godot4::messaging::{GodotSync, EngineMessage};
use crate::aetherion::pipeline::data::{MapBuildOptions, MapDataChunk};
use crate::aetherion::pipeline::builder::threaded::spawn_builder_thread;
use crate::aetherion::core::conductor::Conductor;

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

    current_status: String,

    conductor: Option<Conductor>,
    chunk: Option<MapDataChunk>,
}





#[godot_api]
impl AetherionEngine {
	#[allow(dead_code)]
    fn init(base: Base<Node>) -> Self {
		let sync = GodotSync::init();
		Self {
			base,
			sync: sync.clone(),
			signals_node: None,
			target_tilemap: None,
			current_status: "Uninitialized".into(),
			conductor: Some(Conductor::new(sync)),
			chunk: Some(MapDataChunk::new()),
		}
	}

	#[allow(dead_code)]
    fn ready(&mut self) {
        godot_print!("⚙️ AetherionEngine online. Systems nominal.");
        self.base.to_init_gd().set_process(true);
    }

    fn process(&mut self, _delta: f64) {
        self.apply_chunks_to_tilemap();
        self.emit_pending_signals();
    }

    fn apply_chunks_to_tilemap(&mut self) {
        if let Some(tilemap) = self.target_tilemap.as_mut() {
            for chunk in self.sync.drain_chunks() {
                for (pos, tile_info) in chunk.tiles {
                    tilemap.set_cell_ex(0, pos.into())
                        .source_id(tile_info.source_id)
                        .atlas_coords(tile_info.atlas_coords.into())
                        .alternative_tile(tile_info.alternate_id)
                        .done();
                }
            }
        }
    }

    fn emit_pending_signals(&mut self) {
        if let Some(signals_node) = self.signals_node.as_mut() {
            for signal_msg in self.sync.drain_signals() {
                let result = match signal_msg {
                    EngineMessage::Start => signals_node.emit_signal("build_map_start", &[]),
                    EngineMessage::Progress(percent) => {
                        signals_node.emit_signal("generation_progress", &[percent.to_variant()])
                    }
                    EngineMessage::Status(status) => {
                        self.current_status = status.clone();
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
                };

                if result != Error::OK {
                    godot_warn!("⚠️ Engine: Signal emission failed: {:?}", result);
                }
            }
        }
    }

    #[func]
	pub fn tick(&mut self, tick: u64) {
		if let (Some(conductor), Some(chunk)) = (self.conductor.as_mut(), self.chunk.as_mut()) {
			godot_print!("⚙️ Engine: Tick {} received from Oracle.", tick);
			conductor.tick(tick, chunk);
			self.process(0.0);
		} else {
			godot_warn!("⚠️ Engine: Tick ignored. Conductor or chunk not initialized.");
		}
	}


    #[func]
    pub fn build_map(
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

        godot_print!("⚙️ Engine: Launching map build thread...");
        spawn_builder_thread(self.sync.clone(), config);
    }

    #[func]
    pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.target_tilemap = Some(tilemap);
        godot_print!("⚙️ Engine: TileMap target assigned.");
    }

    #[func]
    pub fn debug_place_tile(&mut self, x: i32, y: i32) {
        if let Some(tilemap) = self.target_tilemap.as_mut() {
            tilemap.set_cell_ex(0, Vector2i::new(x, y))
                .source_id(0)
                .atlas_coords(Vector2i::new(14, 13))
                .alternative_tile(0)
                .done();
            godot_print!("⚙️ Engine: Debug tile placed at ({}, {}).", x, y);
        } else {
            godot_warn!("⚠️ Engine: No TileMap assigned. Cannot place debug tile.");
        }
    }

    #[func]
    pub fn ping(&self) {
        godot_print!("⚙️ Engine: Ping received. Standing by.");
    }

    #[func]
    pub fn get_status(&self) -> String {
        self.current_status.clone()
    }
}
