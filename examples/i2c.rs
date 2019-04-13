#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32l0xx_hal::{prelude::*, rcc::Config, stm32};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());

    let gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_open_drain_output();
    let sda = gpiob.pb7.into_open_drain_output();

    let mut i2c = dp.I2C1.i2c((scl, sda), 10.khz(), &mut rcc);

    let mut buf: [u8; 1] = [0; 1];

    loop {
        match i2c.read(0x60, &mut buf) {
            Ok(_) => hprintln!("Buf: {:?}", buf).unwrap(),
            Err(err) => hprintln!("Err: {:?}", err).unwrap(),
        }
    }
}
