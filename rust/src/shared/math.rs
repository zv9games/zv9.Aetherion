//C:/ZV9/zv9.aetherion/rust/src/shared/math.rs

//! Math utilities and constants shared across Aetherion.
//! Includes vector math, matrix transforms, interpolation, and core constants.


/// Full circle constant (τ = 2π).
pub const TAU: f64 = std::f64::consts::PI * 2.0;

/// Clamps a value between a minimum and maximum bound.
///
/// # Examples
/// ```
/// use crate::aetherion_engine::shared::math::clamp;
/// let x = clamp(5, 0, 10); // returns 5
/// let y = clamp(-3, 0, 10); // returns 0
/// let z = clamp(42, 0, 10); // returns 10
/// ```
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
//end math.rs