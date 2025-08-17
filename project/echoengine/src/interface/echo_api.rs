// echo_api.rs

//! 📡 Echo API
//! External interface for invoking EchoEngine rituals.
//! Designed for editors, tools, and host environments.

use crate::prelude::*;

/// Initialize the engine and return a runtime
pub fn init_engine(config: EngineConfig, use_3d: bool) -> Runtime {
    Runtime::new(config, use_3d)
}

/// Generate tiles and return them
pub fn generate_tiles(runtime: &mut Runtime) -> Vec<Tile> {
    runtime.cycle.advance(Phase::Generate);
    runtime.registry.tiles.values().cloned().collect()
}

/// Advance animation by one tick
pub fn animate(runtime: &mut Runtime) {
    runtime.cycle.advance(Phase::Animate);
    runtime.cycle.advance(Phase::Tick);
}

/// Flip the active dimension
pub fn flip_dimension(runtime: &mut Runtime) {
    runtime.cycle.advance(Phase::FlipDimension);
}

/// Query tiles by kind
pub fn query_by_kind(runtime: &Runtime, kind: TileKind) -> Vec<Tile> {
    runtime.registry.query_kind(kind).into_iter().cloned().collect()
}
