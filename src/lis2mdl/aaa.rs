use defmt::*;
use embassy_stm32::time::Hertz;
use embassy_stm32::PeripheralRef;
use embassy_stm32::{dma::NoDma, i2c, i2c::Instance, interrupt::typelevel::Binding, Peripheral};
mod reg;
pub use reg::Register;


pub struct Lis2mdl<'i, T: i2c::Instance> {
    i2c: &'i mut [i2c::I2c<'i, T, NoDma, NoDma>],
}

impl<'d, T: i2c::I2c> Lis2mdl<'d, T> {
    #[must_use]
    pub fn new(i2c: &'d mut [i2c::I2c<'d, T, NoDma, NoDma>]) -> Self {
        Self { i2c }
    }
    
    pub fn destroy(self) {}

    pub fn check_device_id(&mut self) -> bool {
        let mut data = [0u8, 1];
        match self
            .i2c[0]
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
}
