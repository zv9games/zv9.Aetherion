use godot::prelude::*;
use crate::godot4::api::engine::AetherionEngine;

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
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            engine: None,
            tick_count: 0,
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
