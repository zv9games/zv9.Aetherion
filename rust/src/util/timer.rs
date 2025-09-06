use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}
