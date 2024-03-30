#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[cfg(feature = "out_f32")]
use cast::f32;
#[cfg(feature = "out_f32")]
use num_derive::FromPrimitive;

/// I2C slave address
pub const I2C_SAD: u8 = 0b0011110;

pub const GAUSS_PER_LSB: f32 = 0.0015;
pub const DEG_PER_LSB: f32 = 0.125;
pub const REF_TEMP: f32 = 25.0;

/// Operating mode
#[allow(dead_code)]
pub enum Mode {
    HighResolution,

    LowPower,
}

/// Register mapping
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub enum Register {
    OFFSET_X_REG_L = 0x45,
    OFFSET_X_REG_H = 0x46,
    OFFSET_Y_REG_L = 0x47,
    OFFSET_Y_REG_H = 0x48,
    OFFSET_Z_REG_L = 0x49,
    OFFSET_Z_REG_H = 0x4A,
    WHO_AM_I = 0x4F,
    CFG_REG_A = 0x60,
    CFG_REG_B = 0x61,
    CFG_REG_C = 0x62,
    INT_CTRL_REG = 0x63,
    INT_SOURCE_REG = 0x64,
    INT_THS_L_REG = 0x65,
    INT_THS_H_REG = 0x66,
    STATUS_REG = 0x67,
    OUT_X_L = 0x68,
    OUT_X_H = 0x69,
    OUT_Y_L = 0x6A,
    OUT_Y_H = 0x6B,
    OUT_Z_L = 0x6C,
    OUT_Z_H = 0x6D,
    TEMP_OUT_L_REG = 0x6E,
    TEMP_OUT_H_REG = 0x6F,
}

#[allow(dead_code)]
impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }
}

// === WHO_AM_I (4Fh) ===

/// WHO_AM_I device identification register
pub const DEVICE_ID: u8 = 0b0100_0000;

// === CFG_REG_A (60h) === //TODO

pub const COMP_TEMP_EN: u8 = 0b1000_0000;
pub const REBOOT: u8 = 0b0100_0000;
pub const SOFT_RST: u8 = 0b0010_0000;
pub const LP: u8 = 0b0001_0000;
pub const ODR_MASK: u8 = 0b0000_1100;
pub const MOD_MASK: u8 = 0b0000_0011;

/// Output Data Rate
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum Odr {
    /// 10 Hz
    Hz10 = 0b0000,
    /// 20 Hz
    Hz20 = 0b0100,
    /// 50 Hz
    Hz50 = 0b1000,
    /// 100 Hz
    Hz100 = 0b1100,
}

// Mode of Operation
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum ModeOp {
    Continuous = 0b00,
    Single = 0b01,
    Idle1 = 0b10,
    Idle = 0b11,
}

// === CFG_REG_B (61h) ===
pub const OFF_CANC_ONE_SHOT: u8 = 0b0001_0000;
pub const INT_ON_DATAOFF: u8 = 0b0000_1000;
pub const SET_FREQ: u8 = 0b0000_0100;
pub const OFF_CANC: u8 = 0b0000_0010;
pub const LPF: u8 = 0b0000_0001;

// === CFG_REG_C (62h) ===
pub const INT_ON_PIN: u8 = 0b0100_0000;
pub const I2C_DIS: u8 = 0b0010_0000;
pub const BDU: u8 = 0b0001_0000;
pub const BLE: u8 = 0b0000_1000;
pub const _4WSPI: u8 = 0b0000_0100;
pub const SELF_TEST: u8 = 0b0000_0010;
pub const DRDY_ON_PIN: u8 = 0b0000_0001;

// === INT_CTRL_REG (63h) ===
pub const XIEN: u8 = 0b1000_0000;
pub const YIEN: u8 = 0b0100_0000;
pub const ZIEN: u8 = 0b0010_0000;
pub const IEA: u8 = 0b0000_0100;
pub const IEL: u8 = 0b0000_0010;
pub const IEN: u8 = 0b0000_0001;

// === INT_SOURCE_REG (64h) ===
pub const P_TH_S_X: u8 = 0b1000_0000;
pub const P_TH_S_Y: u8 = 0b0100_0000;
pub const P_TH_S_Z: u8 = 0b0010_0000;
pub const N_TH_S_X: u8 = 0b0001_0000;
pub const N_TH_S_Y: u8 = 0b0000_1000;
pub const N_TH_S_Z: u8 = 0b0000_0100;
pub const MROI: u8 = 0b0000_0010;
pub const INT: u8 = 0b0000_0001;

// === STATUS_REG (67h) ===
pub const ZYXOR: u8 = 0b1000_0000;
pub const ZOR: u8 = 0b0100_0000;
pub const YOR: u8 = 0b0010_0000;
pub const XOR: u8 = 0b0001_0000;
pub const ZYXDA: u8 = 0b0000_1000;
pub const ZDA: u8 = 0b0000_0100;
pub const YDA: u8 = 0b0000_0010;
pub const XDA: u8 = 0b0000_0001;
