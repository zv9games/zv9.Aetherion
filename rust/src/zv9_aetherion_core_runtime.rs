//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_core_runtime.rs
use std::thread::sleep;

use crate::zv9_prelude::*;
use std::fmt;
use std::time::{Duration, Instant};
use crate::ProcCommand;

/// 🕒 Tracks tick progression and frame timing for the engine runtime.
pub struct RuntimeState {
    tick_count: u64,
    last_tick: Instant,
    frame_budget: Duration,
    exceeded_budget: bool,
    avg_tick_duration: Duration,
    on_tick: Option<Box<dyn Fn(u64, Duration) + Send + Sync>>,
}

impl fmt::Debug for RuntimeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeState")
            .field("tick_count", &self.tick_count)
            .field("last_tick", &self.last_tick)
            .field("frame_budget", &self.frame_budget)
            .field("exceeded_budget", &self.exceeded_budget)
            .field("avg_tick_duration", &self.avg_tick_duration)
            .field("has_tick_listener", &self.on_tick.is_some())
            .finish()
    }
}

impl RuntimeState {
    /// Creates a new runtime tracker with a default frame budget (16ms ≈ 60 FPS).
    pub fn new() -> Self {
        Self {
            tick_count: 0,
            last_tick: Instant::now(),
            frame_budget: Duration::from_millis(16),
            exceeded_budget: false,
            avg_tick_duration: Duration::ZERO,
            on_tick: None,
        }
    }

    /// Advances the tick and updates internal timing state.
    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.exceeded_budget = elapsed > self.frame_budget;
        self.last_tick = now;
        self.tick_count += 1;

        self.avg_tick_duration = if self.tick_count == 1 {
            elapsed
        } else {
            (self.avg_tick_duration * 9 + elapsed) / 10
        };

        if let Some(callback) = &self.on_tick {
            callback(self.tick_count, elapsed);
        }
    }

    pub fn is_budget_exceeded(&self) -> bool {
        self.exceeded_budget
    }

    pub fn set_frame_budget(&mut self, millis: u64) {
        self.frame_budget = Duration::from_millis(millis);
    }

    pub fn time_since_last_tick(&self) -> Duration {
        self.last_tick.elapsed()
    }

    pub fn ticks(&self) -> u64 {
        self.tick_count
    }

    pub fn budget(&self) -> Duration {
        self.frame_budget
    }

    pub fn average_tick_duration(&self) -> Duration {
        self.avg_tick_duration
    }

    pub fn set_tick_listener<F>(&mut self, callback: F)
    where
        F: Fn(u64, Duration) + Send + Sync + 'static,
    {
        self.on_tick = Some(Box::new(callback));
    }

    pub fn has_tick_listener(&self) -> bool {
        self.on_tick.is_some()
    }
}

//
// ─── Engine Startup ───────────────────────────────────────────────────────────
//

/// 🚀 Starts the Aetherion engine runtime loop.

pub fn start() {
    log_info("runtime", "Starting Aetherion engine...");

    // 🧭 Engine configuration
    let config = EngineConfig::default();
    let mut state = RuntimeState::new();
    state.set_frame_budget(u64::from((1000 / config.tick_rate).max(1)));

    // 🧱 Runtime components
    let mut conductor = Conductor::new(GodotSync::init());
    let mut chunk = MapDataChunk::default();

    // 🎬 Initial command queue
    conductor.enqueue(ProcCommand::GenerateTerrain);
    conductor.enqueue(ProcCommand::EmitSignal("Engine started".into()));
    conductor.enqueue(ProcCommand::WaitTicks(10));
    conductor.enqueue(ProcCommand::EmitSignal("Midway checkpoint".into()));

    // 🧪 Tick diagnostics
    state.set_tick_listener(|tick, elapsed| {
        log_debug("tick", &format!("Tick {} took {:?}", tick, elapsed));
    });

    // 🔁 Main tick loop
    loop {
        if state.time_since_last_tick() >= state.budget() {
            state.tick();
            conductor.tick(state.ticks(), &mut chunk);

            if state.ticks() >= 20 {
                log_info("runtime", "Engine shutdown requested.");
                break;
            }
        }

        sleep(Duration::from_millis(1)); // Prevent CPU thrashing
    }

    log_info("runtime", "Aetherion engine stopped.");
}




//
// ─── Stress Tests ─────────────────────────────────────────────────────────────
//

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_tick_flood() {
        let mut state = RuntimeState::new();
        for _ in 0..100_000 {
            state.tick();
        }
        assert_eq!(state.ticks(), 100_000);
    }

    #[test]
    fn stress_budget_enforcement() {
        let mut state = RuntimeState::new();
        state.set_frame_budget(1);
        std::thread::sleep(Duration::from_millis(5));
        state.tick();
        assert!(state.is_budget_exceeded());
    }

    #[test]
    fn stress_listener_callback() {
        let mut state = RuntimeState::new();
        let mut called = false;
        state.set_tick_listener(move |tick, _| {
            if tick == 1 {
                called = true;
            }
        });
        state.tick();
        assert!(called);
    }

    #[test]
    fn stress_average_smoothing() {
        let mut state = RuntimeState::new();
        for _ in 0..10 {
            std::thread::sleep(Duration::from_millis(10));
            state.tick();
        }
        assert!(state.average_tick_duration() > Duration::from_millis(5));
    }

    #[test]
    fn stress_time_since_last_tick() {
        let mut state = RuntimeState::new();
        std::thread::sleep(Duration::from_millis(20));
        assert!(state.time_since_last_tick() >= Duration::from_millis(20));
    }
}


// the end