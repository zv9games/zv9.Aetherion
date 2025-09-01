//! AetherionEngine Prelude — Common imports for ergonomic development.
//! Use this module to simplify access to core types, traits, and utilities across the engine.

// 🌐 Godot core types
pub use godot::prelude::*;
pub use godot::classes::{Node, TileMap};
pub use godot::global::Error;

// 🧠 Engine modules
pub use crate::godot4::api::engine::AetherionEngine;
pub use crate::godot4::signals::AetherionSignals;
pub use crate::godot4::messaging::{GodotSync, EngineMessage};
pub use crate::aetherion::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};

// 🧩 Shared traits and types
pub use crate::shared::traits::*;
pub use crate::shared::types::*;
pub use crate::shared::math::*;

// 🛠 Utilities
pub use crate::util::config::EngineConfig;
pub use crate::util::logging::*;
pub use crate::util::timing::TickTimer;

// 📦 External crates
pub use rand::{Rng, SeedableRng};
pub use rayon::prelude::*;
