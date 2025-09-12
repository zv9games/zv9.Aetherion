//C:/ZV9/zv9.aetherion/rust/src/shared/grid_bounds.rs

/// ✅ Suggestions for shared/grid_bounds.rs

// 🔧 Add utility methods for spatial queries:
//     - `fn corners(&self) -> [SerializableVector2i; 4]`
//     - `fn center(&self) -> SerializableVector2i`
//     - Useful for placement, visualization, and chunk alignment

// 🧩 Add support for resizing or shifting bounds:
//     - `fn expand(&mut self, dx: i32, dy: i32)`
//     - `fn shift_origin(&mut self, offset: SerializableVector2i)`
//     - Enables dynamic map growth or viewport movement

// 🚦 Add validation and clamping:
//     - Ensure `width` and `height` are non-negative
//     - Prevent invalid bounds from propagating into grid logic

// 📚 Document coordinate system assumptions:
//     - Clarify whether origin is top-left, center, or bottom-left
//     - Note how bounds interact with chunk or tile indexing

// 🧪 Add unit tests for `contains()` logic:
//     - Validate edge cases, negative positions, and origin-relative checks

// 🧼 Optional: Add display or debug formatting:
//     - `impl std::fmt::Display for GridBounds`
//     - Useful for logging, diagnostics, or editor overlays

// 🚀 Future: Add intersection and containment checks:
//     - `fn intersects(&self, other: &GridBounds) -> bool`
//     - `fn contains_bounds(&self, other: &GridBounds) -> bool`
//     - Enables spatial partitioning or region queries

// 🧠 Consider exposing bounds as a Godot-friendly type:
//     - e.g. `fn to_rect2i() -> Rect2i`
//     - Useful for editor visualization or UI integration


use crate::aetherion::pipeline::data::vector::SerializableVector2i;

#[derive(Debug, Clone, Copy)]
pub struct GridBounds {
    pub width: i32,
    pub height: i32,
    pub origin: SerializableVector2i,
}

impl GridBounds {
    pub fn contains(&self, pos: SerializableVector2i) -> bool {
        let rel_x = pos.x - self.origin.x;
        let rel_y = pos.y - self.origin.y;
        rel_x >= 0 && rel_y >= 0 && rel_x < self.width && rel_y < self.height
    }
}

//end grid_bounds.rs