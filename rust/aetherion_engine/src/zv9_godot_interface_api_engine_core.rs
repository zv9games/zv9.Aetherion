// zv9_godot_interface_api_engine_core.rs

use godot::prelude::*;
use crate::zv9_godot_interface_api_engine_signals::emit_pending_signals;
use crate::zv9_godot_interface_api_engine_util::*; // optional

use crate::zv9_godot_interface_messaging_sync::{GodotDelivery, GodotSync};
use aetherion_core::pipeline::builder::spawn_builder_thread;
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
            current_status: "Awaiting Oracle".into(),
            conductor: Some(Conductor::new(streamer)),
            chunk: Some(MapDataChunk::new()),
        }
    }
}

#[godot_api]
impl AetherionEngine {
    #[func]
    fn _ready(&mut self) {
        godot_print!("⚙️ AetherionEngine online. Systems nominal.");
        self.base.to_init_gd().set_process(true);
    }

    #[func]
    fn process(&mut self, _delta: f64) {
        let drained = self.sync.drain_signals();
        for signal_msg in drained {
            if let EngineMessage::Status(status) = signal_msg {
                self.current_status = status;
            }
        }
        emit_pending_signals(self);
    }

    #[func]
    pub fn tick(&mut self, tick: u64) {
        self.sync.push_status("Idle");
        if let (Some(conductor), Some(chunk)) = (self.conductor.as_mut(), self.chunk.as_mut()) {
            conductor.tick(tick, chunk);
        }
        self.process(0.0);
    }

    #[func]
    pub fn build_map(&mut self, width: i32, height: i32, seed: i64, mode: String, animate: bool, black: Vector2i, blue: Vector2i) {
        let config = build_map_config(width, height, seed, mode, animate, black, blue);
        if let Some(conductor) = self.conductor.as_mut() {
            let delivery = conductor.streamer_mut().delivery_mut();
            spawn_builder_thread(delivery.clone(), config);
        }
    }

    #[func]
    pub fn get_status(&self) -> String {
        self.current_status.clone()
    }

    #[func]
    pub fn ping(&self) {
        godot_print!("⚙️ Engine: Ping received.");
    }

    #[func]
    pub fn set_signals_node(&mut self, node: Gd<AetherionSignals>) {
        self.signals_node = Some(node);
    }
}
