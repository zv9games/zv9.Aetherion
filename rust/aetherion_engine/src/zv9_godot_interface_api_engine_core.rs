use godot::prelude::*;
use godot::classes::TileMap;

use crate::MapDataChunk;
use crate::zv9_godot_interface_api_signals::AetherionSignals;
use crate::zv9_godot_interface_api_engine_signals::emit_pending_signals;
use crate::zv9_godot_interface_api_engine_util::*;
use crate::zv9_godot_interface_messaging_sync::{GodotDelivery, GodotSync};

use aetherion_core::shared::EngineMessage;
use aetherion_core::pipeline::builder::spawn_builder_thread;
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::{ChunkStreamer, SyncBridge};
use aetherion_core::zv9_aetherion_core_conductor::Conductor;

#[derive(GodotClass)]
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
}

impl AetherionEngine {
    #[allow(dead_code)]
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
        }
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
        self.process(0.0);
    }

    #[func]
    fn process(&mut self, _delta: f64) {
        godot_print!("🔄 AetherionEngine → process()");

        if let Some(conductor) = self.conductor.as_mut() {
            godot_print!("🔁 Forwarding bridge signals to GodotSync");
            conductor.streamer_mut().delivery_mut().forward_bridge_signals();
        }

        let drained = self.sync.drain_signals();
        godot_print!("📡 AetherionEngine → drained {} signals", drained.len());

        for signal_msg in &drained {
            godot_print!("📡 Signal received: {:?}", signal_msg);
            if let EngineMessage::Status(status) = signal_msg {
                self.current_status = status.clone();
                self.base_mut().emit_signal("status_updated", &[Variant::from(status.clone())]);


            }
        }

        emit_pending_signals(self);
    }

    #[func]
    pub fn tick(&mut self, tick: u64) {
        godot_print!("⏱️ AetherionEngine → tick({})", tick);
        self.sync.push_status("Idle");

        if let (Some(conductor), Some(chunk)) = (self.conductor.as_mut(), self.chunk.as_mut()) {
            godot_print!("🎛 Conductor tick with chunk");
            conductor.tick(tick, chunk);
        }

        self.process(0.0);
    }

    #[func]
    pub fn build_map(&mut self, width: i32, height: i32, seed: i64, mode: String, animate: bool, black: Vector2i, blue: Vector2i) {
        godot_print!("🗺️ AetherionEngine → build_map({}, {}, seed={}, mode={}, animate={})", width, height, seed, mode, animate);

        let config = build_map_config(width, height, seed, mode, animate, black, blue);

        if let Some(conductor) = self.conductor.as_mut() {
            let delivery = conductor.streamer_mut().delivery_mut();
            godot_print!("🚀 Spawning builder thread with Sync ID: {}", delivery.sync_id());
            spawn_builder_thread(delivery.clone(), config);
        }

        self.process(0.0);
    }

    #[func]
    pub fn get_status(&self) -> String {
        godot_print!("📡 AetherionEngine → get_status() = {}", self.current_status);
        self.current_status.clone()
    }

    #[func]
    pub fn ping(&self) {
        godot_print!("⚙️ Engine: Ping received.");
    }

    #[func]
    pub fn set_signals_node(&mut self, node: Gd<AetherionSignals>) {
        godot_print!("📶 AetherionEngine → set_signals_node()");
        self.signals_node = Some(node);
    }
}
