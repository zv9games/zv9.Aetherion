//C:/ZV9/zv9.aetherion/rust/src/godot4/bindings/godot_types.rs

use godot::prelude::*;
use crate::aetherion::pipeline::data::SerializableVector2i;

/// Enables conversion from engine-side SerializableVector2i to Godot's native Vector2i.
impl From<SerializableVector2i> for Vector2i {
    fn from(value: SerializableVector2i) -> Self {
        Vector2i::new(value.x, value.y)
    }
}

//end godot_types.rs