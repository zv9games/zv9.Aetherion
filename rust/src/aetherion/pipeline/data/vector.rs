/// ✅ Suggestions for aetherion/pipeline/data/vector.rs

// 🔧 Add arithmetic and spatial utilities:
//     - `fn offset(&self, dx: i32, dy: i32) -> Self`
//     - `fn distance_to(&self, other: &Self) -> f32`
//     - Useful for grid navigation, pathfinding, and procedural placement

// 🧩 Implement common traits:
//     - `Add`, `Sub`, `Mul`, `Neg`, etc. for ergonomic math
//     - Enables cleaner code in procedural systems and modifiers

// 🚦 Add bounds or region helpers:
//     - `fn is_within(&self, min: Self, max: Self) -> bool`
//     - Useful for chunking, selection, and spatial queries

// 📚 Document conversion expectations:
//     - Clarify that this wrapper is used for serialization and Godot interop
//     - Note that `Vector2i` is not `Hash`able, hence the wrapper

// 🧪 Add unit tests for conversion and equality:
//     - Validate `From<Vector2i>` and `to_vector2i()` round-trip
//     - Ensure `Hash`, `Eq`, and `Copy` behave as expected

// 🧼 Optional: Add display or debug formatting:
//     - `impl std::fmt::Display for SerializableVector2i`
//     - Useful for logging, diagnostics, or editor overlays

// 🚀 Future: Add support for directional constants:
//     - e.g. `const UP: Self = Self { x: 0, y: -1 };`
//     - Enables intuitive movement and grid traversal

// 🧠 Consider exposing grid-aware methods:
//     - `fn neighbors_4(&self) -> [Self; 4]`
//     - `fn neighbors_8(&self) -> [Self; 8]`
//     - Useful for cellular automata, pathfinding, and terrain logic


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

impl SerializableVector2i {
    /// Converts this wrapper into a Godot-native `Vector2i`.
    pub fn to_vector2i(&self) -> Vector2i {
        Vector2i::new(self.x, self.y)
    }
}
