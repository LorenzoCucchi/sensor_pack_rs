use core::fmt::Error;
use defmt::*;
use embassy_stm32::{dma::NoDma, i2c::I2c, peripherals};
mod reg;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
pub use reg::Register;

type Channel1 =
    Mutex<CriticalSectionRawMutex, Option<I2c<'static, peripherals::I2C1, NoDma, NoDma>>>;

pub struct Lps22hh {
    i2c: &'static Channel1,
}

impl Lps22hh {
    #[must_use]
    pub fn new(i2c: &'static Channel1) -> Self {
        Self { i2c }
    }

    pub async fn check_device_id(&self) -> Result<bool, Error> {
        let mut data = [0u8, 1];
        let mut i2c_unlocked = self.i2c.lock().await;

        let i2c_mut = i2c_unlocked.as_mut().ok_or(Error).unwrap(); // Lock the inner I2c instance

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

    pub async fn apply_config(&self) -> Result<bool, Error> {
        let mut reg1: u8 = 0;

        // === CTRL_REG1 (10h) ===
        reg1 |= reg::Odr::Hz50 as u8;
        reg1 |= reg::LPF::DIV_9 as u8;
        info!("CTRL_REG1 to write: {}", reg1);

        let mut data = [0u8, 1];
        let mut i2c_unlocked = self.i2c.lock().await;

        //let i2c_guard: &I2c<'static, peripherals::I2C1, NoDma, NoDma> = i2c_unlocked.as_ref().ok_or(Error)?;
        let i2c_mut = i2c_unlocked.as_mut().ok_or(Error).unwrap(); // Lock the inner I2c instance

        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::CTRL_REG1 as u8], &mut data) {
            Ok(()) => {
                info!("CTRL_REG1_original: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }

        i2c_mut
            .blocking_write(reg::I2C_SAD, &[Register::CTRL_REG1 as u8, reg1])
            .unwrap();

        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::CTRL_REG1 as u8], &mut data) {
            Ok(()) => {
                info!("CTRL_REG1_written: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }

        let mut reg2: u8 = 0;

        // === CTRL_REG2 (10h) ===
        reg2 |= reg::IF_ADD_INC;
        reg2 |= reg::LOW_NOISE_EN;
        info!("CTRL_REG2 to write: {}", reg2);

        let mut data = [0u8, 1];
        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::CTRL_REG2 as u8], &mut data) {
            Ok(()) => {
                info!("CTRL_REG2_original: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }
        i2c_mut
            .blocking_write(reg::I2C_SAD, &[Register::CTRL_REG2 as u8, reg2])
            .unwrap();
        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::CTRL_REG2 as u8], &mut data) {
            Ok(()) => {
                info!("CTRL_REG2_written: {}", data[0]);
                Ok(true)
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
                Err(Error)
            }
        }
    }

    pub async fn sample(&self) -> Result<bool, Error> {
        let mut buffer = [0u8; 3];

        let mut i2c_unlocked = self.i2c.lock().await;

        //let i2c_guard: &I2c<'static, peripherals::I2C1, NoDma, NoDma> = i2c_unlocked.as_ref().ok_or(Error)?;
        let i2c_mut = i2c_unlocked.as_mut().ok_or(Error).unwrap(); // Lock the inner I2c instance

        match i2c_mut.blocking_write_read(
            reg::I2C_SAD,
            &[Register::PRESSURE_OUT_XL as u8],
            &mut buffer,
        ) {
            Ok(()) => {
                let press_buf =
                    (buffer[2] as i32) << 16 | (buffer[1] as i32) << 8 | buffer[0] as i32;

                let press = f64::from(press_buf) / (reg::LSB_PER_hPA as f64) * 100.0;

                info!("Pressure: {}", press);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }

        let mut buffer = [0u8; 2];

        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::TEMP_OUT_L as u8], &mut buffer)
        {
            Ok(()) => {
                let temp_buf = (buffer[1] as i16) << 8 | buffer[0] as i16;

                let temp = f32::from(temp_buf) * (reg::DEG_PER_LSB as f32);

                info!("Temperature: {}", temp);
                Ok(true)
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
                Err(Error)
            }
        }
    }
}
