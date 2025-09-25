/// ✅ Suggestions for util/direction.rs

// 🔧 Add diagonal directions:
//     - e.g. `Direction { dx: 1, dy: 1 }` for southeast
//     - Could expose `fn diagonals() -> [Direction; 4]`
//     - Useful for pathfinding, smoothing, and spatial queries

// 🧩 Add vector conversion helpers:
//     - `fn to_vector2i(&self) -> Vector2i`
//     - Enables seamless integration with Godot and grid math

// 🚦 Improve rotation logic:
//     - Consider `fn rotate_cw(n: u8)` and `fn rotate_ccw(n: u8)`
//     - Enables flexible turning for AI or procedural agents

// 📚 Document coordinate assumptions:
//     - Clarify that `UP` means decreasing `y` (top-left origin)
//     - Prevents confusion in tilemaps or camera logic

// 🧪 Add unit tests for `left`, `right`, `reverse`, and `all`:
//     - Validate correctness and symmetry of directional transforms

// 🧼 Optional: Add direction name or label:
//     - `fn name(&self) -> &'static str` returning "Up", "Down", etc.
//     - Useful for debugging, UI, or logging

// 🚀 Future: Add direction arithmetic:
//     - `impl Add for Direction`, `impl Neg for Direction`
//     - Enables vector-style composition and reversal

// 🧠 Consider exposing direction to GDScript:
//     - Wrap in a Godot-friendly enum or export via utility node
//     - Useful for runtime control, movement logic, or editor tooling


#[allow(unused_imports)]
use crate::zv9_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction {
    pub dx: i32,
    pub dy: i32,
}

impl Direction {
    pub const UP: Direction = Direction { dx: 0, dy: -1 };
    pub const RIGHT: Direction = Direction { dx: 1, dy: 0 };
    pub const DOWN: Direction = Direction { dx: 0, dy: 1 };
    pub const LEFT: Direction = Direction { dx: -1, dy: 0 };

    /// Returns all four cardinal directions.
    pub fn all() -> [Direction; 4] {
        [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]
    }

    /// Returns the direction to the left (counter-clockwise).
    pub fn left(&self) -> Self {
        match *self {
            Self::UP => Self::LEFT,
            Self::LEFT => Self::DOWN,
            Self::DOWN => Self::RIGHT,
            Self::RIGHT => Self::UP,
            _ => *self,
        }
    }

    /// Returns the direction to the right (clockwise).
    pub fn right(&self) -> Self {
        match *self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            _ => *self,
        }
    }

    /// (Optional) Returns the opposite direction.
    pub fn reverse(&self) -> Self {
        match *self {
            Self::UP => Self::DOWN,
            Self::DOWN => Self::UP,
            Self::LEFT => Self::RIGHT,
            Self::RIGHT => Self::LEFT,
            _ => *self,
        }
    }
}
