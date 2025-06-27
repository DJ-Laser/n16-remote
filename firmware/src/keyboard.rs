use core::array;

use config::KeyboardConfig;
use debouncer::{Debouncer, DebouncerConfig};
use embassy_time::{Duration, Instant, Timer};
use scanner::KeyScanner;

mod config;
pub mod debouncer;
pub mod scanner;

struct KeyState<D: Debouncer> {
    pressed: bool,
    debouncer: D,
}

impl<D: Debouncer> KeyState<D> {
    pub fn new() -> Self {
        Self {
            pressed: false,
            debouncer: D::default(),
        }
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    fn update(
        &mut self,
        switch_state: bool,
        elapsed: Duration,
        debouncer_config: &DebouncerConfig,
    ) {
        self.pressed =
            self.debouncer
                .debounce(self.pressed, switch_state, elapsed, debouncer_config)
    }
}

impl<D: Debouncer> Default for KeyState<D> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Keyboard<const NUM_KEYS: usize, S: KeyScanner<NUM_KEYS>, D: Debouncer> {
    config: KeyboardConfig,
    last_keypress_time: Option<Instant>,
    key_scanner: S,
    key_states: [KeyState<D>; NUM_KEYS],
}

impl<const NUM_KEYS: usize, S: KeyScanner<NUM_KEYS>, D: Debouncer> Keyboard<NUM_KEYS, S, D> {
    pub fn new_with_config(key_scanner: S, config: KeyboardConfig) -> Self {
        Self {
            config,
            key_scanner,
            last_keypress_time: None,
            key_states: array::from_fn(|_| KeyState::new()),
        }
    }

    pub fn new(key_scanner: S) -> Self {
        Self::new_with_config(key_scanner, KeyboardConfig::default())
    }

    pub async fn run(&mut self) {
        loop {
            if let Some(last_keypress_time) = self.last_keypress_time {
                if last_keypress_time.elapsed() > self.config.time_before_idle() {
                    self.last_keypress_time = None;
                    self.key_scanner.wait_for_keypress().await;
                }
            }

            let elapsed = Instant::now() - self.last_keypress_time.unwrap_or(Instant::now());

            self.key_scanner
                .scan_keys(|key: usize, switch_state: bool| {
                    self.key_states[key].update(
                        switch_state,
                        elapsed,
                        &self.config.debouncer_config(),
                    );
                    self.last_keypress_time = Some(Instant::now());
                })
                .await;

            Timer::after_micros(100).await;
        }
    }
}
