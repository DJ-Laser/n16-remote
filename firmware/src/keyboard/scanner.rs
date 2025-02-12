mod matrix;

pub use matrix::MatrixScanner;

pub trait KeyScanner<const NUM_KEYS: usize> {
    /// Wait until a key is pressed. Should return instantly if a key is already pressed
    async fn wait_for_keypress(&mut self);

    /// Scan the keyboard once and update the keyboard state
    async fn scan_keys<F: FnMut(usize, bool)>(&mut self, update_key: F);
}
