// Core Godot types and prelude
use godot::prelude::*;
use godot::builtin::{VariantArray, Variant, Array, Vector2i};

// Macro sigil emitter — only the derive macro is imported
use godot_macros::GodotClass;

// Internal engine binding
use crate::changeover::ChangeOver;

/// 🎩 APIBot — The conductor of tile magic.
#[derive(GodotClass, Debug)]
#[class(init, base=Node)]
pub struct APIBot {
    #[base]
    base: Base<Node>,

    /// 🧱 The tile engine that performs the magic
    changeover: Option<ChangeOver>, // 🧩 Slavebot embedded

    /// 🔁 Is the flipper loop running?
    #[export]
    flipper_active: bool,

    /// 🌫️ Is the dissolve ritual allowed?
    #[export]
    can_dissolve: bool,
}


#[godot_api]
impl APIBot {
    fn new(base: Base<Node>) -> Self {
        godot_print!("🧪 APIBot::new() invoked.");
        Self {
            base,
            changeover: None,
            flipper_active: false,
            can_dissolve: false,
        }
    }

    #[signal]
    fn api_bot_ready();

    #[signal]
    fn transition_finished();

    #[func]
    pub fn set_changeover(&mut self, changeover: Gd<ChangeOver>) {
        godot_print!("🔗 Binding ChangeOver to APIBot.");
        self.changeover = Some(changeover);
    }

    #[func]
    pub fn mode_start(&mut self) {
        godot_print!("🧭 APIBot: Starting the ritual...");
        godot_print!("🔍 flipper_active: {}, can_dissolve: {}", self.flipper_active, self.can_dissolve);

        let Some(changeover) = self.changeover.as_mut() else {
            godot_warn!("❌ Ritual failed: ChangeOver not bound.");
            return;
        };

        godot_print!("🖤 Calling fill_black() on ChangeOver...");
        let veil = changeover
            .call("fill_black", &[])
            .try_to::<VariantArray>()
            .unwrap_or_else(|_| {
                godot_warn!("⚠️ fill_black() returned invalid data.");
                VariantArray::new()
            });

        godot_print!("🖤 Veil contains {} tiles.", veil.len());
        self.base.to_gd().call_deferred("apply_updates", &[Variant::from(veil)]);

        self.flipper_active = true;
        godot_print!("🔁 Flipper activated. Tiles will begin to dance!");

        godot_print!("📣 Scheduling deferred signal: api_bot_ready");
        let timer = SceneTree::singleton().create_timer(0.1);
        timer.connect("timeout", self.base.to_gd(), "emit_api_bot_ready", VariantArray::new(), 0);
    }

    #[func]
    fn emit_api_bot_ready(&mut self) {
        godot_print!("📣 Emitting signal: api_bot_ready");
        self.base.to_gd().emit_signal("api_bot_ready", &[]);
    }

    #[func]
    pub fn mode_loaded(&mut self) {
        godot_print!("🌫️ APIBot: Preparing to reveal...");
        godot_print!("🔍 flipper_active: {}, can_dissolve: {}", self.flipper_active, self.can_dissolve);

        if !self.flipper_active || !self.can_dissolve {
            godot_print!("⚠️ Ritual not ready. Flipper must be active and dissolve allowed.");
            return;
        }

        let Some(changeover) = self.changeover.as_mut() else {
            godot_warn!("❌ Ritual failed: ChangeOver not bound.");
            return;
        };

        self.flipper_active = false;
        self.can_dissolve = false;
        godot_print!("🛑 Flipper deactivated. Dissolve proceeding...");

        let width = changeover
            .call("get_grid_width", &[])
            .try_to::<i32>()
            .unwrap_or_else(|_| {
                godot_warn!("⚠️ get_grid_width() failed.");
                0
            });

        let height = changeover
            .call("get_grid_height", &[])
            .try_to::<i32>()
            .unwrap_or_else(|_| {
                godot_warn!("⚠️ get_grid_height() failed.");
                0
            });

        godot_print!("📏 Grid dimensions: {} x {}", width, height);

        let mut tiles = Array::new();
        for x in 0..width {
            for y in 0..height {
                let pos = Vector2i::new(x, y);
                tiles.push(&Variant::from(pos));
            }
        }

        godot_print!("🌫️ Dissolving {} tiles...", tiles.len());

        let dissolve_frame = changeover
            .call("dissolve_tiles", &[Variant::from(tiles)])
            .try_to::<VariantArray>()
            .unwrap_or_else(|_| {
                godot_warn!("⚠️ dissolve_tiles() returned invalid data.");
                VariantArray::new()
            });

        godot_print!("✅ Dissolve frame contains {} updates.", dissolve_frame.len());
        self.base.to_gd().call_deferred("apply_updates", &[Variant::from(dissolve_frame)]);

        godot_print!("📣 Emitting signal: transition_finished");
        self.base.to_gd().emit_signal("transition_finished", &[]);
    }

    #[func]
    pub fn ritual_trace(&self) {
        godot_print!("📜 Ritual Trace:");
        godot_print!("🔗 ChangeOver bound: {}", self.changeover.is_some());
        godot_print!("🔁 Flipper active: {}", self.flipper_active);
        godot_print!("🌫️ Can dissolve: {}", self.can_dissolve);
    }
}

// ✅ Trait logic — outside macro scope
#[godot_api]
impl INode for APIBot {}
