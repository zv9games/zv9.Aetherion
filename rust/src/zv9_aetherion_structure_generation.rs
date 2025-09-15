//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_structure_generation.rs

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
use std::str::FromStr;
use std::fmt;
use serde::{Serialize, Deserialize};
use godot::prelude::*;
use godot::builtin::GString;
use crate::zv9_prelude::*; // Includes NoiseType, SerializableVector2i, etc.


//
// ─── Noise Type Wrapper ────────────────────────────────────────────────────────
//

/// 🧠 Editor-safe wrapper for exposing noise types to GDScript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GodotNoiseType {
    Basic,
    Perlin,
    Simplex,
    Cellular,
    CellularAutomata,
}

impl GodotNoiseType {
    pub fn to_internal(self) -> NoiseType {
        match self {
            Self::Basic => NoiseType::Basic,
            Self::Perlin => NoiseType::Perlin,
            Self::Simplex => NoiseType::Simplex,
            Self::Cellular => NoiseType::Cellular,
            Self::CellularAutomata => NoiseType::CellularAutomata,
        }
    }
}

impl fmt::Display for GodotNoiseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Basic => "basic",
            Self::Perlin => "perlin",
            Self::Simplex => "simplex",
            Self::Cellular => "cellular",
            Self::CellularAutomata => "automata",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for GodotNoiseType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(Self::Basic),
            "perlin" => Ok(Self::Perlin),
            "simplex" => Ok(Self::Simplex),
            "cellular" => Ok(Self::Cellular),
            "automata" => Ok(Self::CellularAutomata),
            _ => Err(()),
        }
    }
}

impl From<GodotNoiseType> for GString {
    fn from(value: GodotNoiseType) -> Self {
        value.to_string().into()
    }
}

impl From<GString> for GodotNoiseType {
    fn from(value: GString) -> Self {
        GodotNoiseType::from_str(&value.to_string()).unwrap_or(Self::Basic)
    }
}


impl From<GodotNoiseType> for NoiseType {
    fn from(value: GodotNoiseType) -> Self {
        match value {
            GodotNoiseType::Basic => NoiseType::Basic,
            GodotNoiseType::Perlin => NoiseType::Perlin,
            GodotNoiseType::Simplex => NoiseType::Simplex,
            GodotNoiseType::Cellular => NoiseType::Cellular,
            GodotNoiseType::CellularAutomata => NoiseType::CellularAutomata,
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
    pub width: i32,
    pub height: i32,
    pub seed: u64,
    pub mode: GodotNoiseType,
    pub animate: bool,
    pub fill_ratio: f32,
    pub steps: usize,
    pub birth_limit: u8,
    pub survival_limit: u8,
    pub black: SerializableVector2i,
    pub blue: SerializableVector2i,
	pub delivery_interval_ms: Option<u32>,
}

impl MapBuildOptions {
    /// Creates a default checkerboard-style map using CellularAutomata.
    pub fn default(width: i32, height: i32, seed: u64) -> Self {
        Self {
            width: width.clamp(1, 4096),
            height: height.clamp(1, 4096),
            seed,
            mode: GodotNoiseType::CellularAutomata,
            animate: false,
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
            black: SerializableVector2i { x: 0, y: 0 },
            blue: SerializableVector2i { x: 1, y: 1 },
			delivery_interval_ms: Some(2),
        }
    }

    /// Converts this struct into a NoiseConfig for internal use.
    pub fn to_noise_config(&self) -> NoiseConfig {
        NoiseConfig {
            width: self.width.max(1) as usize,
            height: self.height.max(1) as usize,
            seed: self.seed,
            fill_ratio: f64::from(self.fill_ratio.clamp(0.0, 1.0)),

            steps: self.steps,
            birth_limit: self.birth_limit,
            survival_limit: self.survival_limit,
        }
    }

    pub fn noise_type(&self) -> NoiseType {
        self.mode.to_internal()
    }

    pub fn total_tiles(&self) -> usize {
        (self.width * self.height).max(1) as usize
    }

    pub fn is_animated(&self) -> bool {
        self.animate
    }

    pub fn godot_tile_coords(&self) -> (Vector2i, Vector2i) {
        (self.black.into(), self.blue.into())
    }

    /// Optional: Describe config for debugging or logging.
    pub fn describe(&self) -> String {
        format!(
            "MapBuildOptions: {}x{}, mode={}, seed={}, animated={}, fill={}, steps={}, birth={}, survival={}",
            self.width,
            self.height,
            self.mode,
            self.seed,
            self.animate,
            self.fill_ratio,
            self.steps,
            self.birth_limit,
            self.survival_limit
        )
    }
}

// the end