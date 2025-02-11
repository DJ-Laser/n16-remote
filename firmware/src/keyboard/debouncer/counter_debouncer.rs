use super::Debouncer;

pub struct CounterDebouncer {
    changed_ms: u16,
}

impl Debouncer for CounterDebouncer {
    type Config = CounterDebouncerConfig;

    fn debounce(
        &mut self,
        stored: bool,
        current: bool,
        elapsed_ms: u16,
        config: &Self::Config,
    ) -> bool {
        if elapsed_ms == 0 {
            return stored;
        }

        if stored == current {
            self.changed_ms = self.changed_ms.saturating_sub(elapsed_ms);
            return stored;
        } else if self.changed_ms < config.threshold_ms() {
            self.changed_ms = self.changed_ms.saturating_add(elapsed_ms);
            return stored;
        } else {
            return current;
        }
    }
}

pub struct CounterDebouncerConfig {
    threshold_ms: u16,
}

impl CounterDebouncerConfig {
    pub fn new(threshold_ms: u16) -> Self {
        Self { threshold_ms }
    }

    pub fn threshold_ms(&self) -> u16 {
        self.threshold_ms
    }
}

impl Default for CounterDebouncerConfig {
    fn default() -> Self {
        Self::new(super::DEFAULT_THRESHOLD_MS)
    }
}
