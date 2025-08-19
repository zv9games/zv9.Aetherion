// src/interface/mod.rs

//! Interface Manifest — Ritual Entry Point
//! This module binds together all interface components.
//! Each submodule is a ceremonial beat in the engine’s invocation.

pub mod adapter;     // Handles external bindings and translation layers
pub mod signal;      // Manages signal routing and echo propagation
pub mod echo_api;    // Public-facing API for echo-based interactions
pub mod bindings;    // FFI or external system bindings (e.g., Godot)

pub use adapter::*;
pub use signal::*;
pub use echo_api::*;
pub use bindings::*;
