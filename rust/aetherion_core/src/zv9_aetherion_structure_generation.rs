//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_structure_generation.rs

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
        value.to_internal()
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
            fill_ratio: self.fill_ratio.clamp(0.0, 1.0) as f64,
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

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_default_config_bounds() {
        let config = MapBuildOptions::default(9999, -100, 42);
        assert!(config.width <= 4096);
        assert!(config.height >= 1);
        assert_eq!(config.mode, GodotNoiseType::CellularAutomata);
    }

    #[test]
    fn stress_noise_config_conversion() {
        let config = MapBuildOptions::default(128, 128, 123);
        let noise = config.to_noise_config();
        assert_eq!(noise.width, 128);
        assert_eq!(noise.height, 128);
        assert!(noise.fill_ratio <= 1.0);
    }

    #[test]
    fn stress_tile_count_calculation() {
        let config = MapBuildOptions::default(64, 64, 0);
        assert_eq!(config.total_tiles(), 4096);
    }

    #[test]
    fn stress_godot_tile_coords() {
        let config = MapBuildOptions::default(32, 32, 0);
        let (black, blue) = config.godot_tile_coords();
        assert_eq!(black.x, 0);
        assert_eq!(blue.y, 1);
    }

    #[test]
    fn stress_description_formatting() {
        let config = MapBuildOptions::default(16, 16, 999);
        let desc = config.describe();
        assert!(desc.contains("MapBuildOptions"));
        assert!(desc.contains("seed=999"));
    }
}

use crate::zv9_aetherion_pipeline_data_tile::{TileInfo, tile_flags};

/// Procedural tile generator that computes tile metadata on demand.
pub fn tile_at(x: u64, y: u64, seed: u64) -> TileInfo {
    let hash = x.wrapping_mul(31).wrapping_add(y.wrapping_mul(17)).wrapping_add(seed);
    TileInfo {
        rotation: (hash % 4) as u8,
        variant_id: Some((hash % 7) as i32),
        flags: tile_flags::VISIBLE | tile_flags::COLLIDABLE,
        ..Default::default()
    }
}

/// Infinite iterator over a virtual tile field.
pub fn generate_virtual_field(width: u64, height: u64, seed: u64) -> impl Iterator<Item = (u64, u64, TileInfo)> {
    (0..height).flat_map(move |y| {
        (0..width).map(move |x| (x, y, tile_at(x, y, seed)))
    })
}



// the end