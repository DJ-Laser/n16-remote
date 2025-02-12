use embassy_time::Duration;

use super::{Debouncer, DebouncerConfig};

pub struct CounterDebouncer {
    changed_duration: Duration,
}

impl CounterDebouncer {
    pub fn new() -> Self {
        Self {
            changed_duration: Duration::from_ticks(0),
        }
    }
}

impl Debouncer for CounterDebouncer {
    fn debounce(
        &mut self,
        stored: bool,
        current: bool,
        elapsed: Duration,
        config: &DebouncerConfig,
    ) -> bool {
        if elapsed.as_millis() == 0 {
            return stored;
        }

        if stored == current {
            self.changed_duration = self
                .changed_duration
                .checked_sub(elapsed)
                .unwrap_or(Duration::MIN);
            return stored;
        } else if self.changed_duration < config.threshold_ms() {
            self.changed_duration = self
                .changed_duration
                .checked_add(elapsed)
                .unwrap_or(Duration::MAX);
            return stored;
        } else {
            return current;
        }
    }
}

impl Default for CounterDebouncer {
    fn default() -> Self {
        Self::new()
    }
}
