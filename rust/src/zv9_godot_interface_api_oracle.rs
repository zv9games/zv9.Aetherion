//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_godot_interface_api_oracle.rs

// 🔧 Add lifecycle hooks:
//     - `pause()`, `resume()`, `shutdown()` for controlling engine flow
//     - Useful for editor control or runtime toggling

// 🧩 Add tick pacing or scheduling:
//     - e.g. `tick_interval_ms`, `auto_tick_enabled`
//     - Enables timed updates or frame-synced behavior

// 🚦 Add error handling for engine calls:
//     - Validate `engine.call("tick", ...)` result
//     - Emit warning or fallback if call fails

// 📚 Document Oracle’s role:
//     - Clarify that it manually drives the engine from GDScript
//     - Note how it differs from signal-driven or autonomous systems

// 🧪 Add unit tests or mock engine integration:
//     - Validate tick progression, engine linkage, and reset behavior

// 🧼 Optional: Add debug summary or status query:
//     - `fn describe(&self) -> String`
//     - Useful for diagnostics or UI overlays

// 🚀 Future: Add multi-engine support or switching:
//     - e.g. `fn set_engine_by_id(id: &str)`
//     - Enables modular pipelines or runtime reconfiguration

// 🧠 Consider exposing tick hooks or callbacks:
//     - e.g. `fn on_tick(callback: Callable)`
//     - Useful for scripting or plugin systems


use godot::prelude::*;
use crate::zv9_prelude::*;

/// Node wrapper for manually driving the AetherionEngine.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionOracle {
    #[base]
    base: Base<Node>,
    engine: Option<Gd<AetherionEngine>>,
    tick_count: u64,
}

#[godot_api]
impl AetherionOracle {
	#[allow(dead_code)]
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            engine: None,
            tick_count: 0,
        }
    }
	#[allow(dead_code)]
    fn ready(&mut self) {
        godot_print!("🔮 Oracle is online. I await the ignition.");
        self.base.to_init_gd().set_process(true);
    }
	
    #[func]
    pub fn set_engine(&mut self, engine: Gd<AetherionEngine>) {
        self.engine = Some(engine);
        godot_print!("🔗 Oracle: Engine link established.");
    }
	
    #[func]
    pub fn tick(&mut self) {
        match self.engine.as_mut() {
            Some(engine) => {
                godot_print!("🔮 Oracle: Tick {} → Engine", self.tick_count);
                engine.call("tick", &[Variant::from(self.tick_count)]);
                self.tick_count += 1;
            }
            None => {
                godot_warn!("⚠️ Oracle: No engine linked. Tick aborted.");
            }
        }
    }

    #[func]
    pub fn ping(&self) {
        godot_print!("🔮 Oracle: Ping received. I am awake.");
    }

    #[func]
    pub fn reset(&mut self) {
        self.tick_count = 0;
        godot_print!("🔄 Oracle: Tick counter reset.");
    }

    #[func]
    pub fn get_tick(&self) -> u64 {
        self.tick_count
    }
}

// the end