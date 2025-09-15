//C:/ZV9/zv9.aetherion/rust/src/shared/math.rs

// ✅ Suggestions for shared/math.rs

// 🔧 Add interpolation utilities:
//     - `fn lerp(a: f64, b: f64, t: f64) -> f64`
//     - `fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64`
//     - Useful for procedural transitions, animation, and terrain blending

// 🧩 Add vector math helpers:
//     - e.g. `fn distance(a: Vector2, b: Vector2) -> f64`
//     - `fn normalize(v: Vector2) -> Vector2`
//     - Enables spatial queries and directional logic

// 🚦 Add clamping variants:
//     - `fn clamp_f64(val: f64, min: f64, max: f64) -> f64`
//     - `fn clamp_i32(val: i32, min: i32, max: i32) -> i32`
//     - Improves ergonomics and avoids generic inference issues

// 📚 Document math conventions:
//     - Clarify coordinate system (e.g. top-left origin, unit scale)
//     - Note how TAU is used in rotation or wave functions

// 🧪 Add unit tests for clamp and future math functions:
//     - Validate edge cases, floating-point behavior, and overflow safety

// 🧼 Optional: Add angle conversion helpers:
//     - `fn deg_to_rad(deg: f64) -> f64`
//     - `fn rad_to_deg(rad: f64) -> f64`
//     - Useful for Godot interop and procedural rotation

// 🚀 Future: Add matrix and transform utilities:
//     - e.g. `fn rotate_point(...)`, `fn transform_2d(...)`
//     - Enables procedural placement, animation, and spatial logic

// 🧠 Consider exposing math constants:
//     - `pub const EPSILON: f64 = 1e-6`
//     - `pub const GOLDEN_RATIO: f64 = 1.61803398875`
//     - Useful for precision control and aesthetic generation


// Math utilities and constants shared across Aetherion.
// Includes vector math, matrix transforms, interpolation, and core constants.
#[allow(unused_imports)]
use crate::zv9_prelude::*;


// Full circle constant (τ = 2π).
pub const TAU: f64 = std::f64::consts::PI * 2.0;

// Clamps a value between a minimum and maximum bound.
//
// # Examples
// ```
// use crate::zv9_shared_math::clamp;

// let x = clamp(5, 0, 10); // returns 5
// let y = clamp(-3, 0, 10); // returns 0
// let z = clamp(42, 0, 10); // returns 10
// ```
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