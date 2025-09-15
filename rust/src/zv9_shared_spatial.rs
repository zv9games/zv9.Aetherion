//C:/ZV9/zv9.aetherion/rust/src/shared/spatial.rs

// ✅ Suggestions for shared/spatial.rs

// 🔧 Add diagonal-only neighbor utility:
//     - `fn diagonal_neighbors(pos: Vector2i) -> Vec<Vector2i>`
//     - Useful for pathfinding heuristics or terrain smoothing

// 🧩 Add region generation helpers:
//     - `fn rect_region(origin: Vector2i, width: i32, height: i32) -> Vec<Vector2i>`
//     - Enables bulk tile access, chunking, or procedural placement

// 🚦 Add bounds struct integration:
//     - Accept `GridBounds` instead of raw width/height in `in_bounds()`
//     - Improves clarity and reuse across spatial modules

// 📚 Document coordinate assumptions:
//     - Clarify origin (top-left vs center), directionality, and indexing
//     - Prevent confusion in map generation or neighbor logic

// 🧪 Add unit tests for adjacency and bounds:
//     - Validate edge cases, corner tiles, and negative positions

// 🧼 Optional: Add neighbor filtering:
//     - `fn neighbors_within(pos: Vector2i, bounds: GridBounds) -> Vec<Vector2i>`
//     - Useful for safe traversal and chunk-aware logic

// 🚀 Future: Add pathfinding stubs or traits:
//     - e.g. `fn heuristic(a: Vector2i, b: Vector2i) -> i32`
//     - `fn neighbors_for_pathfinding(...)` with cost or terrain awareness

// 🧠 Consider exposing spatial utilities to GDScript:
//     - Wrap in a `SpatialUtils` node or module
//     - Enables dynamic queries and editor integration


// Spatial utilities for grid-based reasoning.
// Includes adjacency, bounds, regions, and pathfinding stubs.

use godot::builtin::Vector2i;
#[allow(unused_imports)]
use crate::zv9_prelude::*;


/// Returns the 4 cardinal neighbors of a tile.
pub fn cardinal_neighbors(pos: Vector2i) -> Vec<Vector2i> {
    vec![
        Vector2i::new(pos.x, pos.y - 1),
        Vector2i::new(pos.x + 1, pos.y),
        Vector2i::new(pos.x, pos.y + 1),
        Vector2i::new(pos.x - 1, pos.y),
    ]
}

/// Returns the 8 surrounding neighbors of a tile.
pub fn all_neighbors(pos: Vector2i) -> Vec<Vector2i> {
    let mut neighbors = Vec::with_capacity(8);
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                neighbors.push(Vector2i::new(pos.x + dx, pos.y + dy));
            }
        }
    }
    neighbors
}

/// Checks if a position is within bounds.
pub fn in_bounds(pos: Vector2i, width: i32, height: i32) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < width && pos.y < height
}

//end spatial.rs