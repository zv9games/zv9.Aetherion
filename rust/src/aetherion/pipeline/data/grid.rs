use crate::util::Position;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Chunk,
    Wall,
    Floor,
	Path,
    // Add more as needed
}

#[derive(Debug)]
pub struct MapGrid {
    tiles: HashMap<Position, TileType>,
}

impl MapGrid {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn get_tile(&self, pos: Position) -> TileType {
        *self.tiles.get(&pos).unwrap_or(&TileType::Empty)
    }

    pub fn set_tile(&mut self, pos: Position, tile: TileType) {
        self.tiles.insert(pos, tile);
    }

    pub fn is_within_bounds(&self, pos: Position) -> bool {
        // You can expand this logic later with actual bounds
        true
    }
}
