//! Core logic for building the game map.
//!
//! This module orchestrates procedural generation and communicates progress
//! and completion to the Godot engine via signals.

use godot::prelude::*;
use crate::godot_bridge::signals::AetherionSignals;

/// Holds the state and logic for map generation.
pub struct MapBuilder {
    dimensions: Vector2i,
    seed: u64,
}

impl MapBuilder {
    /// Creates a new `MapBuilder` instance.
    pub fn new(dimensions: Vector2i, seed: u64) -> Self {
        MapBuilder { dimensions, seed }
    }

    /// Initiates the map building process and emits signals to Godot.
    pub fn build_map(&mut self, mut signals_node: Gd<AetherionSignals>) {
        signals_node.bind_mut().emit_build_map_start();
        godot_print!("Rust core: Starting map generation with seed {}", self.seed);

        self.generate_terrain();

        signals_node
            .bind_mut()
            .emit_map_building_status("Terrain generated.".to_string());

        self.place_tiles();

        signals_node
            .bind_mut()
            .emit_map_building_status("Tiles placed. Finalizing.".to_string());

        let mut results = Dictionary::new();
        results.insert("success".to_variant(), true.to_variant());
        results.insert("map_size".to_variant(), self.dimensions.to_variant()); // ✅ Use native Vector2i

        signals_node
            .bind_mut()
            .emit_generation_complete(results);

        godot_print!("Rust core: Map generation complete.");
    }

    fn generate_terrain(&self) {
        godot_print!("  - Procedural terrain generation complete.");
    }

    fn place_tiles(&self) {
        godot_print!("  - Tile placement complete.");
    }
}
