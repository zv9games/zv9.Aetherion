/// ✅ Suggestions for util/position.rs

// 🔧 Add directional utilities:
//     - `fn offset(&self, dx: i32, dy: i32) -> Position`
//     - `fn to_tuple(&self) -> (i32, i32)`
//     - Improves ergonomics for grid manipulation and interop

// 🧩 Add integration with Godot types:
//     - `fn to_vector2i(&self) -> Vector2i`
//     - Enables seamless use in Godot tilemaps or UI systems

// 🚦 Improve precision handling:
//     - Consider exposing `distance_squared()` for cheaper comparisons
//     - Useful in pathfinding and proximity checks

// 📚 Document coordinate semantics:
//     - Clarify whether origin is top-left, center, or bottom-left
//     - Note how `step()` relates to `Direction` and movement logic

// 🧪 Add unit tests for `step`, `min`, `max`, and `distance_to`:
//     - Validate correctness across edge cases and negative coordinates

// 🧼 Optional: Add arithmetic traits:
//     - `impl Add for Position`, `impl Sub for Position`
//     - Enables vector-style math and spatial reasoning

// 🚀 Future: Add region or bounds helpers:
//     - e.g. `fn within(&self, bounds: GridBounds) -> bool`
//     - Useful for chunking, map generation, and collision checks

// 🧠 Consider exposing position to GDScript:
//     - Wrap in a Godot-friendly struct or export via utility node
//     - Useful for runtime control, debugging, or editor integration

#[allow(unused_imports)]
use crate::zv9_prelude::*;
use std::ops::AddAssign;
#[allow(unused_imports)]
use godot::prelude::*;

/// A 2D grid position used for tile placement, movement, and spatial queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Creates a new position from coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns a new position stepped in the given direction.
    pub fn step(&self, direction: Direction) -> Self {
        Self {
            x: self.x + direction.dx,
            y: self.y + direction.dy,
        }
    }

    /// Returns the minimum of two positions (component-wise).
    pub fn min(self, other: Position) -> Position {
        Position {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Returns the maximum of two positions (component-wise).
    pub fn max(self, other: Position) -> Position {
        Position {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// Returns the Euclidean distance to another position.
    pub fn distance_to(&self, other: Position) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Converts this position to a Godot-native `Vector2i`.
    pub fn to_vec2i(&self) -> Vector2i {
        Vector2i::new(self.x, self.y)
    }

    /// Converts this position to a serializable vector.
    pub fn to_serializable(&self) -> SerializableVector2i {
        SerializableVector2i { x: self.x, y: self.y }
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.dx as i32;
        self.y += rhs.dy as i32;
    }
}

impl From<Position> for SerializableVector2i {
    fn from(pos: Position) -> Self {
        SerializableVector2i { x: pos.x, y: pos.y }
    }
}

impl From<Position> for Vector2i {
    fn from(pos: Position) -> Self {
        Vector2i::new(pos.x, pos.y)
    }
}
