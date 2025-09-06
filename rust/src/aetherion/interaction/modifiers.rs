//C:/ZV9/zv9.aetherion/rust/src/aetherion/interaction/modifiers.rs

//! Tile modifiers for in-game editing and procedural mutation.
//! Supports painting, toggling, and rule-based transformations.

use crate::aetherion::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};

/// A trait for applying mutations to a tile chunk.
pub trait TileModifier {
    fn apply(&self, chunk: &mut MapDataChunk);
}

/// Paints a single tile at a given position.
pub struct PaintTile {
    pub pos: SerializableVector2i,
    pub tile: TileInfo,
}

impl TileModifier for PaintTile {
    fn apply(&self, chunk: &mut MapDataChunk) {
        chunk.insert(self.pos, self.tile.clone());
    }
}

/// Toggles a tile on/off (e.g. structure presence).
pub struct ToggleTile {
    pub pos: SerializableVector2i,
}

impl TileModifier for ToggleTile {
    fn apply(&self, chunk: &mut MapDataChunk) {
        if chunk.tiles.contains_key(&self.pos) {
            chunk.tiles.remove(&self.pos);
        } else {
            // Default tile if toggled on
            let tile = TileInfo {
                source_id: 0,
                atlas_coords: SerializableVector2i { x: 0, y: 0 },
                alternate_id: 0,
                rotation: 0,
                layer: 0,
                flags: 0,
            };
            chunk.insert(self.pos, tile);
        }
    }
}

//end modifiers.rs