// prelude.rs

//! 📜 EchoEngine Prelude
//! Canonical imports for ergonomic access to core engine components.
//! Use `use echo_engine::prelude::*` to onboard rituals.

pub use crate::engine::dimension::{BotFlipper, DimensionContext, Position, Dim2, Dim3};
pub use crate::engine::types::{Tile, TileKind, TileMeta};
pub use crate::engine::generator::{generate_field, TileField};
pub use crate::engine::animator::{Animator, AnimationKind, TileAnimation};
pub use crate::engine::registry::Registry;
pub use crate::engine::lifecycle::{Lifecycle, Phase, EngineLifecycle};
pub use crate::engine::runtime::Runtime;

pub use crate::utils::config::EngineConfig;
pub use crate::utils::time::{Tick, Duration};
