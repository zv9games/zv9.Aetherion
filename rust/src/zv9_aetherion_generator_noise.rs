//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_generator_noise.rs
use rand::Rng;
use rand::SeedableRng;
#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 🔊 Basic sine-cosine hybrid noise function.
/// Placeholder: replace with a real algorithm later.
pub fn basic_noise(x: f32, y: f32) -> f32 {
    (x.sin() + y.cos()) * 0.5
}

/// 🎛 Enum representing supported noise types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoiseType {
    Basic,
    Perlin,
    Simplex,
    Cellular,
    CellularAutomata,
}

impl NoiseType {
    /// Returns the string name of the noise type.
    pub fn as_str(&self) -> &'static str {
        match self {
            NoiseType::Basic => "basic",
            NoiseType::Perlin => "perlin",
            NoiseType::Simplex => "simplex",
            NoiseType::Cellular => "cellular",
            NoiseType::CellularAutomata => "automata",
        }
    }

    /// Indicates whether the noise type is currently implemented.
    pub fn is_available(&self) -> bool {
        matches!(self, NoiseType::Basic | NoiseType::CellularAutomata)
    }
}

/// 🧪 Dispatcher for coordinate-based noise sampling.
pub fn generate_noise(x: f32, y: f32, noise_type: NoiseType) -> f32 {
    match noise_type {
        NoiseType::Basic => basic_noise(x, y),
        NoiseType::Perlin => 0.0,           // TODO: Implement Perlin noise
        NoiseType::Simplex => 0.0,          // TODO: Implement Simplex noise
        NoiseType::Cellular => 0.0,         // TODO: Implement Cellular noise
        NoiseType::CellularAutomata => 0.0, // Not applicable for direct sampling
    }
}

/// 🧱 Generates a binary grid using the specified noise type.
/// For CellularAutomata, applies rule-based evolution after initialization.
pub fn generate_grid_noise(
    width: usize,
    height: usize,
    noise_type: NoiseType,
    seed: u64,
) -> Vec<Vec<u8>> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut grid = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            grid[y][x] = if rng.gen_bool(0.45) { 1 } else { 0 };
        }
    }

    if noise_type == NoiseType::CellularAutomata {
        cellular_automata(&mut grid, 5, 4, 3);
    }

    grid
}

/// 🔁 Evolves a binary grid using cellular automata rules.
pub fn cellular_automata(
    grid: &mut Vec<Vec<u8>>,
    steps: usize,
    birth_limit: u8,
    survival_limit: u8,
) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..steps {
        let mut new_grid = grid.clone();

        for y in 0..height {
            for x in 0..width {
                let neighbors = count_alive_neighbors(grid, x, y);
                let cell = grid[y][x];

                new_grid[y][x] = match cell {
                    1 if neighbors < survival_limit => 0,
                    0 if neighbors > birth_limit => 1,
                    _ => cell,
                };
            }
        }

        *grid = new_grid;
    }
}

/// 📊 Counts the number of alive neighbors around a cell.
fn count_alive_neighbors(grid: &[Vec<u8>], x: usize, y: usize) -> u8 {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if ny >= 0 && ny < grid.len() as isize && nx >= 0 && nx < grid[0].len() as isize {
                count += grid[ny as usize][nx as usize];
            }
        }
    }

    count
}
#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_basic_noise_sampling() {
        for i in 0..10_000 {
            let x = i as f32 * 0.01;
            let y = (i % 100) as f32 * 0.01;
            let val = generate_noise(x, y, NoiseType::Basic);
            assert!(val.is_finite());
        }
    }

    #[test]
    fn stress_grid_generation_basic() {
        let grid = generate_grid_noise(128, 128, NoiseType::Basic, 42);
        assert_eq!(grid.len(), 128);
        assert_eq!(grid[0].len(), 128);
    }

    #[test]
    fn stress_grid_generation_automata() {
        let grid = generate_grid_noise(64, 64, NoiseType::CellularAutomata, 12345);
        assert_eq!(grid.len(), 64);
        assert_eq!(grid[0].len(), 64);

        let alive_count: usize = grid.iter().flatten().filter(|&&v| v == 1).count();
        assert!(alive_count > 0); // Shouldn't be all dead
    }

    #[test]
    fn stress_cellular_automata_evolution() {
        let mut grid = vec![vec![1; 32]; 32];
        cellular_automata(&mut grid, 10, 4, 3);
        let alive_count: usize = grid.iter().flatten().filter(|&&v| v == 1).count();
        assert!(alive_count < 1024); // Should evolve, not stay fully alive
    }

    #[test]
    fn stress_noise_type_dispatch() {
        for noise in [
            NoiseType::Basic,
            NoiseType::Perlin,
            NoiseType::Simplex,
            NoiseType::Cellular,
            NoiseType::CellularAutomata,
        ] {
            let val = generate_noise(1.0, 1.0, noise);
            assert!(val.is_finite());
        }
    }
}


// the end