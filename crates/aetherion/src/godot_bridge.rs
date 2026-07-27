//! Godot 4 GDExtension entry (feature = "godot").
//!
//! Minimal bridge: proves the cdylib loads. Host tile flood / Plan B renderer
//! land in later milestones (see SSXL-ext ashes).

use godot::prelude::*;

struct AetherionExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {}

/// Root engine node exposed to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct AetherionEngine {
    base: Base<Node>,
    /// Simple heartbeat counter for demos.
    ticks: u64,
}

#[godot_api]
impl INode for AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        godot_print!("[Aetherion] engine node init ({})", crate::version_string());
        Self { base, ticks: 0 }
    }

    fn ready(&mut self) {
        godot_print!("[Aetherion] ready — health={}", crate::health());
    }

    fn process(&mut self, _delta: f64) {
        self.ticks = self.ticks.wrapping_add(1);
    }
}

#[godot_api]
impl AetherionEngine {
    /// Returns a version string for GDScript/C#.
    #[func]
    fn get_version(&self) -> GString {
        GString::from(crate::version_string().as_str())
    }

    /// Process tick counter (demo / smoke).
    #[func]
    fn get_ticks(&self) -> u64 {
        self.ticks
    }
}
