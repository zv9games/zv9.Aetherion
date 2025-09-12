
/// ✅ Suggestions for godot4/api/engine.rs

// 🔧 Add engine lifecycle controls:
//     - `pause()`, `resume()`, `shutdown()` to manage generation flow
//     - Useful for editor control, runtime toggling, or debugging

// 🧩 Make `build_map()` more flexible:
//     - Accept full `MapBuildOptions` as a Dictionary or custom class
//     - Enables GDScript-side presets, profiles, and dynamic config

// 🚦 Add error handling and fallback logic:
//     - Validate tilemap assignment, signal node presence, and config values
//     - Emit warnings or fallback to defaults if missing

// 📚 Document signal flow and engine responsibilities:
//     - Clarify how this node interacts with `AetherionSignals`, `TileMap`, and `Conductor`
//     - Could include a diagram or lifecycle summary in module-level docs

// 🧪 Add integration tests or GDScript examples:
//     - Validate signal emission, chunk application, and tick behavior
//     - Ensure tile placement and map generation are deterministic

// 🧼 Optional: Add engine diagnostics or status reporting:
//     - `fn describe_state() -> Dictionary`
//     - Useful for editor overlays, runtime introspection, or debugging

// 🚀 Future: Add support for multiple tilemaps or layers:
//     - e.g. terrain + structures + overlays
//     - Could expose `fn set_tilemap_layer(layer: u8, tilemap: Gd<TileMap>)`

// 🧠 Consider exposing engine hooks or callbacks:
//     - e.g. `fn on_chunk_applied(callback: Callable)`
//     - Enables plugin systems or runtime extensions


use godot::prelude::*;
use godot::classes::TileMap;
use godot::global::Error;

use crate::godot4::api::AetherionSignals;
use crate::godot4::messaging::{GodotSync, EngineMessage};
use crate::aetherion::pipeline::data::{MapBuildOptions, MapDataChunk};
use crate::aetherion::pipeline::builder::threaded::spawn_builder_thread;
use crate::aetherion::core::conductor::Conductor;
use crate::godot4::signals::dispatch::json_to_variant;


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
					// ✅ Core generation
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

					// 🔁 Chunk delivery
					EngineMessage::ChunkReady(chunk) => {
						let dict = chunk.to_dictionary();
						signals_node.emit_signal("chunk_ready", &[dict.to_variant()])
					}

					// 🧠 Lifecycle
					EngineMessage::Cancelled => signals_node.emit_signal("map_build_cancelled", &[]),

					EngineMessage::Diagnostics { memory_usage, thread_count, tick_rate } => {
						signals_node.emit_signal("diagnostics", &[
							memory_usage.to_variant(),
							(thread_count as i32).to_variant(),
							tick_rate.to_variant(),
						])
					}

					// ⚠️ Error & warning
					EngineMessage::Error(msg) => {
						signals_node.emit_signal("rust_error", &[GString::from(msg).to_variant()])
					}

					EngineMessage::Warning(msg) => {
						signals_node.emit_signal("rust_warning", &[GString::from(msg).to_variant()])
					}

					// 🧪 Custom hook
					EngineMessage::Custom { name, payload } => {
						signals_node.emit_signal("custom_event", &[
							GString::from(name).to_variant(),
							json_to_variant(payload),
						])
					}

					// 🧭 Optional: Handle Paused, Resumed, Retry if you’ve added them
					EngineMessage::Paused => signals_node.emit_signal("engine_paused", &[]),
					EngineMessage::Resumed => signals_node.emit_signal("engine_resumed", &[]),
					EngineMessage::Retry => signals_node.emit_signal("engine_retry", &[]),
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
