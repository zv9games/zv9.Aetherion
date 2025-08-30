//! Tick and budget management utilities

use std::time::{Duration, Instant};

pub struct TickTimer {
    last_tick: Instant,
    tick_rate: Duration,
}

impl TickTimer {
    pub fn new(ticks_per_second: u32) -> Self {
        Self {
            last_tick: Instant::now(),
            tick_rate: Duration::from_secs_f64(1.0 / ticks_per_second as f64),
        }
    }

    pub fn should_tick(&mut self) -> bool {
        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
            true
        } else {
            false
        }
    }
}
