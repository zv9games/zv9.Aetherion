//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_generator_noise_config.rs

/// ✅ Suggestions for aetherion/generator/noise_config.rs

// 🔧 Add serialization support:
//     - `#[derive(Serialize, Deserialize)]`
//     - Useful for saving/loading noise presets or exposing to Godot

// 🧩 Add validation logic:
//     - Ensure `fill_ratio` is between 0.0 and 1.0
//     - Clamp or error on invalid `birth_limit` / `survival_limit` values

// 🚦 Expand `generate_grid_from_config()`:
//     - Implement actual logic for `CellularAutomata` (e.g. random fill + evolution)
//     - Integrate real Perlin/Simplex noise generators
//     - Consider using a noise crate or custom algorithm

// 📚 Add documentation examples:
//     - Show how different configs affect terrain generation
//     - Could include sample configs for caves, islands, dungeons

// 🧪 Add unit tests for grid generation:
//     - Validate dimensions, fill ratios, and deterministic output with seed

// 🧼 Optional: Add helper methods to `NoiseConfig`:
//     - `fn is_valid(&self) -> bool`
//     - `fn describe(&self) -> String` for debugging

// 🚀 Future: Add support for multi-layer or 3D noise:
//     - e.g. `Vec<Vec<Vec<u8>>>` for volumetric terrain
//     - Could integrate with `Dimension` enum

// 🧠 Consider exposing noise blending or masking:
//     - Combine multiple noise types for biome transitions or overlays


use crate::zv9_prelude::*;


// Configuration for procedural noise generation.
// Used to control grid dimensions, fill ratio, seed, and automata rules.

#[derive(Debug, Clone)]
pub struct NoiseConfig {
    /// Width of the grid to generate.
    pub width: usize,

    /// Height of the grid to generate.
    pub height: usize,

    /// Seed for deterministic noise generation.
    pub seed: u64,

    /// Initial fill ratio (0.0 to 1.0).
    /// Determines the probability of a cell starting as "alive".
    pub fill_ratio: f64,

    /// Number of evolution steps for cellular automata.
    pub steps: usize,

    /// Birth threshold: number of neighbors required to spawn a new cell.
    pub birth_limit: u8,

    /// Survival threshold: minimum neighbors required to keep a cell alive.
    pub survival_limit: u8,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            seed: 42,
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
        }
    }
}



/// Generates a 2D grid of tile values based on the given config and noise type.
/// Returns a grid of 0s and 1s representing tile states.
pub fn generate_grid_from_config(config: &NoiseConfig, mode: NoiseType) -> Vec<Vec<u8>> {
    match mode {
		NoiseType::CellularAutomata => {
			vec![vec![0; config.width]; config.height]
		}
		NoiseType::Basic => {
			vec![vec![1; config.width]; config.height]
		}
		NoiseType::Perlin => {
			todo!("Perlin noise not implemented yet")
		}
		NoiseType::Simplex => {
			todo!("Simplex noise not implemented yet")
		}
		NoiseType::Cellular => {
			todo!("Cellular noise not implemented yet")
		}
	}

}

// the end