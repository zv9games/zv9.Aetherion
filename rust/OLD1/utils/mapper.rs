//! 🗺️ Mapper Module
//! Provides type translation utilities across EchoEngine domains.
//!
//! Used for:
//! - Overlay rendering
//! - Signal generation
//! - Dimension bridging

use crate::engine::types::{Tile, TileKind};
use crate::interface::signal::EchoSignal;
use crate::engine::dimension::{Position, Dim2, Dim3};
use crate::engine::lifecycle::Phase;

/// Converts a `Tile` into an `EchoSignal`, typically used for overlay updates.
pub fn tile_to_signal(tile: &Tile) -> EchoSignal {
    EchoSignal::TileUpdated(tile.clone())
}

/// Maps a `Phase` to a corresponding debug color (hex string).
pub fn phase_to_color(phase: &Phase) -> &'static str {
    match phase {
        Phase::Init           => "#888888",
        Phase::Generate       => "#00FF00",
        Phase::Animate        => "#5555FF",
        Phase::Register       => "#FF8800",
        Phase::FlipDimension  => "#00FFFF",
        Phase::Tick           => "#FF0000",
        Phase::Idle           => "#888888", // Reuse gray for idle
    }
}

/// Translates a `Position` between 2D and 3D representations.
/// 
/// - `"3D"` mode lifts 2D into 3D space with `z = 0`.
/// - Any other mode compresses 3D into 2D by dropping `z`.
pub fn map_position(pos: &Position, mode: &str) -> Position {
    match mode {
        "3D" => match pos {
            Position::TwoD(d2) => Position::ThreeD(Dim3 {
                x: d2.x,
                y: d2.y,
                z: 0,
            }),
            Position::ThreeD(d3) => Position::ThreeD(d3.clone()),
        },
        _ => match pos {
            Position::ThreeD(d3) => Position::TwoD(Dim2 {
                x: d3.x,
                y: d3.y,
            }),
            Position::TwoD(d2) => Position::TwoD(d2.clone()),
        },
    }
}
