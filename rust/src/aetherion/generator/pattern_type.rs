//C:/ZV9/zv9.aetherion/rust/src/aetherion/generator/pattern_type.rs

/// ✅ Suggestions for aetherion/generator/pattern_type.rs

// 🔧 Add serialization support:
//     - `#[derive(Serialize, Deserialize)]`
//     - Useful for saving/loading pattern configs or exposing to Godot

// 🧩 Add display or identifier methods:
//     - `fn as_str(&self) -> &'static str`
//     - Useful for UI dropdowns, debugging, or editor integration

// 🚦 Add pattern registry or factory:
//     - Map `PatternType` to actual pattern functions from `patterns.rs`
//     - e.g. `fn resolve(&self) -> fn(x: i32, y: i32) -> bool`

// 📚 Document intended use of `blend_noise_and_pattern()`:
//     - Clarify how noise and pattern interact (e.g. masking, modulation)
//     - Consider exposing blend modes: AND, OR, XOR, etc.

/// Future: Add new pattern variants:
//     - DiagonalStripes, Spiral, Grid, PerlinNoise, CellularAutomata
//     - Expand enum to support more generative styles

// 🧪 Add unit tests for `blend_noise_and_pattern()`:
//     - Validate logic and edge cases (e.g. false/false, true/false, etc.)

// 🧼 Optional: Add pattern metadata:
//     - e.g. `fn complexity(&self) -> u8` or `fn is_symmetric(&self) -> bool`
//     - Useful for procedural tuning or adaptive generation


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    Checkerboard,
    Radial,
    XorFractal,
    VerticalStripes,
    HorizontalStripes,
}

pub fn blend_noise_and_pattern(noise: bool, pattern: bool) -> bool {
    noise && pattern // or customize this later
}

//end pattern_type.rs