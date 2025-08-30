use crate::utilities::concurrency::TilePlacementMessage;
use godot::prelude::*;
use crossbeam::channel::{unbounded, Sender, Receiver};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Safe data structure for completion metadata.
#[derive(Debug, Clone)]
pub struct CompletionData {
    pub width: i32,
    pub height: i32,
    pub mode: String,
    pub animate: bool,
    pub duration: f64,
}

/// Thread-safe message enum for engine communication.
#[derive(Debug, Clone)]
pub enum EngineMessage {
    Tile(TilePlacementMessage),
    Progress(i32),
    Complete(CompletionData),
}

/// Global message queue for cross-thread communication.
static ENGINE_MESSAGE_QUEUE: Lazy<Mutex<(Sender<EngineMessage>, Receiver<EngineMessage>)>> = Lazy::new(|| {
    let (tx, rx) = unbounded();
    Mutex::new((tx, rx))
});

/// Pushes a message into the engine queue.
pub fn push_engine_message(msg: EngineMessage) {
    let queue = ENGINE_MESSAGE_QUEUE.lock().unwrap();
    let _ = queue.0.send(msg);
}

/// Drains all pending messages from the queue.
pub fn drain_engine_messages() -> Vec<EngineMessage> {
    let queue = ENGINE_MESSAGE_QUEUE.lock().unwrap();
    queue.1.try_iter().collect()
}
