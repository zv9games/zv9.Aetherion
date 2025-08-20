#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]

//! 📚 Aetherion Engine Core Library
//! Modular, introspectable, legacy-friendly procedural engine for Godot.
//!
//! 🔧 Exposes audit, dimension, signal, lifecycle, and overlay systems.
//! 🧭 Mounts silently on Godot startup, initializing runtime ticker and observatory window.
//! 🪶 Native debugger window (via `egui`) runs in parallel, displaying tick count, phase glyphs, and echo logs.
//!
//! When the temple opens, the echo listens.
//! When the glyph is drawn, the observatory opens.

use godot::prelude::*;
use crate::interface::echo_api::EchoApi;
use std::process::Command;


// --- Modules ---
pub mod engine {
    pub mod animator;
    pub mod dimension;
    pub mod generator;
    pub mod lifecycle;
    pub mod prelude;
    pub mod registry;
    pub mod runtime;
    pub mod types;
}

pub mod interface {
    pub mod signal;
    pub mod echo_api;
    pub mod bindings;
    pub mod adapter;
}

pub mod audit {
    pub mod logger;
    pub mod annotation;
    pub mod overlay;
    pub mod manifest;
    pub mod debugger; // 🧿 Native observatory window
}

pub mod utils {
    pub mod config;
    pub mod threading;
    pub mod mapper;
    pub mod helpers;
    pub mod time;
}

/// Prelude — ceremonial import bundle
pub mod prelude {
    pub use crate::engine::types::*;
    pub use crate::interface::signal::*;
    pub use crate::engine::dimension::*;
    pub use crate::engine::lifecycle::*;
    pub use crate::audit::manifest::*;
    pub use crate::audit::annotation::*;
    pub use crate::audit::overlay::*;
    pub use crate::utils::config::*;
    pub use crate::utils::helpers::*;
}

// --- GDExtension Entry Point ---
#[gdextension]
unsafe impl ExtensionLibrary for EchoApi {}
