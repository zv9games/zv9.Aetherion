//! 🚪 Interface Manifest — Ritual Entry Point
//! Binds together all interface components for EchoEngine invocation.
//!
//! Each submodule is a ceremonial beat in the invocation sequence:
//! - 🔌 `adapter`: bridges engine internals with host environments
//! - 📣 `signal`: routes lifecycle echoes and tile updates
//! - 📡 `echo_api`: exposes frame-safe rituals for external tools
//! - 🪢 `bindings`: wraps engine types for Godot and FFI-safe access
//!
//! This manifest supports modular, thread-safe invocation from Godot or CLI.
//! Ensure all exposed methods avoid blocking and honor the separation rite.

pub mod adapter;     // 🔌 Host adapter layer
pub mod signal;      // 📣 Signal routing and echo propagation
pub mod echo_api;    // 📡 Public-facing API for external invocation
pub mod bindings;    // 🪢 Godot and FFI-safe type wrappers

pub use adapter::*;
pub use signal::*;
pub use echo_api::*;
pub use bindings::*;
