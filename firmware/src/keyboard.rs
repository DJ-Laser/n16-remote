use debouncer::{Debouncer, DebouncerConfig};
use embassy_rp::gpio::{Input, Output};
use embassy_time::{Duration, Instant};

mod debouncer;

pub trait KeyScanner {
    const NUM_KEYS: usize;

    /// Wait until a key is pressed. Should return instantly if a key is already pressed
    async fn wait_for_keypress(&mut self);

    /// Scan the keyboard once and update the keyboard state
    async fn scan_keys(&mut self, last_scan_time: Instant);
}

pub struct KeyState<D: Debouncer> {
    pressed: bool,
    debouncer: D,
}

impl<D: Debouncer> KeyState<D> {
    pub fn pressed(&self) -> bool {
        self.pressed
    }

    fn update(&mut self, switch_state: bool, elapsed: Duration, debouncer_config: &D::Config) {
        self.pressed =
            self.debouncer
                .debounce(self.pressed, switch_state, elapsed, debouncer_config)
    }
}

pub struct Keyboard<S: KeyScanner, D: Debouncer> {
    last_keypress_time: Option<Instant>,
    key_scanner: S,
    debouncer_config: DebouncerConfig,
}

impl<S: KeyScanner> Keyboard<S> {
    pub async fn run(&mut self) {}
}
