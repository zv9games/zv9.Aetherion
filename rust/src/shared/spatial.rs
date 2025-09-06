//C:/ZV9/zv9.aetherion/rust/src/shared/spatial.rs

//! Spatial utilities for grid-based reasoning.
//! Includes adjacency, bounds, regions, and pathfinding stubs.

use godot::builtin::Vector2i;


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