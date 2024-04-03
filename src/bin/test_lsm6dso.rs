#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use sensor_pack::lsm6dso::Lsm6dso;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

type Channel1 =
    Mutex<CriticalSectionRawMutex, Option<I2c<'static, peripherals::I2C1, NoDma, NoDma>>>;
static I2C_REF: Channel1 = Mutex::new(None);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB7, Level::Low, Speed::Low);
    let i2c_ch: I2c<'static, peripherals::I2C1, NoDma, NoDma> = i2c::I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        i2c::Config::default(),
    );

    {
        *(I2C_REF.lock().await) = Some(i2c_ch);
    }

    let mut sensor = Lsm6dso::new(&I2C_REF);

    info!("Blink");
    Timer::after(Duration::from_millis(1000)).await;
    info!("Polling Lsm6dso");
    if sensor.check_device_id().await.unwrap() {
        led.set_high();
    }
    //Timer::after(Duration::from_millis(1000)).await;
    //sensor.apply_config().await.unwrap();
    //led.set_low();

    //loop {
    //    Timer::after(Duration::from_millis(400)).await;
    //    led.set_low();
    //    sensor.sample().await.unwrap();
    //    Timer::after(Duration::from_millis(100)).await;
    //    led.set_high();
    //}
}
