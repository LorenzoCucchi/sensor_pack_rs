use core::fmt::Error;
use defmt::*;
use embassy_stm32::{dma::NoDma, i2c::I2c, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
mod reg;
pub use reg::Register;

type Channel1 =
    Mutex<CriticalSectionRawMutex, Option<I2c<'static, peripherals::I2C1, NoDma, NoDma>>>;

pub struct Lsm6dso {
    i2c: &'static Channel1,
}

impl Lsm6dso {
    #[must_use]
    pub fn new(i2c: &'static Channel1) -> Self {
        Self { i2c }
    }

    pub async fn check_device_id(&self) -> Result<bool, Error> {
        let mut data = [0u8, 1];

        let mut i2c_unlocked = self.i2c.lock().await;

        let i2c_mut = i2c_unlocked.as_mut().ok_or(Error).unwrap();

        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::WHO_AM_I as u8], &mut data) {
            Ok(()) => {
                info!("Whoami: {}", data[0]);
                if data[0] == reg::DEVICE_ID {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
                Err(Error)
            }
        }
    }

    pub async fn apply_config(&self) {
        self.init_accelerometer().await;
        self.init_gyroscope().await;
    }

    pub async fn init_accelerometer(&self) {}
    pub async fn init_gyroscope(&self) {}

}
