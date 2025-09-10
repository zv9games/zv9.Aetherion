use crate::util::Position;
use crate::shared::grid_bounds::GridBounds;
use crate::aetherion::pipeline::data::vector::SerializableVector2i;
use std::collections::HashMap;

/// Types of tiles that can exist in the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Chunk,
    Wall,
    Floor,
    Path,
    // Add more as needed
}

/// Grid structure holding tile data and spatial bounds.
#[derive(Debug)]
pub struct MapGrid {
    tiles: HashMap<Position, TileType>,
    bounds: GridBounds,
}

impl MapGrid {
    /// Creates a new grid with the given bounds.
    pub fn new(bounds: GridBounds) -> Self {
        Self {
            tiles: HashMap::new(),
            bounds,
        }
    }

    /// Retrieves the tile at the given position.
    pub fn get_tile(&self, pos: Position) -> TileType {
        *self.tiles.get(&pos).unwrap_or(&TileType::Empty)
    }

    /// Sets a tile at the given position.
    pub fn set_tile(&mut self, pos: Position, tile: TileType) {
        if self.is_within_bounds(pos) {
            self.tiles.insert(pos, tile);
        }
    }

    /// Checks whether the position is within the grid bounds.
    pub fn is_within_bounds(&self, pos: Position) -> bool {
        let sv = SerializableVector2i { x: pos.x, y: pos.y };
        self.bounds.contains(sv)
    }
}
