use defmt::*;
use embassy_stm32::time::Hertz;
use embassy_stm32::{dma::NoDma, i2c, i2c::Instance, interrupt::typelevel::Binding, Peripheral};
mod reg;
pub use reg::Register;

use crate::lps22hh::reg::{IF_ADD_INC, LOW_NOISE_EN};

pub struct Lps22hh<'a, T: i2c::Instance> {
    i2c: i2c::I2c<'a, T>,
}

impl<'a, T: i2c::Instance> Lps22hh<'a, T> {
    #[must_use]
    pub fn new(
        peripheral: impl Peripheral<P = T> + 'a,
        scl_pin: impl Peripheral<P = impl i2c::SclPin<T>> + 'a,
        sda_pin: impl Peripheral<P = impl i2c::SdaPin<T>> + 'a,
        irq: impl Binding<<T as Instance>::ErrorInterrupt, embassy_stm32::i2c::ErrorInterruptHandler<T>>
            + Binding<<T as Instance>::EventInterrupt, embassy_stm32::i2c::EventInterruptHandler<T>>
            + 'a,
    ) -> Self {
        Self {
            i2c: i2c::I2c::new(
                peripheral,
                scl_pin,
                sda_pin,
                irq,
                NoDma,
                NoDma,
                Hertz(100_000),
                i2c::Config::default(),
            ),
        }
    }

    pub fn check_device_id(&mut self) -> bool {
        let mut data = [0u8, 1];
        match self
            .i2c
            .blocking_write_read(reg::I2C_SAD, &[Register::WHO_AM_I as u8], &mut data)
        {
            Ok(()) => {
                info!("Whoami: {}", data[0]);
                if data[0] == reg::DEVICE_ID {
                    true
                } else {
                    false
                }
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
                false
            }
        }
    }

    pub fn apply_config(&mut self) {
        let mut reg1: u8 = 0;

        // === CTRL_REG1 (10h) ===
        reg1 |= reg::Odr::Hz50 as u8;
        reg1 |= reg::LPF::DIV_9 as u8;
        info!("CTRL_REG1 to write: {}", reg1);

        let mut data = [0u8, 1];
        match self
            .i2c
            .blocking_write_read(reg::I2C_SAD, &[Register::CTRL_REG1 as u8], &mut data)
        {
            Ok(()) => {
                info!("CTRL_REG1_original: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }
        self.i2c
            .blocking_write(reg::I2C_SAD, &[Register::CTRL_REG1 as u8, reg1])
            .unwrap();
        match self
            .i2c
            .blocking_write_read(reg::I2C_SAD, &[Register::CTRL_REG1 as u8], &mut data)
        {
            Ok(()) => {
                info!("CTRL_REG1_written: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }

        let mut reg2: u8 = 0;

        // === CTRL_REG2 (10h) ===
        reg2 |= IF_ADD_INC;
        reg2 |= LOW_NOISE_EN;
        info!("CTRL_REG2 to write: {}", reg2);

        let mut data = [0u8, 1];
        match self
            .i2c
            .blocking_write_read(reg::I2C_SAD, &[Register::CTRL_REG2 as u8], &mut data)
        {
            Ok(()) => {
                info!("CTRL_REG2_original: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }
        self.i2c
            .blocking_write(reg::I2C_SAD, &[Register::CTRL_REG2 as u8, reg2])
            .unwrap();
        match self
            .i2c
            .blocking_write_read(reg::I2C_SAD, &[Register::CTRL_REG2 as u8], &mut data)
        {
            Ok(()) => {
                info!("CTRL_REG2_written: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }
    }

    pub fn sample(&mut self) {
        let mut buffer = [0u8; 3];

        match self.i2c.blocking_write_read(
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

        match self.i2c.blocking_write_read(
            reg::I2C_SAD,
            &[Register::TEMP_OUT_L as u8],
            &mut buffer,
        ) {
            Ok(()) => {
                let temp_buf = (buffer[1] as i16) << 8 | buffer[0] as i16;

                let temp = f32::from(temp_buf) * (reg::DEG_PER_LSB as f32) ;

                info!("Temperature: {}", temp);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }
    }
}
