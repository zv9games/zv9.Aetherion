//C:/ZV9/zv9.aetherion/rust/src/aetherion/core/dimension.rs

/// ✅ Suggestions for aetherion/core/dimension.rs

// 🔧 Add support for dynamic dimension switching:
//     - Consider a `DimensionContext` struct to manage current state and transitions
//     - Useful for hybrid games or editor modes

// 🧩 Add integration hooks:
//     - `fn to_godot_layer()` or `fn to_render_mode()`
//     - Useful for mapping dimension to engine-specific rendering or physics layers

// 🚦 Add validation or constraints:
//     - Prevent switching dimensions mid-frame unless explicitly allowed
//     - Could be tied to lifecycle or runtime state

// 📚 Add documentation for intended use cases:
//     - Clarify how this enum interacts with tilemaps, meshes, and procedural systems
//     - Could include examples in doc comments

// 🧪 Add unit tests for `flipped()` and `as_str()`
//     - Ensure consistent behavior across future refactors

// 🧼 Optional: Add serialization support
//     - `#[derive(Serialize, Deserialize)]` if needed for config or save files

// 🚀 Future: Expand to support higher dimensions or projection modes
//     - e.g. `FourD`, `Isometric`, `Orthographic`, `Perspective`



//! Handles abstraction between 2D and 3D dimensions.
//! Will evolve to support dynamic switching, shared logic, and editor integration.

use std::fmt;

/// Represents the dimensionality of the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    /// Two-dimensional space (tilemaps, flat terrain).
    TwoD,

    /// Three-dimensional space (volumetric terrain, meshes).
    ThreeD,
}

impl Dimension {
    /// Returns true if the dimension is 2D.
    pub fn is_2d(&self) -> bool {
        matches!(self, Dimension::TwoD)
    }

    /// Returns true if the dimension is 3D.
    pub fn is_3d(&self) -> bool {
        matches!(self, Dimension::ThreeD)
    }

    /// Returns a human-readable name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Dimension::TwoD => "2D",
            Dimension::ThreeD => "3D",
        }
    }

    /// Returns the opposite dimension.
    pub fn flipped(&self) -> Self {
        match self {
            Dimension::TwoD => Dimension::ThreeD,
            Dimension::ThreeD => Dimension::TwoD,
        }
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::TwoD
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


//end dimension.rs