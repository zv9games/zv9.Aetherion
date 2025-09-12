/// ✅ Suggestions for util/velocity.rs

// 🔧 Add directional helpers:
//     - `fn is_zero(&self) -> bool`
//     - `fn direction(&self) -> (f32, f32)` or normalized vector
//     - Useful for movement logic and AI steering

// 🧩 Add integration with `Vector2` or `Vector2i`:
//     - `fn to_vector2(&self) -> Vector2`
//     - Enables Godot interop and physics calculations

// 🚦 Improve precision handling:
//     - Consider using `f64` internally if high-precision movement is needed
//     - Prevents rounding errors in long simulations

// 📚 Document coordinate assumptions:
//     - Clarify whether velocity is in pixels, tiles, or units
//     - Note how `apply()` handles float-to-int conversion

// 🧪 Add unit tests for `apply`, `scale`, and `increase`:
//     - Validate rounding behavior and edge cases
//     - Ensure consistent movement across frames

// 🧼 Optional: Add arithmetic operators:
//     - `impl Add for Velocity`, `impl Mul<f32>`
//     - Improves ergonomics for physics and animation systems

// 🚀 Future: Add acceleration or damping support:
//     - `fn apply_acceleration(&mut self, accel: Velocity)`
//     - Enables smoother motion and physics-style updates

// 🧠 Consider exposing velocity to GDScript:
//     - Wrap in a Godot-friendly struct or export via utility node
//     - Useful for runtime control or editor tweaking


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Velocity { dx, dy }
    }

    pub fn zero() -> Self {
        Velocity { dx: 0.0, dy: 0.0 }
    }

    pub fn apply(&self, position: &mut crate::util::Position) {
        position.x += self.dx as i32;
        position.y += self.dy as i32;
    }
	
	pub fn scale(&self, factor: f64) -> Velocity {
        Velocity {
            dx: (self.dx as f64 * factor) as f32,
            dy: (self.dy as f64 * factor) as f32,
        }
    }

    pub fn increase(&mut self, amount: f64) {
        self.dx += amount as f32;
        self.dy += amount as f32;
    }
}
