#![no_std]
#![no_main]
#![feature(generic_const_exprs)]

use core::mem::MaybeUninit;

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::{
    bind_interrupts,
    gpio::{self, AnyPin},
    peripherals::USB,
    usb as rp_usb,
};
use gpio::Output;
use keyboard::{debouncer::counter_debouncer::CounterDebouncer, scanner::MatrixScanner, Keyboard};
use usb::{setup_usb_device, UsbDeviceData};
use {defmt_rtt as _, panic_probe as _};

mod keyboard;
mod usb;

macro_rules! get_keyboard_pins {
    ($p:ident) => {{
        let output_col_pins = [
            AnyPin::from($p.PIN_27),
            AnyPin::from($p.PIN_7),
            AnyPin::from($p.PIN_6),
            AnyPin::from($p.PIN_4),
        ];
        let input_row_pins = [
            AnyPin::from($p.PIN_29),
            AnyPin::from($p.PIN_1),
            AnyPin::from($p.PIN_2),
        ];

        (input_row_pins, output_col_pins)
    }};
}

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => rp_usb::InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn run_usb() {}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut usb_data = UsbDeviceData::new();
    let mut usb_device = setup_usb_device(rp_usb::Driver::new(p.USB, Irqs), &mut usb_data);
    let (reader, mut writer) = usb_device.hid.split();

    let (input_row_pins, output_col_pins) = get_keyboard_pins!(p);

    let mut keyboard: Keyboard<
        12,
        MatrixScanner<3, 4, gpio::Input<'_>, Output<'_>>,
        CounterDebouncer,
    > = Keyboard::new(MatrixScanner::from_pins(input_row_pins, output_col_pins));

    join(
        keyboard.run(),
        join(usb_device.device.run(), reader.ready(buf)),
    )
    .await;
}

pub fn x<T>(t: T) {
    static X: MaybeUninit<T> = MaybeUninit::uninit();
}
