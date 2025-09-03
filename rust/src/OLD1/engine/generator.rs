// generator.rs

//! 🎲 Generator Module
//! This module defines the core procedural generation logic for EchoEngine.
//! It is dimension-agnostic and designed to work with both 2D and 3D contexts
//! via the `DimensionContext` trait defined in `dimension.rs`.

use crate::engine::dimension::DimensionContext;
use crate::engine::types::{Tile, TileKind};
use crate::utils::config::EngineConfig;

/// Represents a generated tile field
pub struct TileField {
    pub tiles: Vec<Tile>,
}

/// Generate a tile field using the provided dimensional context
pub fn generate_field<D: DimensionContext>(
    config: &EngineConfig,
) -> TileField {
    let mut tiles = Vec::new();

    for x in 0..config.grid_width {
        for y in 0..config.grid_height {
            let pos = D::position(x, y, None); // ← Fixed: added z argument
            let kind = if (x + y) % 2 == 0 {
                TileKind::Floor
            } else {
                TileKind::Wall
            };

            tiles.push(Tile {
                position: pos,
                kind,
                metadata: None,
            });
        }
    }

    TileField { tiles }
}
