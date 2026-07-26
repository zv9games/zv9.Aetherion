//! AetherionEngine — The Godot-Rust Bridge
//!
//! Seamlessly connects Godot's single-threaded loop to Aetherion's async generation core.
//! MPSC polling. Signal emission. Chunk streaming. For the hopeless wanderers who build in 60 FPS.

use godot::prelude::*;
use godot::classes::{Node, TileMap};
use godot::obj::{Base, Gd};
use godot::builtin::{GString, Dictionary, Array, Variant};

use tokio::sync::mpsc;

use aetherion_generate::{Conductor, GeneratorConfig};
use aetherion_generate::conductor::GenerationMessage;
use aetherion_math::Vec2i;
use aetherion_shared::chunk_data::ChunkData;
use crate::aetherion_signals::AetherionSignals;

use std::sync::{Arc, Mutex};
use tracing::{info, error};

// ── Godot Class Definition ──────────────────────────────────────────────────
#[derive(GodotClass)]
#[class(tool, base = Node)]
pub struct AetherionEngine {
    conductor: Option<Arc<Mutex<Conductor>>>,
    signals_node: Option<Gd<Node>>,
    tilemap_node: Option<Gd<TileMap>>,
    generation_receiver: Option<mpsc::Receiver<GenerationMessage>>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        Self {
            conductor: None,
            signals_node: None,
            tilemap_node: None,
            generation_receiver: None,
            base,
        }
    }
}

// ── Core Helpers ────────────────────────────────────────────────────────────
impl AetherionEngine {
    /// Converts `ChunkData` → Godot `Dictionary` with full tile metadata.
    fn chunk_to_dict(data: ChunkData, coords: Vec2i, z: i32) -> Dictionary {
        let mut dict = Dictionary::new();
        let mut tiles = Array::new();

        const SIZE: i32 = ChunkData::SIZE as i32;

        for (i, tile) in data.tiles.into_iter().enumerate() {
            let i = i as i32;
            let x = i % SIZE;
            let y = i / SIZE;

            let mut tile_dict = Dictionary::new();
            tile_dict.set("id", tile.tile_type as i32);
            tile_dict.set("level", tile.noise_value);
            tile_dict.set("local_x", x);
            tile_dict.set("local_y", y);
            tiles.push(&tile_dict.to_variant());
        }

        dict.set("key_x", coords.x);
        dict.set("key_y", coords.y);
        dict.set("key_z", z);
        dict.set("tile_count", tiles.len() as i32);
        dict.set("tiles", tiles.to_variant());
        dict
    }

    /// Emits status to Godot.
    fn emit_status(&mut self, msg: &str) {
        self.base_mut().emit_signal("status_updated", &[GString::from(msg).to_variant()]);
    }

    /// Lazy-init Conductor + receiver.
    fn ensure_conductor(&mut self) -> bool {
        if self.conductor.is_some() {
            return true;
        }

        info!("Initializing Conductor from Godot...");

        match Conductor::new(None) {
            Ok((cond, _state, rx)) => {
                self.conductor = Some(Arc::new(Mutex::new(cond)));
                self.generation_receiver = Some(rx);
                info!("Conductor ready.");
                true
            }
            Err(e) => {
                error!("Conductor init failed: {:?}", e);
                false
            }
        }
    }
}

// ── Godot API (#[func]) ────────────────────────────────────────────────────
#[godot_api]
impl AetherionEngine {
    #[signal]
    fn status_updated(status_message: GString);

    #[func]
    fn set_signals_node(&mut self, node: Gd<Node>) {
        self.signals_node = Some(node);
        info!("Signals node linked.");
    }

    #[func]
    fn set_tilemap(&mut self, node: Gd<TileMap>) {
        self.tilemap_node = Some(node);
        info!("TileMap linked.");
    }

    #[func]
    fn get_status(&self) -> GString {
        if self.conductor.is_some() {
            "Conductor Ready".into()
        } else {
            "Initializing...".into()
        }
    }

    // ── Async Message Polling (Main Tick) ───────────────────────────────────
    #[func]
    pub fn tick(&mut self, _tick: u64) {
        let Some(mut rx) = self.generation_receiver.take() else { return };

        let mut messages = Vec::new();
        let mut disconnected = false;

        while let Ok(msg) = rx.try_recv() {
            messages.push(msg);
        }
        if rx.is_closed() {
            disconnected = true;
            error!("Generation channel disconnected.");
        }

        for msg in messages {
            self.handle_message(msg);
        }

        if !disconnected {
            self.generation_receiver = Some(rx);
        }
    }

