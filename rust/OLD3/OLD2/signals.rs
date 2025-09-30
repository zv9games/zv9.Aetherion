use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionSignals {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl AetherionSignals {
    #[signal]
    fn build_map_start();

    #[signal]
    fn map_building_status(status_message: String);

    #[signal]
    fn generation_progress(percent: i32);

    #[signal]
    fn generation_complete(results: Dictionary);

    #[func]
    fn _ready(&mut self) {
        godot_print!("📡 AetherionSignals is ready.");
    }

    #[func]
    pub fn emit_build_map_start(&mut self) {
        self.base.to_init_gd().emit_signal("build_map_start", &[]);
        godot_print!("🚀 Emitted 'build_map_start' signal.");
    }

    #[func]
    pub fn emit_map_building_status(&mut self, status_message: String) {
        self.base.to_init_gd().emit_signal("map_building_status", &[status_message.to_variant()]);
        godot_print!("📢 Status update: {}", status_message);
    }

    #[func]
    pub fn emit_generation_progress(&mut self, percent: i32) {
        self.base.to_init_gd().emit_signal("generation_progress", &[percent.to_variant()]);
        godot_print!("📊 Progress: {}%", percent);
    }

    #[func]
    pub fn emit_generation_complete(&mut self, results: Dictionary) {
        self.base.to_init_gd().emit_signal("generation_complete", &[results.to_variant()]);
        godot_print!("✅ Emitted 'generation_complete' signal.");
    }
}