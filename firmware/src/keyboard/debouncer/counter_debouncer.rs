use embassy_time::Duration;

use super::{Debouncer, DebouncerConfig};

pub struct CounterDebouncer {
    changed_ms: Duration,
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
            self.changed_ms = self
                .changed_ms
                .checked_sub(elapsed)
                .unwrap_or(Duration::MIN);
            return stored;
        } else if self.changed_ms < config.threshold_ms() {
            self.changed_ms = self
                .changed_ms
                .checked_add(elapsed)
                .unwrap_or(Duration::MAX);
            return stored;
        } else {
            return current;
        }
    }
}
