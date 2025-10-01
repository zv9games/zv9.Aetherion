use godot::prelude::*;
use std::sync::{Arc, Mutex};

use crate::zv9_prelude::*;
use aetherion_core::pipeline::builder::ChunkDelivery;
use aetherion_core::pipeline::data::MapDataChunk;
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::SyncBridge;

/// 🧵 GodotSync — thread-safe queue for chunk and signal delivery between Rust and Godot.
#[derive(Clone, Default)]
pub struct GodotSync {
    inner: Arc<Mutex<GodotSyncInner>>,
}

#[derive(Default)]
struct GodotSyncInner {
    chunks: Vec<MapDataChunk>,
    signals: Vec<EngineMessage>,
}

impl GodotSync {
    /// Initializes a new sync queue.
    pub fn init() -> Self {
        Self {
            inner: Arc::new(Mutex::new(GodotSyncInner::default())),
        }
    }

    /// Queues a chunk of tile data.
    pub fn add_chunk(&self, chunk: MapDataChunk) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.chunks.push(chunk);
        } else {
            godot_warn!("⚠️ GodotSync: Failed to acquire lock in add_chunk");
        }
    }

    /// Retrieves and clears all queued chunks.
    pub fn drain_chunks(&self) -> Vec<MapDataChunk> {
        if let Ok(mut inner) = self.inner.lock() {
            inner.chunks.drain(..).collect()
        } else {
            godot_warn!("⚠️ GodotSync: Failed to acquire lock in drain_chunks");
            Vec::new()
        }
    }

    /// Queues a signal message.
    pub fn add_signal(&self, signal: EngineMessage) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.signals.push(signal);
        } else {
            godot_warn!("⚠️ GodotSync: Failed to acquire lock in add_signal");
        }
    }

    /// Retrieves and clears all queued signals.
    pub fn drain_signals(&self) -> Vec<EngineMessage> {
        if let Ok(mut inner) = self.inner.lock() {
            inner.signals.drain(..).collect()
        } else {
            godot_warn!("⚠️ GodotSync: Failed to acquire lock in drain_signals");
            Vec::new()
        }
    }

    /// Checks if any chunks or signals are pending.
    pub fn has_pending(&self) -> bool {
        if let Ok(inner) = self.inner.lock() {
            !inner.chunks.is_empty() || !inner.signals.is_empty()
        } else {
            false
        }
    }
}

/// 🚀 GodotDelivery — wrapper for GodotSync that satisfies ChunkDelivery trait.
pub struct GodotDelivery {
    pub sync: GodotSync,
    pub bridge: SyncBridge,
}

impl Clone for GodotDelivery {
    fn clone(&self) -> Self {
        Self {
            sync: self.sync.clone(),
            bridge: SyncBridge::default(), // fresh bridge for new thread
        }
    }
}

impl ChunkDelivery for GodotDelivery {
    fn deliver(&mut self, chunk: MapDataChunk) {
        self.sync.add_chunk(chunk);
    }

    fn sync(&mut self) -> &mut SyncBridge {
        &mut self.bridge
    }
}
