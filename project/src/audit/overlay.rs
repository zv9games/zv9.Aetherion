// overlay.rs

//! 🪞 Overlay Module
//! Renders visual introspection of EchoEngine state.
//! Designed for Godot debug UI, CLI snapshots, or multiplayer dashboards.

use crate::engine::types::{Tile, TileKind};
use crate::engine::dimension::Position;
use crate::engine::lifecycle::Phase;
use crate::engine::signal::EchoSignal;
use crate::audit::manifest::Manifest;

/// Overlay frame — what to render
#[derive(Clone, Debug)]
pub struct OverlayFrame {
    pub tick: u64,
    pub phase: Phase,
    pub tiles: Vec<Tile>,
    pub signals: Vec<EchoSignal>,
    pub manifest: Vec<String>, // Compressed module summaries
}

/// Overlay renderer trait
pub trait OverlayRenderer {
    fn render(&self, frame: &OverlayFrame);
}

/// Default stdout overlay
pub struct StdoutOverlay;

impl OverlayRenderer for StdoutOverlay {
    fn render(&self, frame: &OverlayFrame) {
        println!("🔍 Overlay Tick: {}", frame.tick);
        println!("📦 Phase: {:?}", frame.phase);
        println!("🧱 Tiles: {}", frame.tiles.len());
        println!("🔔 Signals: {}", frame.signals.len());
        for summary in &frame.manifest {
            println!("📜 {}", summary);
        }
    }
}

/// Helper to compress manifest entries
pub fn compress_manifest(manifest: &Manifest) -> Vec<String> {
    manifest
        .entries
        .values()
        .map(|entry| format!("{} — {}", entry.name, entry.purpose))
        .collect()
}
