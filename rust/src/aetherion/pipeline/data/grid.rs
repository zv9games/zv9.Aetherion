
/// ✅ Suggestions for aetherion/pipeline/data/map_grid.rs

// 🔧 Add tile mutation and removal methods:
//     - `fn remove_tile(&mut self, pos: Position)`
//     - `fn toggle_tile(&mut self, pos: Position, a: TileType, b: TileType)`
//     - Useful for editing, procedural mutation, or gameplay logic

// 🧩 Add spatial utilities:
//     - `fn all_positions(&self) -> impl Iterator<Item = &Position>`
//     - `fn count_by_type(&self, tile_type: TileType) -> usize`
//     - Enables analytics, rendering, or rule-based systems

// 🚦 Add bounds enforcement or diagnostics:
//     - Emit warning or log if `set_tile()` is called out-of-bounds
//     - Could integrate with `EngineMessage::Warning`

// 📚 Document intended use:
//     - Clarify how `MapGrid` differs from `MapDataChunk`
//     - Note whether this is runtime-only or used in serialization

// 🧪 Add unit tests for tile access and bounds checking:
//     - Validate `get_tile()`, `set_tile()`, and `is_within_bounds()` behavior

// 🧼 Optional: Add debug summary or visual dump:
//     - `fn summary(&self) -> String`
//     - Useful for diagnostics or editor overlays

// 🚀 Future: Add support for multi-layer or typed grids:
//     - e.g. `HashMap<Position, Vec<TileType>>` or `HashMap<Position, TileInfo>`
//     - Enables richer terrain, structure, and gameplay overlays

// 🧠 Consider exposing grid diffing or merging:
//     - `fn merge(&mut self, other: &MapGrid)`
//     - Useful for procedural overlays or undo/redo


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
