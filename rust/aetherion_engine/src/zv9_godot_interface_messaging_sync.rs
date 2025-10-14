use godot::prelude::*;
use std::sync::{Arc, Mutex};

use aetherion_shared::shared::EngineMessage;
use aetherion_core::pipeline::builder::ChunkDelivery;
use aetherion_core::pipeline::data::MapDataChunk;
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::SyncBridge;

//
// ─── GodotSync ────────────────────────────────────────────────────────────────
//

/// 🧵 GodotSync — thread-safe queue for chunk and signal delivery between Rust and Godot.
#[derive(Clone, Debug)]
pub struct GodotSync {
    inner: Arc<Mutex<GodotSyncInner>>,
    id: usize,
}

#[derive(Default, Debug)]
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

    pub fn sync_id(&self) -> usize {
        self.id
    }

    pub fn debug_id(&self) {
        godot_print!("🧵 GodotSync[{}]", self.id);
    }

    pub fn add_chunk(&self, chunk: MapDataChunk) {
        match self.inner.lock() {
            Ok(mut inner) => {
                godot_print!("📦 GodotSync[{}] → add_chunk({} tiles)", self.id, chunk.tiles.len());
                inner.chunks.push(chunk);
            }
            Err(_) => {
                godot_warn!("⚠️ GodotSync[{}]: Failed to acquire lock in add_chunk", self.id);
            }
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
            Err(_) => {
                godot_warn!("⚠️ GodotSync[{}]: Failed to acquire lock in add_signal", self.id);
            }
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

//
// ─── GodotDelivery ────────────────────────────────────────────────────────────
//

/// 🚀 GodotDelivery — wrapper for GodotSync that satisfies ChunkDelivery trait.
#[derive(Clone, Debug)]
pub struct GodotDelivery {
    pub sync: GodotSync,
    pub bridge: SyncBridge,
}

impl GodotDelivery {
    pub fn sync_id(&self) -> usize {
        self.sync.sync_id()
    }

    pub fn sync_mut(&mut self) -> &mut GodotSync {
        self.forward_bridge_signals();
        godot_print!("🔧 GodotDelivery → sync_mut() (ID: {})", self.sync.sync_id());
        &mut self.sync
    }

    pub fn push_status(&self, status: &str) {
        godot_print!("📡 GodotDelivery → push_status({}) (Sync ID: {})", status, self.sync.sync_id());
        self.sync.push_status(status);
    }

    pub fn forward_bridge_signals(&mut self) {
        let bridge_signals = self.bridge.drain_signals();
        godot_print!("🔁 GodotDelivery → forwarding {} signals from bridge", bridge_signals.len());
        for signal in bridge_signals {
            godot_print!("➡️ Forwarded to GodotSync[{}]: {:?}", self.sync.sync_id(), signal);
            self.sync.add_signal(signal);
        }
    }
}

impl ChunkDelivery for GodotDelivery {
    fn deliver(&mut self, chunk: MapDataChunk) {
        godot_print!(
            "📦 GodotDelivery → deliver({} tiles) (Sync ID: {})",
            chunk.tiles.len(),
            self.sync.sync_id()
        );
        self.sync.add_chunk(chunk);
    }

    fn sync(&mut self) -> &mut SyncBridge {
        godot_print!("🔧 GodotDelivery → sync() (ID: {})", self.sync.sync_id());
        &mut self.bridge
    }

    fn sync_id(&self) -> usize {
        self.sync.sync_id()
    }
}
