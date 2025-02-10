use debouncer::Debouncer;
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

    fn update(&mut self, switch_state: bool, elapsed_ms: u16, debouncer_config: &D::Config) {
        self.pressed =
            self.debouncer
                .debounce(self.pressed, switch_state, elapsed_ms, debouncer_config)
    }
}

pub struct Keyboard<S: KeyScanner> {
    last_keypress_time: Option<Instant>,
    key_scanner: S,
}

impl<S: KeyScanner> Keyboard<S> {
    pub async fn run(&mut self) {
        // run the KeyScanner in a loop
    }
}
