use crate::zv9_aetherion_pipeline_builder_streamer::{ChunkDelivery, SyncBridge};
use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;

/// 🧪 DummyDelivery — mock delivery backend for Rust-side testing and emulation.
#[derive(Clone)]
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

    /// Returns a dummy sync ID for testing purposes.
    fn sync_id(&self) -> usize {
        0 // Static ID for dummy backend
    }
}
