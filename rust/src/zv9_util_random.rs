//C:/ZV9/zv9/aetherion/rust/src/zv9_util_random.rs 

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// 🎲 Seeded random number generator wrapper for reproducible procedural logic.
pub struct Randomizer {
    rng: StdRng,
}

impl Randomizer {
    /// Creates a new `Randomizer` from a given seed.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Generates a value within a range.
    pub fn gen_range<T: rand::distributions::uniform::SampleUniform>(&mut self, min: T, max: T) -> T {
        self.rng.gen_range(min..max)
    }

    /// Returns true with the given probability (0.0 to 1.0).
    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability)
    }

    /// Generates a float between 0.0 and 1.0 (f32).
    pub fn gen_unit_f32(&mut self) -> f32 {
        self.rng.gen()
    }

    /// Generates a float between 0.0 and 1.0 (f64).
    pub fn gen_unit_f64(&mut self) -> f64 {
        self.rng.gen()
    }
}

/// 🔁 Returns either -1 or 1 randomly.
pub fn random_sign(rng: &mut impl Rng) -> i32 {
    if rng.gen_bool(0.5) { 1 } else { -1 }
}

/// 🧭 Returns a random 2D direction vector (±1, ±1).
pub fn random_direction_2d(rng: &mut impl Rng) -> (i32, i32) {
    (random_sign(rng), random_sign(rng))
}

// the end
