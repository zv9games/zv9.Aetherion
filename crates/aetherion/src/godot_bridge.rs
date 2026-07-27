//! Godot 4 GDExtension entry (feature = "godot").
//!
//! godot-rust 0.4.x (same generation as SSXL-ext confirmation builds).

use crate::chunk::ChunkCoord;
use crate::conductor::run_region;
use crate::generate::FillMode;
use godot::init::{ExtensionLibrary, InitLevel};
use godot::prelude::*;

struct AetherionExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!(
                "[Aetherion] GDExtension Scene init ({})",
                crate::version_string()
            );
        }
    }
}

/// Root engine node exposed to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct AetherionEngine {
    base: Base<Node>,
    ticks: u64,
    last_tiles: u64,
    last_ms: u64,
    last_summary: GString,
}

#[godot_api]
impl INode for AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        godot_print!("[Aetherion] engine node init ({})", crate::version_string());
        Self {
            base,
            ticks: 0,
            last_tiles: 0,
            last_ms: 0,
            last_summary: GString::from(""),
        }
    }

    fn ready(&mut self) {
        godot_print!("[Aetherion] ready — health={}", crate::health());
        let report = self.run_generate_inner(0, 0, 2, 2, 16, 0, 42);
        godot_print!("[Aetherion] auto-smoke {}", report);
    }

    fn process(&mut self, _delta: f64) {
        self.ticks = self.ticks.wrapping_add(1);
    }
}

#[godot_api]
impl AetherionEngine {
    #[func]
    fn get_version(&self) -> GString {
        GString::from(crate::version_string().as_str())
    }

    #[func]
    fn get_ticks(&self) -> u64 {
        self.ticks
    }

    #[func]
    fn get_last_tiles(&self) -> u64 {
        self.last_tiles
    }

    #[func]
    fn get_last_ms(&self) -> u64 {
        self.last_ms
    }

    #[func]
    fn get_last_summary(&self) -> GString {
        self.last_summary.clone()
    }

    /// `mode`: 0 = checkerboard, 1 = hash noise.
    #[func]
    fn generate_region(
        &mut self,
        origin_x: i32,
        origin_y: i32,
        chunks_x: i32,
        chunks_y: i32,
        chunk_size: i32,
        mode: i32,
        seed: i32,
    ) -> GString {
        let summary = self.run_generate_inner(
            origin_x,
            origin_y,
            chunks_x.max(1) as u32,
            chunks_y.max(1) as u32,
            chunk_size.max(1) as u32,
            mode,
            seed as u32,
        );
        godot_print!("[Aetherion] {}", summary);
        GString::from(summary.as_str())
    }

    #[func]
    fn bench_medium(&mut self) -> GString {
        self.generate_region(0, 0, 8, 8, 64, 1, 7)
    }
}

impl AetherionEngine {
    fn run_generate_inner(
        &mut self,
        origin_x: i32,
        origin_y: i32,
        chunks_x: u32,
        chunks_y: u32,
        chunk_size: u32,
        mode: i32,
        seed: u32,
    ) -> String {
        let fill = if mode == 0 {
            FillMode::Checkerboard
        } else {
            FillMode::HashNoise
        };
        let report = run_region(
            ChunkCoord::new(origin_x, origin_y),
            chunks_x,
            chunks_y,
            chunk_size,
            fill,
            seed,
        );
        self.last_tiles = report.tiles;
        self.last_ms = report.elapsed_ms as u64;
        let summary = report.summary();
        self.last_summary = GString::from(summary.as_str());
        summary
    }
}
