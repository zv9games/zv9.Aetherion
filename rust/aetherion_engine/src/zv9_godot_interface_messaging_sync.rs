use godot::prelude::*;
use std::sync::{Arc, Mutex};

use crate::zv9_prelude::*;
use aetherion_core::pipeline::builder::ChunkDelivery;
use aetherion_core::pipeline::data::MapDataChunk;
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::SyncBridge;

/// 🧵 GodotSync — thread-safe queue for chunk and signal delivery between Rust and Godot.
#[derive(Clone)]
pub struct GodotSync {
    inner: Arc<Mutex<GodotSyncInner>>,
    id: usize,
}

#[derive(Default)]
struct GodotSyncInner {
    chunks: Vec<MapDataChunk>,
    signals: Vec<EngineMessage>,
}

impl Default for GodotSync {
    fn default() -> Self {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        godot_print!("🧵 GodotSync[{}] initialized.", id);
        Self {
            inner: Arc::new(Mutex::new(GodotSyncInner::default())),
            id,
        }
    }
}

impl GodotSync {
    pub fn init() -> Self {
        Self::default()
    }

    // 🔍 Debug: Print sync ID for instance tracing (e.g., detect clones/splits)
    pub fn debug_id(&self) {
        godot_print!("🧵 GodotSync[{}]", self.id);
    }

    pub fn add_chunk(&self, chunk: MapDataChunk) {
        match self.inner.lock() {
            Ok(mut inner) => {
                godot_print!("📦 GodotSync[{}] → add_chunk({} tiles)", self.id, chunk.tiles.len());
                inner.chunks.push(chunk);
            }
            Err(_) => godot_warn!("⚠️ GodotSync[{}]: Failed to acquire lock in add_chunk", self.id),
        }
    }

    pub fn drain_chunks(&self) -> Vec<MapDataChunk> {
        match self.inner.lock() {
            Ok(mut inner) => {
                let count = inner.chunks.len();
                godot_print!("📦 GodotSync[{}] → drain_chunks({} chunks)", self.id, count);
                inner.chunks.drain(..).collect()
            }
            Err(_) => {
                godot_warn!("⚠️ GodotSync[{}]: Failed to acquire lock in drain_chunks", self.id);
                Vec::new()
            }
        }
    }

    pub fn add_signal(&self, signal: EngineMessage) {
        match self.inner.lock() {
            Ok(mut inner) => {
                godot_print!("📡 GodotSync[{}] → add_signal({:?})", self.id, signal);
                inner.signals.push(signal);
            }
            Err(_) => godot_warn!("⚠️ GodotSync[{}]: Failed to acquire lock in add_signal", self.id),
        }
    }

    pub fn drain_signals(&self) -> Vec<EngineMessage> {
        match self.inner.lock() {
            Ok(mut inner) => {
                let count = inner.signals.len();
                godot_print!("📡 GodotSync[{}] → drain_signals({} signals)", self.id, count);
                inner.signals.drain(..).collect()
            }
            Err(_) => {
                godot_warn!("⚠️ GodotSync[{}]: Failed to acquire lock in drain_signals", self.id);
                Vec::new()
            }
        }
    }

    pub fn has_pending(&self) -> bool {
        match self.inner.lock() {
            Ok(inner) => {
                let pending = !inner.chunks.is_empty() || !inner.signals.is_empty();
                godot_print!("🔍 GodotSync[{}] → has_pending() = {}", self.id, pending);
                pending
            }
            Err(_) => {
                godot_warn!("⚠️ GodotSync[{}]: Failed to acquire lock in has_pending", self.id);
                false
            }
        }
    }

    pub fn push_status(&self, status: &str) {
        godot_print!("📡 GodotSync[{}] → push_status({})", self.id, status);
        self.add_signal(EngineMessage::Status(status.to_string()));
    }
}

/// 🚀 GodotDelivery — wrapper for GodotSync that satisfies ChunkDelivery trait.
pub struct GodotDelivery {
    pub sync: GodotSync,
    pub bridge: SyncBridge,
}

impl Clone for GodotDelivery {
    fn clone(&self) -> Self {
        godot_print!("🔁 GodotDelivery cloned (Sync ID: {})", self.sync.id);
        Self {
            sync: self.sync.clone(),
            bridge: SyncBridge::default(),
        }
    }
}

impl GodotDelivery {
    pub fn sync_mut(&mut self) -> &mut GodotSync {
        godot_print!("🔧 GodotDelivery → sync_mut() (ID: {})", self.sync.id);
        &mut self.sync
    }

    pub fn push_status(&self, status: &str) {
        godot_print!("📡 GodotDelivery → push_status({}) (Sync ID: {})", status, self.sync.id);
        self.sync.push_status(status);
    }
}

impl ChunkDelivery for GodotDelivery {
    fn deliver(&mut self, chunk: MapDataChunk) {
        godot_print!("📦 GodotDelivery → deliver({} tiles) (Sync ID: {})", chunk.tiles.len(), self.sync.id);
        self.sync.add_chunk(chunk);
    }

    fn sync(&mut self) -> &mut SyncBridge {
        godot_print!("🔧 GodotDelivery → sync() (ID: {})", self.sync.id);
        &mut self.bridge
    }
}