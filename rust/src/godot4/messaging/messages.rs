use godot::prelude::*;

/// Messages sent from Rust threads to Godot for signal dispatch.
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
}
