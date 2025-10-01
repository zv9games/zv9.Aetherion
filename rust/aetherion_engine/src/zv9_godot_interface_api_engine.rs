use godot::prelude::*;
use godot::classes::TileMap;
use godot::global::Error;

use crate::zv9_prelude::*;
use crate::zv9_godot_interface_map_ext::MapDataChunkExt;
use crate::zv9_godot_interface_messaging_sync::{GodotDelivery, GodotSync};

use aetherion_core::log_component;
use aetherion_core::pipeline::builder::spawn_builder_thread;
use aetherion_core::zv9_aetherion_core_conductor::Conductor;
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::SyncBridge;

/// 🚀 AetherionEngine — Godot-facing engine node for procedural generation and signal dispatch.
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
    conductor: Option<Conductor<GodotDelivery>>,
    chunk: Option<MapDataChunk>,
}

#[godot_api]
impl AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        let sync = GodotSync::init();
        let delivery = GodotDelivery {
            sync: sync.clone(),
            bridge: SyncBridge::default(),
        };
        let streamer = ChunkStreamer::new(delivery, 2);

        Self {
            base,
            sync,
            signals_node: None,
            target_tilemap: None,
            current_status: "Uninitialized".into(),
            conductor: Some(Conductor::new(streamer)),
            chunk: Some(MapDataChunk::new()),
        }
    }

    fn ready(&mut self) {
        godot_print!("⚙️ AetherionEngine online. Systems nominal.");
        log_component!("AetherionEngine", "Engine node for procedural generation and signal dispatch");
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
                    let pos_vec = Vector2i::new(pos.x, pos.y);
                    let atlas_vec = Vector2i::new(tile_info.atlas_coords.x, tile_info.atlas_coords.y);

                    tilemap.set_cell_ex(0, pos_vec)
                        .source_id(tile_info.source_id)
                        .atlas_coords(atlas_vec)
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
                    EngineMessage::Progress(percent) => signals_node.emit_signal("generation_progress", &[percent.to_variant()]),
                    EngineMessage::Status(status) => {
                        self.current_status = status.clone();
                        signals_node.emit_signal("map_building_status", &[GString::from(status).to_variant()])
                    }
                    EngineMessage::Complete { width, height, mode, animate, duration } => {
                        let mut dict = Dictionary::new();
                        dict.insert("width", width);
                        dict.insert("height", height);
                        dict.insert("mode", mode);
                        dict.insert("animate", animate);
                        dict.insert("duration", duration);
                        signals_node.emit_signal("generation_complete", &[dict.to_variant()])
                    }
                    EngineMessage::MapChunkReady => signals_node.emit_signal("map_chunk_ready", &[]),
                    EngineMessage::ChunkReady(chunk) => {
                        let dict = chunk.to_dictionary();
                        signals_node.emit_signal("chunk_ready", &[dict.to_variant()])
                    }
                    EngineMessage::Cancelled => signals_node.emit_signal("map_build_cancelled", &[]),
                    EngineMessage::Diagnostics { memory_usage, thread_count, tick_rate } => {
                        signals_node.emit_signal("diagnostics", &[
                            memory_usage.to_variant(),
                            (thread_count as i32).to_variant(),
                            tick_rate.to_variant(),
                        ])
                    }
                    EngineMessage::Error(msg) => signals_node.emit_signal("rust_error", &[GString::from(msg).to_variant()]),
                    EngineMessage::Warning(msg) => signals_node.emit_signal("rust_warning", &[GString::from(msg).to_variant()]),
                    EngineMessage::Custom { name, payload } => signals_node.emit_signal("custom_event", &[
                        GString::from(name).to_variant(),
                        json_to_variant(payload),
                    ]),
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
        let mode_enum = mode.parse::<ExternalNoiseType>().unwrap_or(ExternalNoiseType::CellularAutomata);

        let config = MapBuildOptions {
            width,
            height,
            seed: seed.try_into().unwrap_or_default(),
            mode: mode_enum,
            animate,
            black: SerializableVector2i { x: black.x, y: black.y },
            blue: SerializableVector2i { x: blue.x, y: blue.y },
            birth_limit: 4,
            survival_limit: 3,
            fill_ratio: 0.45,
            steps: 5,
            delivery_interval_ms: Some(2),
        };

        godot_print!("⚙️ Engine: Launching map build thread...");

        if let Some(conductor) = self.conductor.as_mut() {
            let delivery = conductor.streamer_mut().delivery_mut();
            spawn_builder_thread(delivery.clone(), config);
        } else {
            godot_warn!("⚠️ Engine: Cannot build map. Conductor not initialized.");
        }
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
