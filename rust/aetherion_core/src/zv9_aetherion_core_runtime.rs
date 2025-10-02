use std::thread::sleep;
use std::fmt;
use std::time::{Duration, Instant};
#[allow(unused_imports)]
use crate::zv9_prelude::*;
use crate::zv9_aetherion_core_conductor::{Conductor, ProcCommand};
use crate::zv9_util_config::EngineConfig; // ✅
use crate::zv9_util_logging::{log_info, log_debug};
use crate::pipeline::data::MapDataChunk;
use crate::pipeline::builder::{ChunkStreamer, ChunkDelivery};
use crate::log_component;

/// 🕒 Tracks tick progression and frame timing for the engine runtime.
pub struct RuntimeState {
    tick_count: u64,
    last_tick: Instant,
    frame_budget: Duration,
    exceeded_budget: bool,
    avg_tick_duration: Duration,
    on_tick: Option<Box<dyn FnMut(u64, Duration) + Send + Sync>>,
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

        if let Some(callback) = &mut self.on_tick {
            callback(self.tick_count, elapsed);
        }
    }

    pub fn set_frame_budget(&mut self, millis: u64) {
        self.frame_budget = Duration::from_millis(millis);
    }

    pub fn set_tick_listener<F>(&mut self, callback: F)
    where
        F: FnMut(u64, Duration) + Send + Sync + 'static,
    {
        self.on_tick = Some(Box::new(callback));
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

    pub fn is_budget_exceeded(&self) -> bool {
        self.exceeded_budget
    }

    pub fn has_tick_listener(&self) -> bool {
        self.on_tick.is_some()
    }
}

/// 🚀 Starts the Aetherion engine runtime loop with a given delivery backend.
pub fn start<D: ChunkDelivery + Send + 'static>(delivery: D) {
    log_component!("RuntimeState", "Tracks tick progression and frame timing");
    log_info("runtime", "Starting Aetherion engine...");

    let config = EngineConfig::default();
    let mut state = RuntimeState::new();
    let interval_ms = u64::from((1000 / config.tick_rate).max(1));
    state.set_frame_budget(interval_ms);

    let streamer = ChunkStreamer::new(delivery, config.interval_ms);
    let mut conductor = Conductor::new(streamer);
    let mut chunk = MapDataChunk::default();

    conductor.enqueue(ProcCommand::GenerateTerrain);
    conductor.enqueue(ProcCommand::EmitSignal("Engine started".into()));
    conductor.enqueue(ProcCommand::WaitTicks(10));
    conductor.enqueue(ProcCommand::EmitSignal("Midway checkpoint".into()));

    state.set_tick_listener(|tick, elapsed| {
        log_debug("tick", &format!("Tick {} took {:?}", tick, elapsed));
    });

    while state.ticks() < 20 {
        if state.time_since_last_tick() >= state.budget() {
            state.tick();
            conductor.tick(state.ticks(), &mut chunk);
        }
        sleep(Duration::from_millis(1));
    }

    log_info("runtime", "Aetherion engine stopped.");
}

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
        let called = std::sync::Arc::new(std::sync::Mutex::new(false));
        let called_clone = called.clone();

        state.set_tick_listener(move |tick, _| {
            if tick == 1 {
                *called_clone.lock().unwrap() = true;
            }
        });

        state.tick();
        assert!(*called.lock().unwrap());
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
        let state = RuntimeState::new();
        std::thread::sleep(Duration::from_millis(20));
        assert!(state.time_since_last_tick() >= Duration::from_millis(20));
    }
}
