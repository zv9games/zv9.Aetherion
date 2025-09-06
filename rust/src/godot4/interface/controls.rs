//C:/ZV9/zv9.aetherion/rust/src/godot4/interface/controls.rs

//! 🧭 Control panel UI for interacting with the Aetherion runtime.
//!
//! Provides configuration inputs for terrain generation, structure overlays,
//! pacing control, and animation toggles. Dispatches build options to the engine.

use godot::prelude::*;
use crate::aetherion::pipeline::data::map_build_options::{MapBuildOptions, GodotNoiseType};
use crate::godot4::api::engine::AetherionEngine;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct ControlPanel {
    #[base]
    base: Base<Control>,

    // Terrain generation mode (e.g. "Perlin", "Cellular", etc.)
    #[export]
    terrain_mode: String,

    // Structure overlay mode (e.g. "None", "Maze", "Rooms")
    #[export]
    structure_mode: String,

    // Chunk delivery pacing in milliseconds
    #[export]
    pacing_ms: i32,

    // Whether to animate terrain generation
    #[export]
    animate: bool,
}

#[godot_api]
impl ControlPanel {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            terrain_mode: "Perlin".into(),
            structure_mode: "None".into(),
            pacing_ms: 2,
            animate: false,
        }
    }

    /// Dispatches a map generation request to the engine using current settings.
    #[func]
    fn generate_map(&self) {
        let options = MapBuildOptions {
            mode: GodotNoiseType::from_str(&self.terrain_mode),
            structure: self.structure_mode.clone(),
            pacing_ms: self.pacing_ms,
            animate: self.animate,
            ..Default::default()
        };

        AetherionEngine::spawn_map(options);
    }

    /// Toggles animation mode for terrain generation.
    #[func]
    fn toggle_animate(&mut self, value: bool) {
        self.animate = value;
    }

    /// Sets the terrain generation mode.
    #[func]
    fn set_terrain_mode(&mut self, mode: String) {
        self.terrain_mode = mode;
    }

    /// Sets the structure overlay mode.
    #[func]
    fn set_structure_mode(&mut self, mode: String) {
        self.structure_mode = mode;
    }

    /// Sets the chunk pacing interval in milliseconds.
    #[func]
    fn set_pacing(&mut self, ms: i32) {
        self.pacing_ms = ms;
    }
}

//end controls.rs