use godot::prelude::*;
use godot::builtin::{VariantArray, Vector2i, Variant};
use crate::logging::{log, LogLevel};
use crate::changeover::ChangeOver;

/// 🗺️ MapBuilder — Rust-side tile placement ritual
#[derive(GodotClass, Debug)]
#[class(base=Node2D, init)]
pub struct MapBuilder {
    #[export]
    use_rust: bool,
    base: Base<Node2D>,
}

#[godot_api]
impl MapBuilder {
    fn init(base: Base<Node2D>) -> Self {
        log(LogLevel::Info, "🛠️ Initializing MapBuilder");
        Self {
            base,
            use_rust: true,
        }
    }

    /// 🚀 Entry point for expansive tile placement
    #[func]
    fn begin_expansive_build(&mut self, payload: VariantArray) {
        log(LogLevel::Info, &format!("🚀 Starting begin_expansive_build with {} entries", payload.len()));
        if !self.base.to_gd().is_instance_valid() {
            log(LogLevel::Error, "🚫 Base Node2D instance is null in begin_expansive_build");
            return;
        }
        log(LogLevel::Info, "✅ Base Node2D instance is valid");

        if self.use_rust && !payload.is_empty() {
            log(LogLevel::Info, "🧬 Using Rust pipeline for tile placement");
            self.place_tiles_rust(payload);
        } else {
            log(LogLevel::Warn, &format!(
                "🪜 Falling back to GDScript pipeline (use_rust: {}, payload_empty: {})",
                self.use_rust,
                payload.is_empty()
            ));
        }
    }

    /// 🪵 Rust-side tile placement logic
    fn place_tiles_rust(&self, payload: VariantArray) {
        log(LogLevel::Info, &format!("🪵 Starting place_tiles_rust with {} entries", payload.len()));
        if !self.base.to_gd().is_instance_valid() {
            log(LogLevel::Error, "🚫 Base Node2D instance is null in place_tiles_rust");
            return;
        }
        log(LogLevel::Info, "✅ Base Node2D instance is valid");

        let mut placed = 0;

        for i in 0..payload.len() {
            log(LogLevel::Info, &format!("🔍 Processing entry #{}", i));
            let entry = payload.get(i).unwrap_or(Variant::nil());
            if entry.is_nil() {
                log(LogLevel::Warn, &format!("⚠️ Entry #{} is nil — skipping", i));
                continue;
            }

            let arr = match entry.try_to::<VariantArray>() {
                Ok(arr) => arr,
                Err(e) => {
                    log(LogLevel::Warn, &format!("⚠️ Entry #{} is not an array, error: {} — skipping", i, e));
                    continue;
                }
            };
            log(LogLevel::Info, &format!("📋 Entry #{} is a valid array with {} elements", i, arr.len()));

            if arr.len() < 5 {
                log(LogLevel::Warn, &format!("⚠️ Malformed tile #{} with {} elements — skipping", i, arr.len()));
                continue;
            }

            let x = arr.get(0).and_then(|v| v.try_to::<i64>().ok()).unwrap_or(0) as i32;
            let y = arr.get(1).and_then(|v| v.try_to::<i64>().ok()).unwrap_or(0) as i32;
            let source_id = arr.get(2).and_then(|v| v.try_to::<i64>().ok()).unwrap_or(0) as i32;
            let atlas_coords = arr.get(3).and_then(|v| v.try_to::<Vector2i>().ok()).unwrap_or(Vector2i::ZERO);
            let alt_id = arr.get(4).and_then(|v| v.try_to::<i64>().ok()).unwrap_or(0) as i32;

            log(LogLevel::Info, &format!(
                "🧱 Tile #{}: pos=({},{}), source_id={}, atlas_coords={:?}, alt_id={}",
                i, x, y, source_id, atlas_coords, alt_id
            ));

            placed += 1;
        }

        log(LogLevel::Info, &format!("🪵 Rust placement complete — {} tiles set", placed));
        self.finalize_map_build(placed);
    }

    /// ✅ Finalize and emit signals
    #[func]
    fn finalize_map_build(&self, placed: i32) {
        log(LogLevel::Info, &format!("✅ MapBuilder ceremony complete — {} tiles placed", placed));
        if !self.base.to_gd().is_instance_valid() {
            log(LogLevel::Error, "🚫 Base Node2D instance is null in finalize_map_build");
            return;
        }

        // 🗺️ Emit map_builder ready signal
        self.base.to_gd().emit_signal("mb_ready", &[]);
        log(LogLevel::Info, "📡 Emitted mb_ready signal");

        // 🧱 Emit tile count for debug/overlay
        self.base.to_gd().emit_signal("tiles_placed", &[Variant::from(placed)]);
        log(LogLevel::Info, &format!("📡 Emitted tiles_placed signal with count {}", placed));
    }

    /// 🧱 Declare debug signal
    #[signal]
    fn tiles_placed(count: i32);
}
