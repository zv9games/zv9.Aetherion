use std::str::FromStr;

use godot::prelude::*;
use godot::classes::TileMap;

use crate::MapDataChunk;
use crate::zv9_godot_interface_api_signals::AetherionSignals;
use crate::zv9_godot_interface_api_engine_signals::emit_pending_signals;
use crate::zv9_godot_interface_messaging_sync::{GodotDelivery, GodotSync};

use aetherion_core::pipeline::builder::spawn_map_builder;
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::{ChunkStreamer, SyncBridge};
use aetherion_core::zv9_aetherion_core_conductor::{Conductor, ProcCommand};
use aetherion_core::zv9_aetherion_generator_noise::NoiseType;
use aetherion_core::pipeline::data::SerializableVector2i;
use aetherion_shared::shared::EngineMessage;

#[derive(GodotClass, Debug)]
#[class(init, base = Node)]
pub struct AetherionEngine {
    #[base]
    pub base: Base<Node>,

    pub sync: GodotSync,
    pub signals_node: Option<Gd<AetherionSignals>>,

    #[export]
    pub target_tilemap: Option<Gd<TileMap>>,

    pub current_status: String,
    pub conductor: Option<Conductor<GodotDelivery>>,
    pub chunk: Option<MapDataChunk>,
    pub last_reported_status: Option<String>,
}

impl AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        let sync = GodotSync::init();
        let delivery = GodotDelivery {
            sync: sync.clone(),
            bridge: SyncBridge::default(),
        };
        let streamer = ChunkStreamer::new(delivery, 2);

        godot_print!("🧵 AetherionEngine initialized with Sync ID: {}", sync.sync_id());

        Self {
            base,
            sync,
            signals_node: None,
            target_tilemap: None,
            current_status: "Awaiting Oracle".into(),
            conductor: Some(Conductor::new(streamer)),
            chunk: Some(MapDataChunk::new()),
            last_reported_status: None,
        }
    }

    fn apply_chunk_to_tilemap(&mut self, chunk: &MapDataChunk) {
        let Some(tilemap) = &mut self.target_tilemap else {
            godot_print!("❌ apply_chunk_to_tilemap: target_tilemap None");
            return;
        };

        godot_print!("🧩 Applying chunk to TileMap: {} tiles", chunk.len());

        for (key, tile) in chunk.iter() {
            let pos = Vector2i::new(key.x, key.y);
            let atlas = Vector2i::new(tile.atlas_coords.x, tile.atlas_coords.y);

            tilemap
                .set_cell_ex(tile.layer as i32, pos)
                .source_id(tile.source_id)
                .atlas_coords(atlas)
                .alternative_tile(tile.alternate_id)
                .done();
        }

        tilemap.force_update();
        godot_print!("✅ TileMap updated with chunk data");
    }
}

#[godot_api]
impl AetherionEngine {
    #[signal]
    fn status_updated(status: String);

    #[func]
    fn _ready(&mut self) {
        godot_print!("⚙️ AetherionEngine online. Systems nominal.");
        self.base_mut().set_process(true);

        self.sync.push_status("🟢 Engine initialized. Awaiting map build...");
        self.last_reported_status = None;
        self.process(0.0);

        godot_print!("🧠 AetherionEngine boot sequence complete. Ready for map generation.");
    }

    #[func]
    fn process(&mut self, _delta: f64) {
        if let Some(conductor) = self.conductor.as_mut() {
            conductor.streamer_mut().delivery_mut().forward_bridge_signals();
        }

        let drained = self.sync.drain_signals();
        for signal_msg in &drained {
            match signal_msg {
                EngineMessage::Status(status) => {
                    self.current_status = status.clone();
                    self.base_mut().emit_signal("status_updated", &[Variant::from(status.clone())]);
                }
                EngineMessage::Chunk(chunk) => {
                    self.apply_chunk_to_tilemap(chunk);
                }
                _ => {
                    godot_print!("⚠️ Unhandled EngineMessage: {:?}", signal_msg);
                }
            }
        }

        emit_pending_signals(self);
    }

    #[func]
    fn tick(&mut self, tick: u64) {
        self.sync.push_status("Idle");

        if let (Some(conductor), Some(chunk)) = (self.conductor.as_mut(), self.chunk.as_mut()) {
            conductor.tick(tick, chunk);
        }

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
		godot_print!(
			"🗺️ AetherionEngine → build_map({}, {}, seed={}, mode={}, animate={})",
			width, height, seed, mode, animate
		);

		if let Some(signals) = self.signals_node.as_mut() {
			signals.emit_signal("build_map_start", &[]);
		}

		if let Some(tilemap) = &mut self.target_tilemap {
			for (x, y) in [(0, 0), (1, 0), (0, 1)] {
				tilemap
					.set_cell_ex(0, Vector2i::new(x, y))
					.source_id(0)
					.atlas_coords(Vector2i::new(x, y))
					.alternative_tile(0)
					.done();
			}
			tilemap.force_update();
		}

		let black_ser = SerializableVector2i { x: black.x, y: black.y };
		let blue_ser = SerializableVector2i { x: blue.x, y: blue.y };
	
		let config = aetherion_core::zv9_aetherion_generator_noise_config::NoiseConfig {
			width: width as usize,
			height: height as usize,
			seed: seed as u64,
			birth_limit: 4,
			survival_limit: 3,
			fill_ratio: 0.45,
			steps: 5,
		};

		let noise_type = NoiseType::from_str(&mode).unwrap_or(NoiseType::Basic);
	
		if let Some(conductor) = self.conductor.as_mut() {
			let streamer = conductor.streamer_mut();
			spawn_map_builder(streamer, config, noise_type, animate, black_ser, blue_ser);
			conductor.enqueue(ProcCommand::GenerateTerrain);
			self.sync.push_status("🛠️ Map build requested");
		} else {
			godot_print!("❌ conductor is None; cannot enqueue build");
		}

		self.last_reported_status = None;
		self.process(0.0);
	}




    #[func]
    fn get_status(&mut self) -> String {
        if self.last_reported_status.as_deref() != Some(&self.current_status) {
            self.last_reported_status = Some(self.current_status.clone());
        }
        self.current_status.clone()
    }

    #[func]
    fn ping(&self) {
        godot_print!("⚙️ Engine: Ping received.");
    }

    #[func]
    fn set_signals_node(&mut self, node: Gd<AetherionSignals>) {
        self.signals_node = Some(node);
    }

    #[func]
    fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.target_tilemap = Some(tilemap);
    }
}
