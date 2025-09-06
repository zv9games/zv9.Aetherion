use godot::prelude::*;
use crate::aetherion::pipeline::data::{MapGrid, TileType};
use crate::util::{Position, Direction};

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

    /// Expands the map outward from the origin in all cardinal directions.
    pub fn expand(&mut self, grid: &mut MapGrid) {
        let directions = Direction::all();
        let mut new_chunks = Vec::new();

        for chunk in &self.active_chunks {
            for dir in &directions {
                let next = chunk.step(*dir);
                if grid.is_within_bounds(next) && grid.get_tile(next) == TileType::Empty {
                    grid.set_tile(next, TileType::Chunk);
                    new_chunks.push(next);
                }
            }
        }

        self.active_chunks.extend(new_chunks);
    }

    /// Simulates multiple expansion steps.
    pub fn tick(&mut self, grid: &mut MapGrid, steps: usize) {
        for _ in 0..steps {
            self.expand(grid);
        }
    }

    /// Returns the bounding box of all active chunks.
    pub fn bounds(&self) -> (Position, Position) {
        let min = self.active_chunks.iter().fold(self.origin, |acc, p| acc.min(*p));
        let max = self.active_chunks.iter().fold(self.origin, |acc, p| acc.max(*p));
        (min, max)
    }
}
