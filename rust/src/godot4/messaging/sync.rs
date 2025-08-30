use std::sync::{Arc, Mutex};
use crate::godot4::messaging::messages::EngineMessage;
use crate::aetherion::pipeline::data::MapDataChunk;

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
        let mut inner = self.inner.lock().expect("Lock poisoned on add_chunk");
        inner.chunks.push(chunk);
    }

    /// Drains all tile data chunks from the queue.
    pub fn drain_chunks(&self) -> Vec<MapDataChunk> {
        let mut inner = self.inner.lock().expect("Lock poisoned on drain_chunks");
        inner.chunks.drain(..).collect()
    }

    /// Adds a signal message to the queue.
    pub fn add_signal(&self, signal: EngineMessage) {
        let mut inner = self.inner.lock().expect("Lock poisoned on add_signal");
        inner.signals.push(signal);
    }

    /// Drains all signal messages from the queue.
    pub fn drain_signals(&self) -> Vec<EngineMessage> {
        let mut inner = self.inner.lock().expect("Lock poisoned on drain_signals");
        inner.signals.drain(..).collect()
    }
}
