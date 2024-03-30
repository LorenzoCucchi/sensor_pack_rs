
use defmt::*;
use embassy_stm32::time::Hertz;
use embassy_stm32::{dma::NoDma, i2c, i2c::Instance, interrupt::typelevel::Binding, Peripheral};
mod reg;
use self::reg::COMP_TEMP_EN;
pub use reg::Register;

#[derive(Debug, defmt::Format)]
pub struct DataStatus {
    pub zyxor: bool,
    pub xyzor: (bool, bool, bool),
    pub zyxda: bool,
    pub xyzda: (bool, bool, bool),
}

pub struct Lis2mdl<'a, T: i2c::Instance> {
    i2c: i2c::I2c<'a, T>,
}

impl<'a, T: i2c::Instance> Lis2mdl<'a, T> {
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
            Err(i2c::Error::Timeout) => {
                error!("Operation timed out");
                false
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
                false
            }
        }
    }

    pub fn apply_config(&mut self) {
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
        match self
            .i2c
            .blocking_write_read(reg::I2C_SAD, &[Register::CFG_REG_A as u8], &mut data)
        {
            Ok(()) => {
                info!("CFG_REG_A_original: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }
        self.i2c
            .blocking_write(reg::I2C_SAD, &[Register::CFG_REG_A as u8, reg])
            .unwrap();
        match self
            .i2c
            .blocking_write_read(reg::I2C_SAD, &[Register::CFG_REG_A as u8], &mut data)
        {
            Ok(()) => {
                info!("CFG_REG_A_written: {}", data[0]);
            }
            Err(e) => {
                error!("I2C Error: {:?}", e);
            }
        }
    }

    pub fn sample(&mut self) {
        let mut buffer = [0u8; 6];

        match self
            .i2c
            .blocking_write_read(reg::I2C_SAD, &[Register::OUT_X_L as u8], &mut buffer)
        {
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
        match self
        .i2c
        .blocking_write_read(reg::I2C_SAD, &[Register::TEMP_OUT_L_REG as u8], &mut temp)
    {
        Ok(()) => {
            let out_x = (temp[1] as i16) << 8 | temp[0] as i16;


            let temperature = reg::DEG_PER_LSB * f32::from(out_x) + reg::REF_TEMP;

            info!(
                "Temperature: {}",
                temperature
            );
        }
        Err(e) => {
            error!("I2C Error: {:?}", e);
        }
    }
    }
}
