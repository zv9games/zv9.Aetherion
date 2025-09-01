use std::sync::{Arc, Mutex};
use crate::godot4::messaging::messages::EngineMessage;
use crate::aetherion::pipeline::data::MapDataChunk;
use godot::prelude::*;

/// Thread-safe queue system for communicating between Rust threads and Godot.
/// Stores tilemap chunks and signal messages to be drained in `_process()`.
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
    /// Creates a new sync queue.
    pub fn init() -> Self {
        Self {
            inner: Arc::new(Mutex::new(GodotSyncInner::default())),
        }
    }

    /// Adds a chunk of tile data to the queue.
    pub fn add_chunk(&self, chunk: MapDataChunk) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.chunks.push(chunk);
        } else {
            godot_warn!("Failed to acquire lock in add_chunk");
        }
    }

    /// Drains all tile data chunks from the queue.
    pub fn drain_chunks(&self) -> Vec<MapDataChunk> {
        if let Ok(mut inner) = self.inner.lock() {
            inner.chunks.drain(..).collect()
        } else {
            godot_warn!("Failed to acquire lock in drain_chunks");
            Vec::new()
        }
    }

    /// Adds a signal message to the queue.
    pub fn add_signal(&self, signal: EngineMessage) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.signals.push(signal);
        } else {
            godot_warn!("Failed to acquire lock in add_signal");
        }
    }

    /// Drains all signal messages from the queue.
    pub fn drain_signals(&self) -> Vec<EngineMessage> {
        if let Ok(mut inner) = self.inner.lock() {
            inner.signals.drain(..).collect()
        } else {
            godot_warn!("Failed to acquire lock in drain_signals");
            Vec::new()
        }
    }

    /// Returns true if there are any pending chunks or signals.
    pub fn has_pending(&self) -> bool {
        if let Ok(inner) = self.inner.lock() {
            !inner.chunks.is_empty() || !inner.signals.is_empty()
        } else {
            false
        }
    }
}
