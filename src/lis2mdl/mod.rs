use core::fmt::Error;
use defmt::*;
use embassy_stm32::{dma::NoDma, i2c::I2c, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
mod reg;
use self::reg::COMP_TEMP_EN;
pub use reg::Register;

type Channel1 =
    Mutex<CriticalSectionRawMutex, Option<I2c<'static, peripherals::I2C1, NoDma, NoDma>>>;

pub struct Lis2mdl {
    i2c: &'static Channel1,
}

impl Lis2mdl {
    #[must_use]
    pub fn new(i2c: &'static Channel1) -> Self {
        Self { i2c }
    }

    pub async fn check_device_id(&mut self) -> Result<bool, Error> {
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

    pub async fn apply_config(&mut self) -> Result<bool, Error> {
        let mut reg: u8 = 0;

        // === CFG_REG_A (60h) ===
        reg |= COMP_TEMP_EN;
        reg |= reg::Odr::Hz100 as u8;
        reg |= reg::ModeOp::Continuous as u8;
        info!("comp_temp: {}", COMP_TEMP_EN);
        info!("Odr: {}", reg::Odr::Hz100 as u8);
        info!("ModeOp: {}", reg::ModeOp::Continuous as u8);
        info!("CFG_REG_A to write: {}", reg);

        let mut data = [0u8, 1];

        let mut i2c_unlocked = self.i2c.lock().await;

        let i2c_mut = i2c_unlocked.as_mut().ok_or(Error).unwrap(); // Lock the inner I2c instance
        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::CFG_REG_A as u8], &mut data) {
            Ok(()) => {
                info!("CFG_REG_A_original: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }
        i2c_mut
            .blocking_write(reg::I2C_SAD, &[Register::CFG_REG_A as u8, reg])
            .unwrap();
        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::CFG_REG_A as u8], &mut data) {
            Ok(()) => {
                info!("CFG_REG_A_written: {}", data[0]);
                Ok(true)
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
                Err(Error)
            }
        }
    }

    pub async fn sample(&mut self) -> Result<bool, Error> {
        let mut buffer = [0u8; 6];

        let mut i2c_unlocked = self.i2c.lock().await;

        let i2c_mut = i2c_unlocked.as_mut().ok_or(Error).unwrap(); // Lock the inner I2c instance

        match i2c_mut.blocking_write_read(reg::I2C_SAD, &[Register::OUT_X_L as u8], &mut buffer) {
            Ok(()) => {
                let out_x = (buffer[1] as i16) << 8 | buffer[0] as i16;
                let out_y = (buffer[3] as i16) << 8 | buffer[2] as i16;
                let out_z = (buffer[5] as i16) << 8 | buffer[4] as i16;

                let magnetic_field_x = reg::GAUSS_PER_LSB * f32::from(out_x);
                let magnetic_field_y = reg::GAUSS_PER_LSB * f32::from(out_y);
                let magnetic_field_z = reg::GAUSS_PER_LSB * f32::from(out_z);

                info!(
                    "Mag Field: {}, {}, {}",
                    magnetic_field_x, magnetic_field_y, magnetic_field_z
                );
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }

        let mut temp = [0u8; 2];
        match i2c_mut.blocking_write_read(
            reg::I2C_SAD,
            &[Register::TEMP_OUT_L_REG as u8],
            &mut temp,
        ) {
            Ok(()) => {
                let out_x = (temp[1] as i16) << 8 | temp[0] as i16;

                let temperature = reg::DEG_PER_LSB * f32::from(out_x) + reg::REF_TEMP;

                info!("Temperature: {}", temperature);
                Ok(true)
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
                Err(Error)
            }
        }
    }
}
