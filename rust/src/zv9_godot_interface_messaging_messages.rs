/// ✅ Suggestions for godot4/messaging/messages.rs

// 🔧 Add serialization support:
//     - Derive `Serialize` and `Deserialize` if needed for logging, networking, or persistence
//     - Enables external tooling or replay systems

// 🧩 Add message introspection:
//     - `fn kind(&self) -> &'static str` or `fn is_terminal(&self) -> bool`
//     - Useful for filtering, diagnostics, or UI feedback

// 🚦 Add message prioritization or tagging:
//     - e.g. `priority: u8` or `category: MessageType`
//     - Enables smarter dispatch and queue management

// 📚 Document signal mapping:
//     - Clarify which `EngineMessage` maps to which Godot signal
//     - Could include a table or comment block for reference

// 🧪 Add unit tests for message construction and matching:
//     - Validate `Complete`, `Diagnostics`, and `Custom` payloads
//     - Ensure all variants behave correctly in signal dispatch

// 🧼 Optional: Add helper constructors:
//     - e.g. `EngineMessage::progress(percent: i32)`
//     - Improves readability and reduces boilerplate

// 🚀 Future: Add support for batched or grouped messages:
//     - e.g. `EngineMessage::Batch(Vec<EngineMessage>)`
//     - Useful for reducing signal overhead or syncing large updates

// 🧠 Consider exposing message metadata:
//     - e.g. timestamp, origin thread, or context ID
//     - Enables profiling, debugging, or distributed tracing


use crate::zv9_prelude::*;

/// 📨 EngineMessage — messages sent from Rust to Godot for signal dispatch.
/// These drive procedural feedback, status updates, and runtime events.
#[derive(Clone, Debug)]
pub enum EngineMessage {
    // ✅ Lifecycle
    Start,
    Cancelled,
    Complete {
        width: i32,
        height: i32,
        mode: String,
        animate: bool,
        duration: f64,
    },

    // 📊 Feedback
    Progress(i32),
    Status(String),
    Warning(String),
    Error(String),

    // 🧩 Chunk Delivery
    MapChunkReady,
    ChunkReady(MapDataChunk),

    // 🧠 Runtime Signals
    Paused,
    Resumed,
    Retry,
    Diagnostics {
        memory_usage: u64,
        thread_count: usize,
        tick_rate: f32,
    },

    // 🧪 Custom Event
    Custom {
        name: String,
        payload: serde_json::Value,
    },
}