    fn handle_message(&mut self, msg: GenerationMessage) {
        let mut status = None;

        if let Some(sig_node) = &self.signals_node {
            match msg {
                GenerationMessage::StatusUpdate(s) => {
                    info!("Status: {}", s);
                    status = Some(format!("GENERATING: {}", s));
                }
                GenerationMessage::ChunkGenerated(coords, data) => {
                    info!("Chunk ready: ({}, {})", coords.x, coords.y);
                    let dict = Self::chunk_to_dict(data, coords, 0);
                    if let Ok(mut sig) = sig_node.clone().try_cast::<AetherionSignals>() {
                        sig.bind_mut().emit_chunk_data_ready(dict);
                    }
                }
                GenerationMessage::GenerationComplete => {
                    info!("Map generation complete.");
                    status = Some("IDLE: Ready".to_string());
                    if let Ok(mut sig) = sig_node.clone().try_cast::<AetherionSignals>() {
                        sig.bind_mut().emit_build_map_complete();
                    }
                }
                GenerationMessage::Error(e) => {
                    error!("Generation error: {}", e);
                    status = Some(format!("ERROR: {}", e));
                }
            }
        }

        if let Some(s) = status {
            self.emit_status(&s);
        }
    }

    // ── Map Generation Command ─────────────────────────────────────────────
    #[func]
    fn build_map(&mut self, w: i32, h: i32, seed: GString, gen: GString) {
        info!("build_map(W={}, H={}, seed='{}', gen='{}')", w, h, seed, gen);

        if !self.ensure_conductor() {
            self.emit_status("ERROR: Init failed");
            return;
        }

        let config = GeneratorConfig {
            width: w as usize,
            height: h as usize,
            seed: seed.to_string(),
            generator_name: gen.to_string(),
        };

        let conductor = self.conductor.clone().unwrap();
        let result = conductor.lock()
            .map(|mut c| c.start_generation(config))
            .map_err(|e| format!("Lock failed: {:?}", e));

        match result {
            Ok(Ok(())) => {
                self.emit_status("GENERATING");
                if let Some(sig) = &self.signals_node {
                    if let Ok(mut s) = sig.clone().try_cast::<AetherionSignals>() {
                        s.bind_mut().emit_build_map_start();
                    }
                }
            }
            Ok(Err(e)) => self.emit_status(&format!("ERROR: {}", e)),
            Err(e) => self.emit_status(&e),
        }
    }

    // ── Sync Chunk Generation (Debug/CLI) ──────────────────────────────────
    #[func]
    fn generate_chunk(&mut self, x: i32, y: i32, z: i32) -> Dictionary {
        if !self.ensure_conductor() {
            return Dictionary::new();
        }

        let coords = Vec2i::new(x, y);
        let conductor = self.conductor.as_ref().unwrap();

        match conductor.lock() {
            Ok(mut c) => {
                let data = c.generate_single_chunk(coords);
                Self::chunk_to_dict(data, coords, z)
            }
            Err(_) => Dictionary::new(),
        }
    }

    #[func]
    fn set_generator(&mut self, id: GString) -> bool {
        if !self.ensure_conductor() { return false; }
        let id = id.to_string();
        self.conductor.as_ref().unwrap().lock()
            .map(|mut c| c.set_active_generator(&id).is_ok())
            .unwrap_or(false)
    }

    #[func]
    fn get_active_generator_id(&self) -> GString {
        let id_str = self.conductor.as_ref()
            .and_then(|c| c.lock().ok())
            .map(|c| c.get_active_generator_id())
            .unwrap_or_else(|| "ERROR".to_string());
        GString::from(id_str.as_str())
    }

    #[func]
    fn shutdown_engine(&mut self) {
        info!("Shutting down AetherionEngine...");
        self.generation_receiver = None;

        if let Some(arc) = self.conductor.take() {
            if let Ok(mutex) = Arc::try_unwrap(arc) {
                if let Ok(conductor) = mutex.into_inner() {
                    conductor.graceful_teardown();
                }
            }
        }
    }
}