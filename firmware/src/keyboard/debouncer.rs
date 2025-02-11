pub mod counter_debouncer;

pub const DEFAULT_THRESHOLD_MS: u16 = 10;

pub trait Debouncer {
    type Config;

    fn debounce(
        &mut self,
        stored: bool,
        current: bool,
        elapsed_ms: u16,
        config: &Self::Config,
    ) -> bool;
}
