use godot::prelude::*;
use crate::aetherion::generator::noise::NoiseType;
use crate::aetherion::generator::noise_config::NoiseConfig;

/// Editor-safe wrapper for exposing noise types to GDScript.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GodotNoiseType {
    Basic,
    CellularAutomata,
}

impl GodotNoiseType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "automata" => Self::CellularAutomata,
            _ => Self::Basic,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::CellularAutomata => "automata",
        }
    }

    pub fn to_internal(self) -> NoiseType {
        match self {
            Self::Basic => NoiseType::Basic,
            Self::CellularAutomata => NoiseType::CellularAutomata,
        }
    }
}

/// Godot-facing configuration for procedural map generation.
#[derive(GodotClass)]
#[class(init)]
pub struct MapBuildOptions {
    #[export]
    pub width: i32,

    #[export]
    pub height: i32,

    #[export]
    pub seed: i64,

    #[export]
    pub mode: GString, // "basic", "automata"

    #[export]
    pub animate: bool,

    #[export]
    pub black: Vector2i,

    #[export]
    pub blue: Vector2i,
}

#[godot_api]
impl MapBuildOptions {
    /// Converts this configuration into an internal noise config.
    pub fn to_noise_config(&self) -> NoiseConfig {
        NoiseConfig {
            width: self.width as usize,
            height: self.height as usize,
            seed: self.seed as u64,
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
        }
    }

    /// Converts the Godot-facing noise mode into an internal enum.
    pub fn noise_type(&self) -> NoiseType {
        GodotNoiseType::from_str(self.mode.to_string().as_str()).to_internal()
    }
}
