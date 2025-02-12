use embassy_time::Duration;

pub mod counter_debouncer;

pub trait Debouncer: Default {
    fn debounce(
        &mut self,
        stored: bool,
        current: bool,
        elapsed: Duration,
        config: &DebouncerConfig,
    ) -> bool;
}

pub struct DebouncerConfig {
    threshold_ms: Duration,
}

impl DebouncerConfig {
    pub const DEFAULT_DEBOUNCE_DURATION: Duration = Duration::from_millis(5);

    pub fn new(threshold_ms: Duration) -> Self {
        Self { threshold_ms }
    }

    pub fn threshold_ms(&self) -> Duration {
        self.threshold_ms
    }
}

impl Default for DebouncerConfig {
    fn default() -> Self {
        Self::new(Self::DEFAULT_DEBOUNCE_DURATION)
    }
}
