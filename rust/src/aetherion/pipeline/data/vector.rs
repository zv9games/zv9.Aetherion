use godot::prelude::*;
use serde::{Deserialize, Serialize};

/// Serializable wrapper for Godot's `Vector2i`.
/// Used in map data, tile metadata, and chunk streaming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerializableVector2i {
    pub x: i32,
    pub y: i32,
}

impl From<Vector2i> for SerializableVector2i {
    fn from(v: Vector2i) -> Self {
        Self { x: v.x, y: v.y }
    }
}
