#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

mod lps22hh;
use lps22hh::Lps22hh;
mod lis2mdl;
use lis2mdl::Lis2mdl;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB7, Level::Low, Speed::Low);

    let mut sensor = Lps22hh::new(p.I2C1, p.PB8, p.PB9, Irqs);

    
        info!("Blink");
        Timer::after(Duration::from_millis(1000)).await;
        if sensor.check_device_id() {
            led.set_high();
        }
        Timer::after(Duration::from_millis(1000)).await;
        sensor.apply_config();
        loop {
            Timer::after(Duration::from_millis(100)).await;
            sensor.sample();
            led.set_low();
            Timer::after(Duration::from_millis(100)).await;
            led.set_high();
        }

        
}
