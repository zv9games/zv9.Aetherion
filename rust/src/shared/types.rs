//C:/ZV9/zv9.aetherion/rust/src/shared/types.rs

/// ✅ Suggestions for shared/types.rs

// 🔧 Add display and formatting support:
//     - `impl std::fmt::Display for EntityId`
//     - Useful for logging, debugging, and UI overlays

// 🧩 Add semantic wrappers for other primitives:
//     - e.g. `pub struct Tick(pub u64)`, `pub struct Layer(pub u8)`
//     - Improves type safety and expressiveness across systems

// 🚦 Add validation or reserved ID logic:
//     - e.g. `EntityId::is_reserved()`, `EntityId::is_valid()`
//     - Enables special handling for system entities or null IDs

// 📚 Document usage patterns:
//     - Clarify where `Coord` and `Timestamp` are used (e.g. tilemaps, diagnostics)
//     - Note that `EntityId` is hashable and suitable for maps/sets

// 🧪 Add unit tests for `EntityId` methods:
//     - Validate `from_raw()` and `value()` round-trip
//     - Ensure equality and hashing behave as expected

// 🧼 Optional: Add conversion traits:
//     - `impl From<u64> for EntityId`, `impl Into<u64> for EntityId`
//     - Improves ergonomics and interop with external systems

// 🚀 Future: Add entity metadata or tagging:
//     - e.g. `pub struct TaggedEntityId { id: EntityId, tag: String }`
//     - Enables grouping, filtering, or debugging

// 🧠 Consider exposing these types to GDScript:
//     - Wrap in a Godot-friendly struct or export via utility node
//     - Useful for editor tooling or runtime scripting


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