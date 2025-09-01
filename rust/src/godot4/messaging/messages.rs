use godot::prelude::*;

/// Messages sent from Rust threads to Godot for signal dispatch.
/// These drive procedural feedback, status updates, and completion signals.
#[derive(Clone, Debug)]
pub enum EngineMessage {
    /// Signals the start of a map generation process.
    Start,

    /// Reports progress as a percentage (0–100).
    Progress(i32),

    /// Sends a status message (e.g. "Building map").
    Status(String),

    /// Signals completion with metadata.
    Complete {
        width: i32,
        height: i32,
        mode: String,
        animate: bool,
        duration: f64,
    },

    // Future variants:
    // Cancelled,
    // Error(String),
    // ChunkReady(MapDataChunk),
}
