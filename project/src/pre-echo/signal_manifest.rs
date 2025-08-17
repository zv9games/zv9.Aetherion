// src/signal_manifest.rs

/// 🚀 SignalManifest — The ceremonial codex of all engine signals.
/// Each entry defines the signal name, its origin script, expected listeners,
/// and whether it’s used for debug/trace purposes only.

pub struct SignalManifestEntry {
    pub name: &'static str,
    pub origin: &'static str,
    pub listeners: &'static [&'static str],
    pub debug_only: bool,
}

pub const SIGNALS: &[SignalManifestEntry] = &[
    // ─── Core Rituals ─────────────────────────────
    SignalManifestEntry {
        name: "mb_ready",
        origin: "map_builder.rs",
        listeners: &["tile_smasher", "editor_overlay"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "transition_done",
        origin: "changeover.rs",
        listeners: &["api_bot", "tile_smasher"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "api_bot_ready",
        origin: "api_bot.rs",
        listeners: &["tile_smasher"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "transition_finished",
        origin: "api_bot.rs",
        listeners: &["tile_smasher", "editor_overlay"],
        debug_only: false,
    },

    // ─── Module Completion ───────────────────────
    SignalManifestEntry {
        name: "ip_ready",
        origin: "image_processor.rs",
        listeners: &["map_builder"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "expansive_ready",
        origin: "expansive.rs",
        listeners: &["map_builder"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "al_ready",
        origin: "annotation_loader.rs",
        listeners: &["map_builder"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "editor_ready",
        origin: "editor.rs",
        listeners: &["tile_smasher"],
        debug_only: false,
    },

    // ─── Debug & Trace ───────────────────────────
    SignalManifestEntry {
        name: "veil_applied",
        origin: "changeover.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "tiles_flipped",
        origin: "changeover.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "tile_dissolved",
        origin: "changeover.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "ritual_trace",
        origin: "api_bot.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "image_loaded",
        origin: "image_processor.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "annotations_parsed",
        origin: "annotation_loader.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "config_applied",
        origin: "map_builder.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "tiles_placed",
        origin: "map_builder.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "tile_updated",
        origin: "map_builder.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "map_checksum",
        origin: "map_builder.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
    SignalManifestEntry {
        name: "signal_echo",
        origin: "universal",
        listeners: &["debug_overlay"],
        debug_only: true,
    },

    // ─── Threading & Finalization ────────────────
    SignalManifestEntry {
        name: "thread_started",
        origin: "threading.rs",
        listeners: &["tile_smasher"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "thread_failed",
        origin: "threading.rs",
        listeners: &["tile_smasher", "debug_overlay"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "engine_ready",
        origin: "tile_smasher.rs",
        listeners: &["scene_root"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "engine_paused",
        origin: "tile_smasher.rs",
        listeners: &["input_manager", "debug_overlay"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "engine_resumed",
        origin: "tile_smasher.rs",
        listeners: &["input_manager", "debug_overlay"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "mode_changed",
        origin: "tile_smasher.rs",
        listeners: &["scene_root", "debug_overlay"],
        debug_only: false,
    },
    SignalManifestEntry {
        name: "engine_trace",
        origin: "tile_smasher.rs",
        listeners: &["debug_overlay"],
        debug_only: true,
    },
];
