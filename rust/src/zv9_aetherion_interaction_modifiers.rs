//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_interaction_modifiers.rs

// ✅ Suggestions for aetherion/interaction/modifiers.rs

// 🔧 Add more modifier types:
//     - `FillRegion`, `ReplaceTile`, `RotateTile`, `ShiftLayer`, etc.
//     - Enables richer in-game editing and procedural effects

// 🧩 Add modifier composition or chaining:
//     - e.g. `CompositeModifier(Vec<Box<dyn TileModifier>>)`
//     - Useful for applying multiple transformations in sequence

// 🚦 Add undo/redo support:
//     - Track previous tile state before mutation
//     - Could integrate with an `EditorHistory` system

// 📚 Document expected behavior of modifiers:
//     - Clarify how modifiers interact with layers, flags, and tile metadata
//     - Could include examples or usage patterns

// 🧪 Add unit tests for `apply()` logic:
//     - Validate insertion, removal, and toggle behavior
//     - Ensure modifiers behave predictably across edge cases

// 🧼 Optional: Add debug or display hooks:
//     - `fn describe(&self) -> String` for logging or UI previews
//     - Useful for editor overlays or diagnostics

// 🚀 Future: Add conditional or rule-based modifiers:
//     - e.g. `ApplyIf(predicate: fn(&TileInfo) -> bool)`
//     - Enables reactive or context-aware editing

// 🧠 Consider exposing modifiers to scripting or DSL:
//     - Allow runtime-defined modifiers via `ProcCommand::ApplyModifier(...)`


// Tile modifiers for in-game editing and procedural mutation.
// Supports painting, toggling, and rule-based transformations.

use crate::zv9_prelude::*;


// A trait for applying mutations to a tile chunk.
pub trait TileModifier {
    fn apply(&self, chunk: &mut MapDataChunk);
}

// Paints a single tile at a given position.
pub struct PaintTile {
    pub pos: SerializableVector2i,
    pub tile: TileInfo,
}

impl TileModifier for PaintTile {
    fn apply(&self, chunk: &mut MapDataChunk) {
        chunk.insert(self.pos, self.tile.clone());
    }
}

// Toggles a tile on/off (e.g. structure presence).
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
				variant_id: None,
				frame_count: None,
				animation_speed: None,
			};

            chunk.insert(self.pos, tile);
        }
    }
}

// the end