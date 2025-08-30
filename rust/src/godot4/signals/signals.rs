use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node)] // Removed `init` to avoid conflict
pub struct AetherionSignals {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for AetherionSignals {
    fn init(base: Base<Node>) -> Self {
        AetherionSignals { base }
    }

    fn ready(&mut self) {
        godot_print!("📡 AetherionSignals is ready.");
    }
}

#[godot_api]
impl AetherionSignals {
    #[signal]
    fn build_map_start();

    #[signal]
    fn map_building_status(status_message: GString);

    #[signal]
    fn generation_progress(percent: i32);

    #[signal]
    fn generation_complete(results: Dictionary);
}


