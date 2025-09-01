use crate::aetherion::generator::noise::NoiseType;

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

