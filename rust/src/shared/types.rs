//! Common type aliases and primitive wrappers

pub type Coord = (i32, i32);
pub type Timestamp = u64;

/// Wrapper for a unique entity ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);
