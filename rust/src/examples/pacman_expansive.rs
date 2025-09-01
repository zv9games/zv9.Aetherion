// pacman_expansive.rs

use godot::prelude::*;
use crate::map::{TileType, MapGrid};
use crate::utils::{Direction, Position};

/// 🟡 PacmanExpansive — Expands map regions in a Pac-Man-like fashion.
/// Used during procedural generation to simulate corridor growth or AI pathing.
pub struct PacmanExpansive {
    pub position: Position,
    pub direction: Direction,
    pub visited: Vec<Position>,
}

impl PacmanExpansive {
    pub fn new(start: Position, direction: Direction) -> Self {
        Self {
            position: start,
            direction,
            visited: vec![start],
        }
    }

    /// Expands the map by one tile in the current direction.
    pub fn expand(&mut self, grid: &mut MapGrid) -> Option<Position> {
        let next_pos = self.position.step(self.direction);

        if grid.is_within_bounds(next_pos) && grid.get_tile(next_pos) == TileType::Empty {
            grid.set_tile(next_pos, TileType::Path);
            self.visited.push(next_pos);
            self.position = next_pos;
            Some(next_pos)
        } else {
            None
        }
    }

    /// Attempts to turn left or right if blocked.
    pub fn try_turn(&mut self, grid: &MapGrid) {
        for &turn_dir in &[self.direction.left(), self.direction.right()] {
            let test_pos = self.position.step(turn_dir);
            if grid.is_within_bounds(test_pos) && grid.get_tile(test_pos) == TileType::Empty {
                self.direction = turn_dir;
                break;
            }
        }
    }

    /// Runs a full expansion loop until blocked.
    pub fn run(&mut self, grid: &mut MapGrid, max_steps: usize) {
        for _ in 0..max_steps {
            if self.expand(grid).is_none() {
                self.try_turn(grid);
                if self.expand(grid).is_none() {
                    break;
                }
            }
        }
    }
}
