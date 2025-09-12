/// ✅ Suggestions for aetherion/pipeline/data/map_build_options.rs

// 🔧 Expand `GodotNoiseType` to match full internal `NoiseType` enum:
//     - Add Perlin, Simplex, Cellular, etc.
//     - Update `From<GString>` and `Into<GString>` to handle all variants

// 🧩 Make `MapBuildOptions::to_noise_config()` fully configurable:
//     - Expose `fill_ratio`, `steps`, `birth_limit`, and `survival_limit` as fields
//     - Enables fine-tuning from GDScript or editor UI

// 🚦 Add validation logic:
//     - Clamp `width`, `height`, and `fill_ratio` to safe ranges
//     - Emit warnings or fallback to defaults if values are out of bounds

// 📚 Document intended usage in Godot:
//     - Clarify how this struct is used to trigger terrain generation
//     - Note assumptions about `black` and `blue` tile roles

// 🧪 Add unit tests for conversion and default logic:
//     - Validate `to_noise_config()` and `noise_type()` mappings
//     - Ensure `default()` produces expected values

// 🧼 Optional: Add helper methods for debugging or display:
//     - `fn describe(&self) -> String`
//     - Useful for logging or editor overlays

// 🚀 Future: Add support for presets or profiles:
//     - e.g. “cave”, “island”, “plains” with pre-filled config values
//     - Could expose `fn preset(name: &str) -> Self`

// 🧠 Consider exposing tile styling as a struct:
//     - Replace `black` and `blue` with `TileStyle { land: Vector2i, water: Vector2i }`
//     - Improves clarity and extensibility


use serde::{Serialize, Deserialize};

use godot::prelude::*;
use godot::builtin::GString;

use crate::aetherion::generator::noise::NoiseType;
use crate::aetherion::generator::noise_config::NoiseConfig;
use super::vector::SerializableVector2i;

//
// ─── Noise Type Wrapper ────────────────────────────────────────────────────────
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GodotNoiseType {
    Basic,
    CellularAutomata,
}

impl GodotNoiseType {
    pub fn to_internal(self) -> NoiseType {
        match self {
            Self::Basic => NoiseType::Basic,
            Self::CellularAutomata => NoiseType::CellularAutomata,
        }
    }
}

impl From<GodotNoiseType> for GString {
    fn from(value: GodotNoiseType) -> Self {
        match value {
            GodotNoiseType::Basic => "basic".into(),
            GodotNoiseType::CellularAutomata => "automata".into(),
        }
    }
}

impl From<GString> for GodotNoiseType {
    fn from(value: GString) -> Self {
        match value.to_string().as_str() {
            "automata" => GodotNoiseType::CellularAutomata,
            _ => GodotNoiseType::Basic,
        }
    }
}

//
// ─── Map Build Options ─────────────────────────────────────────────────────────
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapBuildOptions {
    pub width: i32,
    pub height: i32,
    pub seed: u64,
    pub mode: GodotNoiseType,
    pub animate: bool,
    pub black: SerializableVector2i,
    pub blue: SerializableVector2i,
}

impl MapBuildOptions {
    pub fn default(width: i32, height: i32, seed: u64) -> Self {
        Self {
            width,
            height,
            seed,
            mode: GodotNoiseType::CellularAutomata,
            animate: false,
            black: SerializableVector2i { x: 0, y: 0 },
            blue: SerializableVector2i { x: 1, y: 1 },
        }
    }

    pub fn to_noise_config(&self) -> NoiseConfig {
        NoiseConfig {
            width: self.width as usize,
            height: self.height as usize,
            seed: self.seed,
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
        }
    }

    pub fn noise_type(&self) -> NoiseType {
        self.mode.to_internal()
    }

    pub fn total_tiles(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn is_animated(&self) -> bool {
        self.animate
    }

    pub fn godot_tile_coords(&self) -> (Vector2i, Vector2i) {
        (self.black.into(), self.blue.into())
    }
}
