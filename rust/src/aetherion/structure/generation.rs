 /// ✅ Suggestions for map_build_options.rs

// 🔧 Expand `GodotNoiseType` to match full internal `NoiseType`:
//     - Add Perlin, Simplex, Cellular, etc.
//     - Update `From<GString>` and `Into<GString>` to support all variants
//     - Prevent silent fallback to Basic on unknown strings

// 🧩 Make `MapBuildOptions::to_noise_config()` fully configurable:
//     - Expose `fill_ratio`, `steps`, `birth_limit`, and `survival_limit` as fields
//     - Enables full procedural control from GDScript or editor UI

// 🚦 Add validation and clamping:
//     - Ensure `width`, `height`, and `seed` are within safe bounds
//     - Clamp `fill_ratio` to [0.0, 1.0] if exposed

// 📚 Document integration expectations:
//     - Clarify how this struct is used by the engine and editor
//     - Note assumptions about `black` and `blue` tile roles (e.g. land vs water)

/// 🧪 Add unit tests for conversion logic:
//     - Validate `to_noise_config()` and `noise_type()` mappings
//     - Ensure `default()` produces expected values

// 🧼 Optional: Add helper methods for display or logging:
//     - `fn describe(&self) -> String`
//     - Useful for debugging, diagnostics, or editor overlays

// 🚀 Future: Add support for presets or profiles:
//     - e.g. “island”, “cave”, “plains” with pre-filled config values
//     - Could expose `fn preset(name: &str) -> Self`

// 🧠 Consider exposing tile styling as a struct:
//     - Replace `black` and `blue` with `TileStyle { land: Vector2i, water: Vector2i }`
//     - Improves clarity and extensibility


// 🧭 Map generation configuration for Godot integration and internal engine use.

use serde::{Serialize, Deserialize};

use godot::prelude::*;
use godot::builtin::GString;

use crate::aetherion::generator::noise::NoiseType;
use crate::aetherion::generator::noise_config::NoiseConfig;
use crate::aetherion::pipeline::data::vector::SerializableVector2i;

//
// ─── Noise Type Wrapper ────────────────────────────────────────────────────────
//

/// 🧠 Editor-safe wrapper for exposing noise types to GDScript.
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

/// 🗺️ Configuration options for procedural map generation.
/// Used in the editor and passed into the engine from GDScript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapBuildOptions {
    pub width: i32,                        // Grid width in tiles
    pub height: i32,                       // Grid height in tiles
    pub seed: u64,                         // Seed for deterministic generation
    pub mode: GodotNoiseType,              // Noise generation mode
    pub animate: bool,                     // Whether to animate tile placement
    pub black: SerializableVector2i,       // Atlas coordinates for "black" tile
    pub blue: SerializableVector2i,        // Atlas coordinates for "blue" tile
}

impl MapBuildOptions {
    /// Creates a default checkerboard-style map using CellularAutomata.
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

    /// Converts this struct into a NoiseConfig for internal use.
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

    /// Converts Godot-facing config into internal noise type.
    pub fn noise_type(&self) -> NoiseType {
        self.mode.to_internal()
    }

    /// Returns the total number of tiles.
    pub fn total_tiles(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Returns true if animation is enabled.
    pub fn is_animated(&self) -> bool {
        self.animate
    }

    /// Converts black and blue tile positions to Godot-native Vector2i.
    pub fn godot_tile_coords(&self) -> (Vector2i, Vector2i) {
        (self.black.into(), self.blue.into())
    }
}
