
/// Placeholder for manual class registration.
/// This file is inactive unless macro-based registration fails or dynamic registration is needed.
/// To activate, uncomment the lines below and ensure `InitHandle` is in scope.

/// use godot::init::InitHandle;
/// use crate::godot_bridge::commands::AetherionEngine;
/// use crate::godot_bridge::signals::AetherionSignals;

/// pub fn register_classes(handle: &mut InitHandle) {
///     godot_print!("📦 Registering core classes...");
///     handle.add_class::<AetherionEngine>();
///     handle.add_class::<AetherionSignals>();
/// }
use godot::prelude::*;
