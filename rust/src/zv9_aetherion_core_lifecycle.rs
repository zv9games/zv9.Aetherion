//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_core_lifecycle.rs

// ✅ Suggestions for aetherion/core/lifecycle.rs

// 🔧 Add intermediate states if needed:
//     - Paused: for gameplay freeze or menu overlays
//     - Error: for unrecoverable runtime faults
//     - Restarting: for hot reload or engine reset

// 🧩 Consider adding timestamps or durations:
//     - Track when each state was entered
//     - Useful for diagnostics, profiling, or analytics

// 🚦 Add transition validation:
//     - Prevent invalid transitions (e.g. Terminated → Running)
//     - Consider a method like `can_transition_to(target: LifecycleState) -> bool`

// 📡 Integrate with signal system:
//     - Emit signals on state changes (already commented)
//     - Could hook into AetherionSignals or RuntimeState

// 🧪 Add unit tests:
//     - Validate state transitions
//     - Ensure lifecycle behaves predictably under edge cases

// 🧼 Optional: Add display or logging helpers:
//     - `fn status_string(&self) -> &str`
//     - `fn log_transition(&self, from: LifecycleState, to: LifecycleState)`

// 🚀 Future: Add lifecycle hooks or callbacks:
//     - `on_initialize`, `on_shutdown`, etc.
//     - Useful for modular subsystems or plugin architecture



// Manages engine startup, shutdown, and lifecycle transitions.
// Tracks the current state and provides hooks for signal dispatch and diagnostics.

// Represents the current lifecycle state of the engine.

#[allow(unused_imports)]
use crate::zv9_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    /// Engine has not been initialized.
    Uninitialized,

    /// Engine is performing startup routines.
    Initializing,

    /// Engine is actively running.
    Running,

    /// Engine is shutting down.
    ShuttingDown,

    /// Engine has completed termination.
    Terminated,
}

/// Tracks the lifecycle state and provides transition methods.
#[derive(Debug, Clone)]
pub struct Lifecycle {
    pub state: LifecycleState,
}

impl Lifecycle {
    /// Creates a new lifecycle in the uninitialized state.
    pub fn new() -> Self {
        Self {
            state: LifecycleState::Uninitialized,
        }
    }

    /// Begins initialization.
    pub fn initialize(&mut self) {
        self.state = LifecycleState::Initializing;
        // Future: emit signal "engine_init_start"
    }

    /// Marks the engine as running.
    pub fn mark_running(&mut self) {
        self.state = LifecycleState::Running;
        // Future: emit signal "engine_initialized"
    }

    /// Begins shutdown.
    pub fn shutdown(&mut self) {
        self.state = LifecycleState::ShuttingDown;
        // Future: emit signal "engine_shutdown_start"
    }

    /// Finalizes termination.
    pub fn terminate(&mut self) {
        self.state = LifecycleState::Terminated;
        // Future: emit signal "engine_terminated"
    }

    /// Returns true if the engine is actively running.
    pub fn is_active(&self) -> bool {
        matches!(self.state, LifecycleState::Running)
    }

    /// Returns true if the engine is in the process of shutting down.
    pub fn is_shutting_down(&self) -> bool {
        matches!(self.state, LifecycleState::ShuttingDown)
    }

    /// Returns true if the engine has completed termination.
    pub fn is_terminated(&self) -> bool {
        matches!(self.state, LifecycleState::Terminated)
    }
}

// the end