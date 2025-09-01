use godot::prelude::*;
use crate::aetherion::generator::noise::NoiseType;
use crate::aetherion::generator::noise_config::NoiseConfig;

/// Internal enum for mapping string-based noise modes to engine logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GodotNoiseType {
    Basic,
    CellularAutomata,
}

impl GodotNoiseType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "automata" => GodotNoiseType::CellularAutomata,
            _ => GodotNoiseType::Basic,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            GodotNoiseType::Basic => "basic",
            GodotNoiseType::CellularAutomata => "automata",
        }
    }

    pub fn to_internal(self) -> NoiseType {
        match self {
            GodotNoiseType::Basic => NoiseType::Basic,
            GodotNoiseType::CellularAutomata => NoiseType::CellularAutomata,
        }
    }
}

/// Godot-facing configuration for procedural map generation.
/// Used in the editor and passed into the engine from GDScript.
#[derive(GodotClass)]
#[class(init)]
pub struct MapBuildOptions {
    #[export]
    pub width: i32,

    #[export]
    pub height: i32,

    #[export]
    pub seed: i64,

    /// Noise mode as a string ("basic", "automata")
    #[export]
    pub mode: GString,

    #[export]
    pub animate: bool,

    #[export]
    pub black: Vector2i,

    #[export]
    pub blue: Vector2i,
}

#[godot_api]
impl MapBuildOptions {
    fn init() -> Self {
        Self {
            width: 64,
            height: 64,
            seed: 42,
            mode: "automata".into(),
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
            seed: self.seed as u64,

            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
        }
    }

    /// Converts the string-based mode into an internal NoiseType.
    pub fn noise_type(&self) -> NoiseType {
        GodotNoiseType::from_str(self.mode.to_string().as_str()).to_internal()

    }
}
