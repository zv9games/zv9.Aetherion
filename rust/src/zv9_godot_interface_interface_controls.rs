//C:/ZV9/zv9.aetherion/rust/src/zv9__godot_interface__controls.rs

use godot::prelude::*;
use std::str::FromStr;
#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 🧭 ControlPanel — UI node for interacting with the Aetherion runtime.
///
/// Provides configuration inputs for terrain generation, structure overlays,
/// pacing control, and animation toggles. Dispatches build options to the engine.
#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct ControlPanel {
    #[base]
    base: Base<Control>,

    /// Terrain generation mode (e.g. "Perlin", "Cellular", etc.)
    #[export]
    terrain_mode: GString,

    /// Structure overlay mode (e.g. "None", "Maze", "Rooms")
    #[export]
    structure_mode: GString,

    /// Chunk delivery pacing in milliseconds
    #[export]
    pacing_ms: i32,

    /// Whether to animate terrain generation
    #[export]
    animate: bool,

    /// Starting color (used for biome or seed placement)
    #[export]
    black: Vector2i,

    /// Ending color (used for biome or seed placement)
    #[export]
    blue: Vector2i,
}

#[godot_api]
impl ControlPanel {
	#[allow(dead_code)]
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            terrain_mode: GString::from("Perlin"),
            structure_mode: GString::from("None"),
            pacing_ms: 2,
            animate: false,
            black: Vector2i::new(0, 0),
            blue: Vector2i::new(64, 64),
        }
    }

    #[func]
    fn _ready(&self) {
		godot_print!("🧭 ControlPanel ready.");
		log_component!("ControlPanel", "UI node for interacting with the Aetherion runtime");
	}

    /// Dispatches a map generation request to the engine using current settings.
    #[func]
    fn generate_map(&self) {
        let pacing = self.pacing_ms.clamp(1, 1000);

        let godot_mode = GodotNoiseType::from_str(&self.terrain_mode.to_string())
            .unwrap_or_else(|_| {
                godot_warn!("⚠️ Unknown terrain mode: {}", self.terrain_mode);
                GodotNoiseType::Perlin
            });

        let mode: NoiseType = godot_mode.into();

        let config = NoiseConfig {
            width: 128,
            height: 128,
            seed: 42,
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
        };

        let sync = GodotSync::init();
        let streamer = ChunkStreamer::new(sync.clone(), pacing as u64);

        spawn_map_builder(
            streamer,
            config,
            mode,
            self.animate,
            self.black.into(),
            self.blue.into(),
        );

        self.base.to_init_gd().emit_signal("map_generation_requested", &[]);
        godot_print!("🧭 ControlPanel: Map generation triggered.");
    }

    /// Sets the chunk pacing interval in milliseconds.
    #[func]
    fn set_pacing(&mut self, ms: i32) {
        self.pacing_ms = ms.clamp(1, 1000);
    }

    /// Applies a preset configuration by name.
    #[func]
    fn apply_preset(&mut self, name: GString) {
        match name.to_string().as_str() {
            "island" => {
                self.terrain_mode = GString::from("Perlin");
                self.structure_mode = GString::from("None");
                self.pacing_ms = 5;
                self.animate = true;
            }
            "maze" => {
                self.terrain_mode = GString::from("Cellular");
                self.structure_mode = GString::from("Maze");
                self.pacing_ms = 2;
                self.animate = false;
            }
            "plains" => {
                self.terrain_mode = GString::from("Perlin");
                self.structure_mode = GString::from("Rooms");
                self.pacing_ms = 3;
                self.animate = true;
            }
            _ => {
                godot_warn!("⚠️ Unknown preset: {}", name);
            }
        }
    }

    /// Returns a summary of current settings.
    #[func]
    fn describe_settings(&self) -> String {
        format!(
            "Mode: {}, Structure: {}, Pacing: {}ms, Animate: {}, Black: {:?}, Blue: {:?}",
            self.terrain_mode, self.structure_mode, self.pacing_ms, self.animate, self.black, self.blue
        )
    }

    /// Converts current settings to a Godot Dictionary.
    #[func]
    fn to_config_dict(&self) -> Dictionary {
        let mut dict = Dictionary::new();
        let _ = dict.insert("terrain_mode", self.terrain_mode.clone());
        let _ = dict.insert("structure_mode", self.structure_mode.clone());
        let _ = dict.insert("pacing_ms", self.pacing_ms);
        let _ = dict.insert("animate", self.animate);
        let _ = dict.insert("black", self.black);
        let _ = dict.insert("blue", self.blue);
        dict
    }
}



// the end 