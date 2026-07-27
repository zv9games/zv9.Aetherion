//! Godot 4 GDExtension entry (feature = "godot").
//!
//! godot-rust 0.4.x (same generation as SSXL-ext confirmation builds).

use crate::chunk::ChunkCoord;
use crate::conductor::run_region_data;
use crate::generate::FillMode;
use crate::host_multimesh::apply_chunks_to_multimesh;
use crate::host_tilemap::{apply_chunks_to_tilemap, ensure_demo_tileset};
use godot::classes::{MultiMeshInstance2D, TileMap};
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

/// Host visualization target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ApplyTarget {
    None,
    TileMap,
    MultiMesh,
}

/// Root engine node exposed to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct AetherionEngine {
    base: Base<Node>,
    ticks: u64,
    last_tiles: u64,
    last_ms: u64,
    last_apply_ms: u64,
    last_summary: GString,
    apply_mode: ApplyTarget,
    tilemap: Option<Gd<TileMap>>,
    multimesh: Option<Gd<MultiMeshInstance2D>>,
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
            last_apply_ms: 0,
            last_summary: GString::from(""),
            apply_mode: ApplyTarget::None,
            tilemap: None,
            multimesh: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("[Aetherion] ready — health={}", crate::health());
        self.try_autobind();
        let report = self.run_generate_inner(0, 0, 2, 2, 16, 0, 42, true);
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
    fn get_last_apply_ms(&self) -> u64 {
        self.last_apply_ms
    }

    #[func]
    fn get_last_summary(&self) -> GString {
        self.last_summary.clone()
    }

    /// Bind a TileMap for host apply (SSXL-ext host_tilemap lineage).
    #[func]
    fn bind_tilemap(&mut self, map: Gd<TileMap>) {
        let mut map = map;
        ensure_demo_tileset(&mut map);
        godot_print!("[Aetherion] bound TileMap {}", map.get_name());
        self.tilemap = Some(map);
        self.apply_mode = ApplyTarget::TileMap;
    }

    /// Bind MultiMeshInstance2D for fast large floods (Plan-B-lite).
    #[func]
    fn bind_multimesh(&mut self, mmi: Gd<MultiMeshInstance2D>) {
        godot_print!("[Aetherion] bound MultiMeshInstance2D {}", mmi.get_name());
        self.multimesh = Some(mmi);
        self.apply_mode = ApplyTarget::MultiMesh;
    }

    /// Prefer MultiMesh for large demos when both are available.
    #[func]
    fn set_prefer_multimesh(&mut self, prefer: bool) {
        if prefer && self.multimesh.is_some() {
            self.apply_mode = ApplyTarget::MultiMesh;
        } else if self.tilemap.is_some() {
            self.apply_mode = ApplyTarget::TileMap;
        }
    }

    /// `mode`: 0 = checkerboard, 1 = hash noise. Applies to bound host when present.
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
            true,
        );
        godot_print!("[Aetherion] {}", summary);
        GString::from(summary.as_str())
    }

    /// CPU-only generation (no host apply).
    #[func]
    fn generate_region_cpu(
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
            false,
        );
        godot_print!("[Aetherion] cpu {}", summary);
        GString::from(summary.as_str())
    }

    /// Medium flood (~16k tiles) with host apply.
    #[func]
    fn bench_medium(&mut self) -> GString {
        self.generate_region(0, 0, 4, 4, 32, 1, 7)
    }

    /// ~1M MultiMesh flood (16×16 × 64² = 1,048,576). Kept for lighter machines.
    #[func]
    fn flood_million(&mut self) -> GString {
        if self.multimesh.is_some() {
            self.apply_mode = ApplyTarget::MultiMesh;
        }
        self.generate_region(0, 0, 16, 16, 64, 1, 11)
    }

    /// Big showcase flood: 50×50 chunks of 64² = **10,240,000** tiles (MultiMesh).
    /// Pacman-class demos sat ~3.5M; 10M covers “crazy adventure” headroom.
    #[func]
    fn flood_10m(&mut self) -> GString {
        if self.multimesh.is_some() {
            self.apply_mode = ApplyTarget::MultiMesh;
        }
        self.generate_region(0, 0, 50, 50, 64, 1, 17)
    }

    /// CPU-only ~4M tiles (32×32 × 64²).
    #[func]
    fn bench_4m_cpu(&mut self) -> GString {
        self.generate_region_cpu(0, 0, 32, 32, 64, 1, 13)
    }

    /// CPU-only ~10.24M tiles (50×50 × 64²).
    #[func]
    fn bench_10m_cpu(&mut self) -> GString {
        self.generate_region_cpu(0, 0, 50, 50, 64, 1, 19)
    }
}

