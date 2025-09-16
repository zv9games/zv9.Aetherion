//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_generator_patterns.rs


#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 🟫 Checkerboard pattern: alternating tiles.
pub fn checkerboard(x: i32, y: i32) -> bool {
    (x + y) % 2 == 0
}

/// 🌀 Radial symmetry pattern centered at origin.
/// Returns true if the tile is within a circular radius.
pub fn radial(x: i32, y: i32, radius: f32) -> bool {
    let dist_sq = (x * x + y * y) as f32;
    dist_sq <= radius * radius
}

/// 🧬 Fractal-like pattern using bitwise XOR.
/// Creates a chaotic grid of toggled tiles.
pub fn xor_fractal(x: i32, y: i32) -> bool {
    (x ^ y) % 7 == 0
}

/// 🧵 Vertical stripes pattern.
pub fn vertical_stripes(x: i32, stripe_width: i32) -> bool {
    (x / stripe_width) % 2 == 0
}

/// 🧵 Horizontal stripes pattern.
pub fn horizontal_stripes(y: i32, stripe_height: i32) -> bool {
    (y / stripe_height) % 2 == 0
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_checkerboard_grid() {
        let mut count = 0;
        for y in 0..100 {
            for x in 0..100 {
                if checkerboard(x, y) {
                    count += 1;
                }
            }
        }
        assert!(count > 0);
    }

    #[test]
    fn stress_radial_bounds() {
        let radius = 50.0;
        let mut inside = 0;
        for y in -100..100 {
            for x in -100..100 {
                if radial(x, y, radius) {
                    inside += 1;
                }
            }
        }
        assert!(inside > 0);
    }

    #[test]
    fn stress_xor_fractal_distribution() {
        let mut hits = 0;
        for y in 0..128 {
            for x in 0..128 {
                if xor_fractal(x, y) {
                    hits += 1;
                }
            }
        }
        assert!(hits > 0);
    }

    #[test]
    fn stress_vertical_stripes() {
        let mut light = 0;
        for x in 0..100 {
            if vertical_stripes(x, 10) {
                light += 1;
            }
        }
        assert!(light > 0);
    }

    #[test]
    fn stress_horizontal_stripes() {
        let mut light = 0;
        for y in 0..100 {
            if horizontal_stripes(y, 10) {
                light += 1;
            }
        }
        assert!(light > 0);
    }
}


// the end