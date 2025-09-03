// Godot-facing API modules for engine integration and procedural control.
pub mod engine;
pub mod signals;
pub mod generator;
pub mod config;
pub mod map;
pub mod oracle;

// Public exports for external use in Godot bindings and runtime orchestration.
pub use engine::AetherionEngine;
pub use signals::AetherionSignals;
pub use generator::AetherionGenerator;
pub use config::AetherionConfig;
pub use map::AetherionMap;
