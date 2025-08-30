//! Math utilities and constants shared across Aetherion

pub mod vector;
pub mod matrix;
pub mod interpolation;

pub const TAU: f64 = std::f64::consts::PI * 2.0;

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min { min } else if val > max { max } else { val }
}
