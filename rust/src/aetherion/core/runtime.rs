/// Tracks runtime state, tick progression, and frame budget.
pub struct RuntimeState {
    pub tick_count: u64,
}

impl RuntimeState {
    pub fn new() -> Self {
        Self { tick_count: 0 }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        // Future: track frame time, emit tick signals
    }
}
