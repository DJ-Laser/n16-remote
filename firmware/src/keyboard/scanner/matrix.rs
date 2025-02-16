use embassy_futures::select::select_slice;
use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;
use gpio::AnyPin;

use embassy_rp::gpio;

use super::KeyScanner;

pub struct MatrixScanner<
    const INPUT_NUM: usize,
    const OUTPUT_NUM: usize,
    Input: InputPin + Wait,
    Output: OutputPin,
> {
    inputs: [Input; INPUT_NUM],
    outputs: [Output; OUTPUT_NUM],
}

impl<
        const INPUT_NUM: usize,
        const OUTPUT_NUM: usize,
        Input: InputPin + Wait,
        Output: OutputPin,
    > MatrixScanner<INPUT_NUM, OUTPUT_NUM, Input, Output>
{
    pub fn new(inputs: [Input; INPUT_NUM], outputs: [Output; OUTPUT_NUM]) -> Self {
        Self { inputs, outputs }
    }
}

impl<const INPUT_NUM: usize, const OUTPUT_NUM: usize>
    MatrixScanner<INPUT_NUM, OUTPUT_NUM, gpio::Input<'_>, gpio::Output<'_>>
{
    pub fn from_pins(inputs: [AnyPin; INPUT_NUM], outputs: [AnyPin; OUTPUT_NUM]) -> Self {
        Self::new(
            inputs.map(|input_pin| gpio::Input::new(input_pin, gpio::Pull::Down)),
            outputs.map(|output_pin| gpio::Output::new(output_pin, gpio::Level::Low)),
        )
    }
}

impl<
        const INPUT_NUM: usize,
        const OUTPUT_NUM: usize,
        Input: InputPin + Wait,
        Output: OutputPin,
    > KeyScanner<{ INPUT_NUM * OUTPUT_NUM }>
    for MatrixScanner<INPUT_NUM, OUTPUT_NUM, Input, Output>
{
    async fn wait_for_keypress(&mut self) {
        // Turn on all outputs so that an input will go high when any switch gets pressed
        for output in self.outputs.iter_mut() {
            output.set_high().ok();
        }

        Timer::after_micros(1).await;

        // Wait until any input goes high, signifying a keypress
        let mut futures = self.inputs.each_mut().map(|input| input.wait_for_high());
        let _ = select_slice(&mut futures).await;

        // Set all output pins back to low
        for out in self.outputs.iter_mut() {
            out.set_low().ok();
        }
    }

    async fn scan_keys<F: FnMut(usize, bool)>(&mut self, mut update_key: F) {
        for (output_idx, output) in self.outputs.iter_mut().enumerate() {
            // Turn on output pin and wait for input to register
            output.set_high().ok();
            Timer::after_micros(1).await;

            for (input_idx, input) in self.inputs.iter_mut().enumerate() {
                update_key(
                    output_idx * INPUT_NUM + input_idx,
                    input.is_high().unwrap_or(false),
                )
            }

            output.set_low().ok();
        }
    }
}
