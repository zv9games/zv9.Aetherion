//C:/ZV9/zv9.aetherion/rust/src/godot4/bindings/godot_types.rs


/// ✅ Suggestions for godot4/bindings/godot_types.rs

// 🔧 Add bidirectional conversion:
//     - Implement `From<Vector2i> for SerializableVector2i`
//     - Ensures seamless round-trip between Rust and Godot types

// 🧩 Add conversion for other Godot types if needed:
//     - e.g. `Vector3i`, `Rect2i`, or `Transform2D` wrappers
//     - Useful for expanding procedural support beyond 2D tiles

// 🚦 Add unit tests for conversion:
//     - Validate `SerializableVector2i → Vector2i` and vice versa
//     - Ensure values match exactly and edge cases are handled

// 📚 Document conversion rationale:
//     - Clarify why `SerializableVector2i` is needed (e.g. for hashing, serialization)
//     - Note that `Vector2i` is not `Hash`able, hence the wrapper

// 🧼 Optional: Add helper traits or macros:
//     - e.g. `impl IntoGodot for SerializableVector2i`
//     - Improves ergonomics and consistency across bindings

// 🚀 Future: Add conversion utilities for arrays or collections:
//     - e.g. `Vec<SerializableVector2i> → Array<Vector2i>`
//     - Enables bulk data transfer between Rust and GDScript

// 🧠 Consider centralizing type conversions:
//     - Create a `godot4::bindings::conversions` module for all interop logic
//     - Keeps bindings clean and maintainable



use godot::prelude::*;
use crate::aetherion::pipeline::data::SerializableVector2i;

/// Enables conversion from engine-side SerializableVector2i to Godot's native Vector2i.
impl From<SerializableVector2i> for Vector2i {
    fn from(value: SerializableVector2i) -> Self {
        Vector2i::new(value.x, value.y)
    }
}

//end godot_types.rs