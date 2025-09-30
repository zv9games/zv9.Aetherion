use godot::prelude::*;

/// 📦 Shared semantic structure for tile placement
#[derive(Debug, Clone)]
pub struct TilePayload {
    pub pos: Vector2i,
    pub source_id: i32,
    pub atlas_coords: Vector2i,
    pub alt_id: i32,
    pub label: Option<String>, // e.g., "floor", "wall", veil, etc.
}

/// 🧭 Veil tile glyphs
pub const ZERO_TILE: Vector2i = Vector2i::new(0, 7);
pub const ONE_TILE:  Vector2i = Vector2i::new(1, 7);
pub const BLACK_TILE: Vector2i = Vector2i::new(14, 13);

/// 🧱 Terrain tile definitions
pub const WALL_TILE:  Vector2i = Vector2i::new(15, 13);
pub const FLOOR_TILE: Vector2i = Vector2i::new(14, 13);

/// 🔄 Ceremony timing
pub const DEFAULT_FLIP_RATE: f32 = 0.01;
pub const DEFAULT_GRID_WIDTH: i32 = 33;
pub const DEFAULT_GRID_HEIGHT: i32 = 33;
