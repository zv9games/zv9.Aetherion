use godot::prelude::*;
use godot::meta::{GodotConvert, ToGodot, FromGodot};
use godot::meta::error::ConvertError;
use serde::{Deserialize, Serialize};

/// A serde-compatible wrapper for Godot's `Vector2i`.
/// Used for serialization, hashing, and cross-language compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerializableVector2i {
    pub x: i32,
    pub y: i32,
}

// Trait to define how this type converts to/from Godot's native Vector2i
impl GodotConvert for SerializableVector2i {
    type Via = Vector2i;
}

impl ToGodot for SerializableVector2i {
    type ToVia<'v> = Vector2i;

    fn to_godot(&self) -> Self::ToVia<'_> {
        Vector2i::new(self.x, self.y)
    }
}

impl FromGodot for SerializableVector2i {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(Self { x: via.x, y: via.y })
    }
}
