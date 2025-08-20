//! 📡 Echo API
//! External interface for invoking EchoEngine rituals.
//!
//! 🛠️ Designed for editors, tools, and host environments.
//! 🧩 Not directly exposed to Godot—wrap in `EchoApi` for ABI-safe access.

use godot::prelude::{GodotClass, godot_api, Base, Object};
use crate::prelude::{EngineConfig, Tile, TileKind};
use crate::engine::lifecycle::{Lifecycle, Phase};
use crate::engine::runtime::Runtime;
use std::cell::{RefCell, Ref};
use std::process::Command;

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

// --- Godot ABI Wrapper ---
#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct EchoApi {
    runtime: RefCell<Option<Runtime>>,
}

#[godot_api]
impl EchoApi {
    /// 🔧 Initializes runtime when mounted by Godot
    fn init(_base: Base<Object>) -> Self {
        let config = EngineConfig::default();
        let runtime = Runtime::new(config, true);

        // 🪶 Attempt to launch native debugger window
        let exe_path = std::env::current_dir()
            .unwrap_or_else(|_| ".".into())
            .join("target")
            .join("release")
            .join(if cfg!(windows) { "debugger.exe" } else { "debugger" });

        println!("🧿 Launching debugger at {:?}", exe_path);

        if let Err(e) = Command::new(exe_path).spawn() {
            eprintln!("🧿 Debugger launch failed: {:?}", e);
        }

        Self {
            runtime: RefCell::new(Some(runtime)),
        }
    }

    /// 🔒 Internal helper — not exposed to Godot
    fn runtime_handle(&self) -> Ref<Runtime> {
        Ref::map(self.runtime.borrow(), |opt| {
            opt.as_ref().expect("Runtime not initialized")
        })
    }

    /// 📡 Godot-facing method — safe and callable
    #[func]
    fn get_debug_output(&self) -> String {
        let runtime = self.runtime_handle();
        let tick = runtime.cycle.tick;
        let tile_count = runtime.cycle.registry.tiles.len();

        format!("Tick: {}\nTiles: {}", tick, tile_count)
    }

    /// 🧪 Optional: Manual runtime reboot from Godot
    #[func]
    fn init_runtime(&mut self) {
        let config = EngineConfig::default();
        let runtime = Runtime::new(config, true);
        self.runtime.replace(Some(runtime));
    }
}
