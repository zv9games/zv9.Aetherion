use crate::zv9_aetherion_pipeline_builder_streamer::{ChunkDelivery, SyncBridge};
use crate::pipeline::data::MapDataChunk;

/// 🧪 Dummy delivery backend for Rust-side testing and emulation.
pub struct DummyDelivery {
    bridge: SyncBridge,
}

impl DummyDelivery {
    /// Creates a new dummy delivery instance with a fresh sync bridge.
    pub fn new() -> Self {
        Self {
            bridge: SyncBridge::new(),
        }
    }
}

impl ChunkDelivery for DummyDelivery {
    /// Logs receipt of a chunk and its tile count.
    fn deliver(&mut self, chunk: MapDataChunk) {
        println!("🧪 DummyDelivery received chunk with {} tiles", chunk.len());
    }

    /// Returns a mutable reference to the internal sync bridge.
    fn sync(&mut self) -> &mut SyncBridge {
        &mut self.bridge
    }
}
