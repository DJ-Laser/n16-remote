#![no_std]
#![no_main]
#![feature(generic_const_exprs)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

mod keyboard;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::High);

    loop {
        info!("led on!");
        led.set_low();
        Timer::after_secs(1).await;

        info!("led off!");
        led.set_high();
        Timer::after_secs(1).await;
    }
}
