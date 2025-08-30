/// Pattern-based generation logic (e.g. checkerboard, symmetry, fractals).

/// Simple checkerboard pattern.
pub fn checkerboard(x: i32, y: i32) -> bool {
    (x + y) % 2 == 0
}
