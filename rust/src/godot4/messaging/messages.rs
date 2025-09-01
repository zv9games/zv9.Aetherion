use godot::prelude::*;

/// Messages sent from Rust threads to Godot for signal dispatch.
/// These drive procedural feedback, status updates, and completion signals.
#[derive(Clone, Debug)]
pub enum EngineMessage {
    Start,
    Progress(i32),
    Status(String),
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
