//C:/ZV9/zv9.aetherion/rust/src/godot4/messaging/sync.rs

/// ✅ Suggestions for godot4/messaging/sync.rs

// 🔧 Add non-blocking or timeout-based access:
//     - e.g. `try_drain_chunks_with_timeout(duration: Duration)`
//     - Prevents stalls in high-load scenarios or debugging loops

// 🧩 Add queue size introspection:
//     - `fn chunk_count() -> usize`, `fn signal_count() -> usize`
//     - Useful for diagnostics, pacing logic, or UI overlays

// 🚦 Improve error reporting:
//     - Include context in `godot_warn!` (e.g. which thread or operation failed)
//     - Optionally emit `EngineMessage::Warning` for lock failures

// 📚 Document threading expectations:
//     - Clarify that this is safe for multi-threaded use and drained in `_process()`
//     - Note that `Arc<Mutex<...>>` is used for interior mutability across threads

// 🧪 Add unit tests for queue behavior:
//     - Validate chunk and signal enqueue/dequeue logic
//     - Ensure thread safety and lock recovery

// 🧼 Optional: Add signal prioritization or tagging:
//     - e.g. `add_signal_with_priority(signal, priority: u8)`
//     - Enables smarter dispatch or filtering

// 🚀 Future: Add support for event batching:
//     - e.g. `drain_signals_grouped_by_type()`
//     - Useful for reducing overhead in signal-heavy pipelines

// 🧠 Consider exposing sync state snapshot:
//     - `fn snapshot() -> Dictionary` with counts and flags
//     - Enables runtime introspection or debugging


use godot::prelude::*;
use std::sync::{Arc, Mutex};

use crate::zv9_prelude::*;


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

//end sync.rs