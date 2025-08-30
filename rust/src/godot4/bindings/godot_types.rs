use crate::SerializableVector2i;
use godot_core::builtin::Vector2i;

impl From<SerializableVector2i> for Vector2i {
    fn from(value: SerializableVector2i) -> Self {
        Vector2i::new(value.x, value.y)
    }
}