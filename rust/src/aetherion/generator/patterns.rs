//C:/ZV9/zv9.aetherion/rust/src/aetherion/generator/patterns.rs

/// ✅ Suggestions for aetherion/generator/patterns.rs

// 🔧 Add parameterized pattern variants:
//     - e.g. `checkerboard_offset(x, y, offset_x, offset_y)`
//     - Useful for shifting or rotating patterns dynamically

// 🧩 Add composite pattern support:
//     - Combine multiple patterns with logical operators
//     - e.g. `checkerboard(x, y) && radial(x, y, r)`
//     - Could expose a `PatternFn` trait or boxed closures

// 🚦 Add bounds checking or normalization:
//     - Optional: clamp or wrap coordinates for edge-safe generation
//     - Useful for procedural maps with toroidal wrapping

// 📚 Add documentation examples:
//     - Show visual or textual examples of each pattern
//     - Could include ASCII previews or usage notes

// 🧪 Add unit tests for each pattern:
//     - Validate expected output for known coordinates
//     - Ensure symmetry and repeatability

// 🧼 Optional: Add pattern metadata or descriptors:
//     - e.g. `fn name() -> &'static str` for each pattern
//     - Useful for editor integration or debugging

// 🚀 Future: Add noise-based or Perlin-style patterns:
//     - e.g. `fn perlin_noise(x, y, seed) -> bool`
//     - Enables organic terrain shaping and biome blending


//! Pattern-based generation logic for tile placement and terrain shaping.
//! Includes checkerboard, symmetry, fractals, and stripe-based patterns.

/// Simple checkerboard pattern.
/// Returns true if the tile at (x, y) should be marked.
pub fn checkerboard(x: i32, y: i32) -> bool {
    (x + y) % 2 == 0
}

/// Radial symmetry pattern centered at origin.
/// Returns true if the tile is within a circular radius.
pub fn radial(x: i32, y: i32, radius: f32) -> bool {
    let dist_sq = (x * x + y * y) as f32;
    dist_sq <= radius * radius
}

/// Fractal-like pattern using bitwise XOR.
/// Creates a chaotic grid of toggled tiles.
pub fn xor_fractal(x: i32, y: i32) -> bool {
    (x ^ y) % 7 == 0
}

/// Vertical stripes pattern.
/// Returns true if the tile is in a "light" stripe.
pub fn vertical_stripes(x: i32, stripe_width: i32) -> bool {
    (x / stripe_width) % 2 == 0
}

/// Horizontal stripes pattern.
/// Returns true if the tile is in a "light" stripe.
pub fn horizontal_stripes(y: i32, stripe_height: i32) -> bool {
    (y / stripe_height) % 2 == 0
}

//end patterns.rs