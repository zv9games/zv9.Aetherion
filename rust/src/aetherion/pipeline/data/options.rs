//C:/ZV9/zv9.aetherion/rust/src/aetherion/pipeline/data/options.rs

use godot::prelude::*;
use crate::aetherion::generator::noise::{NoiseType};
use crate::aetherion::generator::noise_config::NoiseConfig;
use super::vector::SerializableVector2i;
use godot::builtin::GString;


/// Editor-safe wrapper for exposing noise types to GDScript.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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


/// Configuration options for procedural map generation.
/// Used in the editor and passed into the engine from GDScript.
#[derive(Debug, Clone)]
pub struct MapBuildOptions {
    /// Grid width in tiles.
    pub width: i32,

    /// Grid height in tiles.
    pub height: i32,

    /// Seed for deterministic generation.
    pub seed: u64,

    /// Noise generation mode (editor-safe wrapper).
    pub mode: GodotNoiseType,

    /// Whether to animate tile placement.
    pub animate: bool,

    /// Atlas coordinates for "black" tile.
    pub black: Vector2i,

    /// Atlas coordinates for "blue" tile.
    pub blue: Vector2i,
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
            black: Vector2i::new(0, 0),
            blue: Vector2i::new(1, 1),
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
}

//end options.rs