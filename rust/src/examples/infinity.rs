// infinity.rs

use godot::prelude::*;
use crate::map::{MapGrid, TileType};
use crate::utils::{Position, Direction};

/// ♾️ Infinity — Handles infinite map expansion and recursive simulation.
/// Used for procedural generation, streaming terrain, or unbounded logic.
pub struct Infinity {
    pub origin: Position,
    pub active_chunks: Vec<Position>,
    pub expansion_radius: i32,
}

impl Infinity {
    pub fn new(origin: Position, radius: i32) -> Self {
        Self {
            origin,
            active_chunks: vec![origin],
            expansion_radius: radius,
        }
    }

    /// Expands the map outward from the origin in all directions.
    pub fn expand(&mut self, grid: &mut MapGrid) {
        let directions = Direction::all();

        for chunk in &self.active_chunks {
            for dir in &directions {
                let next = chunk.step(*dir);
                if grid.is_within_bounds(next) && grid.get_tile(next) == TileType::Empty {
                    grid.set_tile(next, TileType::Chunk);
                    self.active_chunks.push(next);
                }
            }
        }
    }

    /// Simulates infinite ticks or steps.
    pub fn tick(&mut self, grid: &mut MapGrid, steps: usize) {
        for _ in 0..steps {
            self.expand(grid);
        }
    }

    /// Returns the current bounds of the infinite system.
    pub fn bounds(&self) -> (Position, Position) {
        let min = self.active_chunks.iter().fold(self.origin, |acc, p| acc.min(*p));
        let max = self.active_chunks.iter().fold(self.origin, |acc, p| acc.max(*p));
        (min, max)
    }
}
