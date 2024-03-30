#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[cfg(feature = "out_f32")]
use cast::f32;
#[cfg(feature = "out_f32")]
use num_derive::FromPrimitive;

/// I2C slave address
pub const I2C_SAD: u8 = 0b1011101;

pub const LSB_PER_hPA: f32 = 4096.0;
pub const DEG_PER_LSB: f32 = 0.01;
pub const REF_TEMP: f32 = 25.0;

/// Register mapping
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub enum Register {
    INTERRUPT_CFG = 0x0B,
    THS_P_L = 0x0C,
    THS_P_H = 0x0D,
    IF_CTRL = 0x0E,
    WHO_AM_I = 0x0F,
    CTRL_REG1 = 0x10,
    CTRL_REG2 = 0x11,
    CTRL_REG3 = 0x12,
    FIFO_CTRL = 0x13,
    FIFO_WTM = 0x14,
    REF_P_L = 0x15,
    REF_P_H = 0x16,
    RPDS_L = 0x18,
    RPDS_H = 0x19,
    INT_SOURCE = 0x24,
    FIFO_STATUS1 = 0x25,
    FIFO_STATUS2 = 0x26,
    STATUS = 0x27,
    PRESSURE_OUT_XL = 0x28,
    PRESSURE_OUT_L = 0x29,
    PRESSURE_OUT_H = 0x2A,
    TEMP_OUT_L = 0x2B,
    TEMP_OUT_H = 0x2C,
    FIFO_DATA_OUT_P_XL = 0x78,
    FIFO_DATA_OUT_P_L = 0x79,
    FIFO_DATA_OUT_P_H = 0x7A,
    FIFO_DATA_OUT_T_L = 0x7B,
    FIFO_DATA_OUT_T_H = 0x7C,
}

#[allow(dead_code)]
impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }
}

// === WHO_AM_I (0Fh) ===

/// WHO_AM_I device identification register
pub const DEVICE_ID: u8 = 0b10110011;

// === IF_CTRL (0Eh) === //TODO

pub const INT_EN_I3C: u8 = 0b1000_0000;
pub const SDA_PU_EN: u8 = 0b0001_0000;
pub const SDO_PU_EN: u8 = 0b0000_1000;
pub const PD_DIS_INT1: u8 = 0b0000_0100;
pub const I3C_DISABLE: u8 = 0b0000_0010;
pub const I2C_DISABLE: u8 = 0b0000_0001;

// === CTRL_REG1 (10h) ===
pub const BDU: u8 = 0b0000_0010;
pub const SIM: u8 = 0b0000_0001;

/// Output Data Rate
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum Odr {
    /// One-shot
    One_shot = 0b0000_0000,
    /// 1 Hz
    Hz1 = 0b0001_0000,
    /// 10 Hz
    Hz10 = 0b0010_0000,
    /// 25 Hz
    Hz20 = 0b0011_0000,
    /// 50 Hz
    Hz50 = 0b0100_0000,
    /// 75 Hz
    Hz75 = 0b0101_0000,
    /// 100 Hz
    Hz100 = 0b0110_0000,
    /// 200 Hz
    Hz200 = 0b0111_0000,
}

/// Low-pass filter
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum LPF {
    /// ODR/2
    DIV_2 = 0b0000_0000,
    /// ODR/9
    DIV_9 = 0b0000_1000,
    /// ODR/20
    DIV_20 = 0b0000_1100,
}

// === CTRL_REG2 (11h) ===
pub const BOOT: u8 = 0b1000_0000;
pub const INT_H_L: u8 = 0b0100_0000;
pub const PP_OD: u8 = 0b0010_0000;
pub const IF_ADD_INC: u8 = 0b0001_0000;
pub const SWRESET: u8 = 0b0000_0100;
pub const LOW_NOISE_EN: u8 = 0b0000_0010;
pub const ONE_SHOT: u8 = 0b0000_0001;

// === CTRL_REG3 (12h) ===
pub const INT_F_FULL: u8 = 0b0010_0000;
pub const INT_F_WTM: u8 = 0b0001_0000;
pub const INT_F_OVR: u8 = 0b0000_1000;
pub const DRDY: u8 = 0b0000_0100;
pub const INT_S1: u8 = 0b0000_0010;
pub const INT_S0: u8 = 0b0000_0001;

// === FIFO_CTRL (13h) ===
pub const STOP_ON_WTM: u8 = 0b0000_1000;
// Fifo Mode
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum FifoMode {
    /// Bypass
    Bypass = 0b0000_0000,
    /// FIFO mode
    FIFO_Mode = 0b0000_0001,
    /// Continuous (Dynamic-Stream)
    Continuous = 0b0000_0010,
    /// Bypass-to-FIFO
    BypToFifo = 0b0000_0101,
    /// Bypass-to-Continuous
    BypToCont = 0b0000_0110,
    /// Continuous-to-FIFO
    ContToFifo = 0b0000_0111,
}

