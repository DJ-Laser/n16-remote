use debouncer::{Debouncer, DebouncerConfig};
use embassy_time::{Duration, Instant};

mod debouncer;

pub trait KeyScanner<const NUM_KEYS: usize> {
    /// Wait until a key is pressed. Should return instantly if a key is already pressed
    async fn wait_for_keypress(&mut self);

    /// Scan the keyboard once and update the keyboard state
    async fn scan_keys<D: Debouncer>(&mut self, key_states: &mut [KeyState<D>; NUM_KEYS]);
}

pub struct KeyState<D: Debouncer> {
    pressed: bool,
    debouncer: D,
}

impl<D: Debouncer> KeyState<D> {
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

pub struct Keyboard<const NUM_KEYS: usize, S: KeyScanner<NUM_KEYS>, D: Debouncer> {
    debouncer_config: DebouncerConfig,
    last_keypress_time: Option<Instant>,
    time_before_idle: Duration,
    key_scanner: S,
    key_states: [KeyState<D>; NUM_KEYS],
}

impl<const NUM_KEYS: usize, S: KeyScanner<NUM_KEYS>, D: Debouncer> Keyboard<NUM_KEYS, S, D> {
    pub async fn run(&mut self) {
        loop {
            if let Some(last_keypress_time) = self.last_keypress_time {
                if last_keypress_time.elapsed() > self.time_before_idle {
                    self.last_keypress_time = None;
                    self.key_scanner.wait_for_keypress().await;
                    self.last_keypress_time = Some(Instant::now())
                }
            }

            self.key_scanner.scan_keys(&mut self.key_states).await
        }
    }
}
