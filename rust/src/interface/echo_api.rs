//! 📡 Echo API
//! External interface for invoking EchoEngine rituals.
//! Designed for editors, tools, and host environments.
//! Not directly exposed to Godot—wrap in RuntimeHandle for ABI-safe access.

use crate::prelude::*;
use crate::GodotClass;
use crate::godot_api;
use crate::engine::runtime::Runtime;

/// Initializes the engine and returns a runtime instance.
/// Used by external tools and wrappers.
pub fn init_engine(config: EngineConfig, use_3d: bool) -> Runtime {
    Runtime::new(config, use_3d)
}

/// Advances the engine to the Generate phase and returns all generated tiles.
pub fn generate_tiles(runtime: &mut Runtime) -> Vec<Tile> {
    runtime.cycle.advance(Phase::Generate);
    runtime.cycle.registry.tiles.values().cloned().collect()
}

/// Advances animation and tick phases.
pub fn animate(runtime: &mut Runtime) {
    runtime.cycle.advance(Phase::Animate);
    runtime.cycle.advance(Phase::Tick);
}

/// Flips the active dimension.
pub fn flip_dimension(runtime: &mut Runtime) {
    runtime.cycle.advance(Phase::FlipDimension);
}

/// Queries tiles by kind.
pub fn query_by_kind(runtime: &Runtime, kind: TileKind) -> Vec<Tile> {
    runtime.cycle.registry.query_kind(kind).into_iter().cloned().collect()
}

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct EchoApi;

#[godot_api]
impl EchoApi {}