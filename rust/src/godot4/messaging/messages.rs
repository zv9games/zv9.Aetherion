
/// 📨 EngineMessage — messages sent from Rust to Godot for signal dispatch.
/// These drive procedural feedback, status updates, and runtime events.
#[derive(Clone, Debug)]
pub enum EngineMessage {
    /// Signals the start of map generation.
    Start,

    /// Reports generation progress as a percentage.
    Progress(i32),

    /// Sends a status update message.
    Status(String),

    /// Signals completion of map generation with metadata.
    Complete {
        width: i32,
        height: i32,
        mode: String,
        animate: bool,
        duration: f64,
    },

    /// Signals that a new chunk is ready for placement.
    MapChunkReady,

    // 🔮 Future variants:
    // Cancelled,
    // Error(String),
    // ChunkReady(MapDataChunk),
}