impl AetherionEngine {
    fn try_autobind(&mut self) {
        // MultiMesh first for large demos if present.
        if let Some(child) = self.base().get_node_or_null("MultiMeshInstance2D") {
            if let Ok(mm) = child.try_cast::<MultiMeshInstance2D>() {
                self.bind_multimesh(mm);
            }
        }
        if let Some(child) = self.base().get_node_or_null("TileMap") {
            if let Ok(map) = child.try_cast::<TileMap>() {
                let prefer_mm = matches!(self.apply_mode, ApplyTarget::MultiMesh);
                self.bind_tilemap(map);
                // MultiMesh wins when both exist (large-flood path).
                if prefer_mm || self.multimesh.is_some() {
                    self.apply_mode = ApplyTarget::MultiMesh;
                }
            }
        }
        if let Some(parent) = self.base().get_parent() {
            if self.multimesh.is_none() {
                if let Some(node) = parent.get_node_or_null("MultiMeshInstance2D") {
                    if let Ok(mm) = node.try_cast::<MultiMeshInstance2D>() {
                        self.bind_multimesh(mm);
                    }
                }
            }
            if self.tilemap.is_none() {
                if let Some(node) = parent.get_node_or_null("TileMap") {
                    if let Ok(map) = node.try_cast::<TileMap>() {
                        self.bind_tilemap(map);
                    }
                }
            }
        }
    }

    fn run_generate_inner(
        &mut self,
        origin_x: i32,
        origin_y: i32,
        chunks_x: u32,
        chunks_y: u32,
        chunk_size: u32,
        mode: i32,
        seed: u32,
        apply: bool,
    ) -> String {
        let fill = if mode == 0 {
            FillMode::Checkerboard
        } else {
            FillMode::HashNoise
        };
        let (chunks, report) = run_region_data(
            ChunkCoord::new(origin_x, origin_y),
            chunks_x,
            chunks_y,
            chunk_size,
            fill,
            seed,
        );
        self.last_tiles = report.tiles;
        self.last_ms = report.elapsed_ms as u64;
        self.last_apply_ms = 0;

        let mut summary = report.summary();
        if apply {
            match self.apply_mode {
                ApplyTarget::TileMap => {
                    if let Some(map) = self.tilemap.as_mut() {
                        let (cells, apply_ms) = apply_chunks_to_tilemap(map, &chunks);
                        self.last_apply_ms = apply_ms as u64;
                        summary =
                            format!("{summary} | tilemap apply {cells} cells in {apply_ms} ms");
                    } else {
                        summary = format!("{summary} | apply skipped (no TileMap)");
                    }
                }
                ApplyTarget::MultiMesh => {
                    if let Some(mmi) = self.multimesh.as_mut() {
                        let (n, apply_ms) = apply_chunks_to_multimesh(mmi, &chunks);
                        self.last_apply_ms = apply_ms as u64;
                        summary =
                            format!("{summary} | multimesh apply {n} instances in {apply_ms} ms");
                    } else {
                        summary = format!("{summary} | apply skipped (no MultiMesh)");
                    }
                }
                ApplyTarget::None => {
                    summary = format!("{summary} | apply skipped (no host bound)");
                }
            }
        }
        self.last_summary = GString::from(summary.as_str());
        summary
    }
}
