// lib.rs

//! 📚 EchoEngine Core Library
//! Modular, introspectable, legacy-friendly procedural engine for Godot.
//! Exposes audit, dimension, signal, lifecycle, and overlay systems.

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]

pub mod engine {
    pub mod types;
    pub mod signal;
    pub mod dimension;
    pub mod lifecycle;
}

pub mod audit {
    pub mod manifest;
    pub mod annotation;
    pub mod overlay;
}

pub mod utils {
    pub mod config;
    pub mod threading;
    pub mod mapper;
    pub mod helpers;
}

pub mod echo {
    pub mod logger;
    pub mod ritual;
    pub mod flipper; // Bot Flipper keystone
}

/// Prelude — ceremonial import bundle
pub mod prelude {
    pub use crate::engine::types::*;
    pub use crate::engine::signal::*;
    pub use crate::engine::dimension::*;
    pub use crate::engine::lifecycle::*;
    pub use crate::audit::manifest::*;
    pub use crate::audit::annotation::*;
    pub use crate::audit::overlay::*;
    pub use crate::utils::config::*;
    pub use crate::utils::helpers::*;
    pub use crate::echo::flipper::*;
}
