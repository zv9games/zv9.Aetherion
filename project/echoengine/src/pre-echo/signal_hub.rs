// src/signal_hub.rs

use godot::prelude::*;

pub fn register_signals(handle: &mut InitHandle) {
    // ─── Core Ritual Signals ─────────────────────────────

    // 🗺️ map_builder.rs — Emitted when map generation completes
    handle.add_signal::<()>("mb_ready");

    // 🎬 changeover.rs — Emitted when visual transition completes
    handle.add_signal::<()>("transition_done");

    // 📣 api_bot.rs — Emitted when APIBot ritual begins
    handle.add_signal::<()>("api_bot_ready");

    // 🌫️ api_bot.rs — Emitted when dissolve animation finishes
    handle.add_signal::<()>("transition_finished");

    // ─── Module Completion Signals ───────────────────────

    // 🖼️ image_processor.rs — Emitted when image processing finishes
    handle.add_signal::<()>("ip_ready");

    // 🧠 expansive.rs — Emitted when expansive logic completes
    handle.add_signal::<()>("expansive_ready");

    // 🏷️ annotation_loader.rs — Emitted when annotation parsing finishes
    handle.add_signal::<()>("al_ready");

    // 🛠️ editor.rs — Emitted when editor setup completes
    handle.add_signal::<()>("editor_ready");

    // ─── Debug & Trace Signals ───────────────────────────

    // 🖤 changeover.rs — Emitted after veil applied (tile count)
    handle.add_signal::<i32>("veil_applied");

    // 🔁 changeover.rs — Emitted after flip_tiles (update count)
    handle.add_signal::<i32>("tiles_flipped");

    // 🌫️ changeover.rs — Emitted per tile dissolved
    handle.add_signal::<Vector2i>("tile_dissolved");

    // 📜 api_bot.rs — Emits internal state snapshot
    handle.add_signal::<String>("ritual_trace");

    // 🖼️ image_processor.rs — Emits image filename or asset ID
    handle.add_signal::<String>("image_loaded");

    // 🏷️ annotation_loader.rs — Emits count of parsed annotations
    handle.add_signal::<i32>("annotations_parsed");

    // ⚙️ map_builder.rs — Emits config mode or preset name
    handle.add_signal::<String>("config_applied");

    // ─── Threading & Finalization ────────────────────────

    // 🧵 threading.rs — Signals async generation start
    handle.add_signal::<()>("thread_started");

    // ❌ threading.rs — Emits error message on thread failure
    handle.add_signal::<String>("thread_failed");

    // 🧩 tile_smasher.rs — Final signal before gameplay begins
    handle.add_signal::<()>("engine_ready");
}
