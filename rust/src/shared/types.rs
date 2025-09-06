//C:/ZV9/zv9.aetherion/rust/src/shared/types.rs

//! Common type aliases and primitive wrappers used across Aetherion.
//! These types provide semantic clarity and lightweight abstraction.

/// Grid coordinate (x, y) used for tilemaps and spatial indexing.
pub type Coord = (i32, i32);

/// Timestamp in milliseconds or ticks, depending on context.
pub type Timestamp = u64;

/// Wrapper for a unique entity ID.
/// Used for identifying procedural objects, runtime actors, or map elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

impl EntityId {
    /// Returns the raw ID value.
    pub fn value(self) -> u64 {
        self.0
    }

    /// Returns a new EntityId from a raw value.
    pub fn from_raw(id: u64) -> Self {
        EntityId(id)
    }
}

//end types.rs