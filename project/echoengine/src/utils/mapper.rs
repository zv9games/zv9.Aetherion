// mapper.rs

//! 🗺️ Mapper Module
//! Provides type translation utilities across EchoEngine domains.
//! Used for overlay rendering, signal generation, and dimension bridging.

use crate::engine::types::{Tile, TileKind};
use crate::engine::signal::{EchoSignal, SignalKind};
use crate::engine::dimension::Position;
use crate::engine::lifecycle::Phase;

/// Map a Tile to an EchoSignal
pub fn tile_to_signal(tile: &Tile) -> EchoSignal {
    EchoSignal {
        kind: match tile.kind {
            TileKind::Spawn => SignalKind::Pulse,
            TileKind::Wall => SignalKind::Block,
            TileKind::Portal => SignalKind::Warp,
            _ => SignalKind::Neutral,
        },
        position: tile.position.clone(),
        strength: 1.0,
        phase: Phase::Active,
    }
}

/// Map a Phase to a debug color
pub fn phase_to_color(phase: &Phase) -> &'static str {
    match phase {
        Phase::Init => "#888888",
        Phase::Active => "#00FF00",
        Phase::Dormant => "#5555FF",
        Phase::Retired => "#FF0000",
    }
}

/// Map a Position across dimensions (e.g., 2D to 3D stub)
pub fn map_position(pos: &Position, mode: &str) -> Position {
    match mode {
        "3D" => Position {
            x: pos.x,
            y: pos.y,
            z: Some(0), // Stubbed for now
        },
        _ => Position {
            x: pos.x,
            y: pos.y,
            z: None,
        },
    }
}
