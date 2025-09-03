use godot::prelude::*;
use crate::godot4::api::engine::AetherionEngine;

/// Node wrapper for manually driving the AetherionEngine.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionOracle {
    #[base]
    base: Base<Node>,
    engine: Option<Gd<AetherionEngine>>,
}

#[godot_api]
impl AetherionOracle {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            engine: None,
        }
    }

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
                godot_print!("🔮 Oracle: Delivering pulse to engine...");
                engine.call("aetherionoracle", &[]);
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
}
