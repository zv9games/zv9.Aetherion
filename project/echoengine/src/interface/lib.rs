// interface/lib.rs

//! 🧩 EchoEngine Interface
//! Public API surface for external tools, games, and editors.
//! Exposes ritual-safe components via the canonical prelude.

#![deny(missing_docs)]

/// Canonical prelude for ergonomic access
pub use crate::prelude::*;

/// Interface modules
pub mod adapter;
pub mod signal;
pub mod overlay;
pub mod ritual;

/// Engine entrypoint (optional)
pub fn start_engine(config: EngineConfig, use_3d: bool, ticks: u64) {
    let mut runtime = Runtime::new(config, use_3d);
    runtime.run(ticks);
}
