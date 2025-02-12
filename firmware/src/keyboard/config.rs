use embassy_time::Duration;

use super::debouncer::DebouncerConfig;

pub struct KeyboardConfig {
    debouncer_config: DebouncerConfig,
    time_before_idle: Duration,
}

impl KeyboardConfig {
    pub const DEFAULT_TIME_BEFORE_IDLE: Duration = Duration::from_millis(10);

    pub fn new(debouncer_config: DebouncerConfig, time_before_idle: Duration) -> Self {
        Self {
            debouncer_config,
            time_before_idle,
        }
    }

    pub fn debouncer_config(&self) -> &DebouncerConfig {
        &self.debouncer_config
    }

    pub fn time_before_idle(&self) -> Duration {
        self.time_before_idle
    }
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self::new(DebouncerConfig::default(), Self::DEFAULT_TIME_BEFORE_IDLE)
    }
}
