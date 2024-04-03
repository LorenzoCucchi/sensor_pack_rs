#![allow(non_upper_case_globals)]
#![allow(dead_code)]

/// I2C slave address
pub const I2C_SAD: u8 = 0b1101011;

/// Operating mode
#[allow(dead_code)]
pub enum AccMode {
    PowerDown,
    UltraLowPower,
    LowPower,
    Normal,
    HighPerf,
}

/// Operating mode
#[allow(dead_code)]
pub enum GyrMode {
    PowerDown,
    LowPower,
    Normal,
    HighPerf,
}

/// Register mapping
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub enum Register {
    FUNC_CFG_ACCESS = 0x01,
    PIN_CTRL = 0x02,
    FIFO_CTRL1 = 0x07,
    FIFO_CTRL2 = 0x08,
    FIFO_CTRL3 = 0x09,
    FIFO_CTRL4 = 0x0A,
    COUNTER_BDR_REG1 = 0x0B,
    COUNTER_BDR_REG2 = 0x0C,
    INT1_CTRL = 0x0D,
    INT2_CTRL = 0x0E,
    WHO_AM_I = 0x0F,
    CTRL1_XL = 0x10,
    CTRL2_G = 0x11,
    CTRL3_C = 0x12,
    CTRL4_C = 0x13,
    CTRL5_C = 0x14,
    CTRL6_C = 0x15,
    CTRL7_G = 0x16,
    CTRL8_XL = 0x17,
    CTRL9_XL = 0x18,
    CTRL10_C = 0x19,
    ALL_INT_SRC = 0x1A,
    WAKE_UP_SRC = 0x1B,
    TAP_SRC = 0x1C,
    D6D_SRC = 0x1D,
    STATUS_REG = 0x1E,
    OUT_TEMP_L = 0x20,
    OUT_TEMP_H = 0x21,
    OUTX_L_G = 0x22,
    OUTX_H_G = 0x23,
    OUTY_L_G = 0x24,
    OUTY_H_G = 0x25,
    OUTZ_L_G = 0x26,
    OUTZ_H_G = 0x27,
    OUTX_L_A = 0x28,
    OUTX_H_A = 0x29,
    OUTY_L_A = 0x2A,
    OUTY_H_A = 0x2B,
    OUTZ_L_A = 0x2C,
    OUTZ_H_A = 0x2D,
    EMB_FUNC_STATUS_MAINPAGE = 0x35,
    FSM_STATUS_A_MAINPAGE = 0x36,
    FSM_STATUS_B_MAINPAGE = 0x37,
    STATUS_MASTER_MAINPAGE = 0x39,
    FIFO_STATUS1 = 0x3A,
    FIFO_STATUS2 = 0x3B,
    TIMESTAMP0 = 0x40,
    TIMESTAMP1 = 0x41,
    TIMESTAMP2 = 0x42,
    TIMESTAMP3 = 0x43,
    TAP_CFG0 = 0x56,
    TAP_CFG1 = 0x57,
    TAP_CFG2 = 0x58,
    TAP_THS_6D = 0x59,
    INT_DUR2 = 0x5A,
    WAKE_UP_THS = 0x5B,
    WAKE_UP_DUR = 0x5C,
    FREE_FALL = 0x5D,
    MD1_CFG = 0x5E,
    MD2_CFG = 0x5F,
    I3C_BUS_AVB = 0x62,
    INTERNAL_FREQ_FINE = 0x63,
    INT_OIS = 0x6F,
    CTRL1_OIS = 0x70,
    CTRL2_OIS = 0x71,
    CTRL3_OIS = 0x72,
    X_OFS_USR = 0x73,
    Y_OFS_USR = 0x74,
    Z_OFS_USR = 0x75,
    FIFO_DATA_OUT_TAG = 0x78,
    FIFO_DATA_OUT_X_L = 0x79,
    FIFO_DATA_OUT_X_H = 0x7A,
    FIFO_DATA_OUT_Y_L = 0x7B,
    FIFO_DATA_OUT_Y_H = 0x7C,
    FIFO_DATA_OUT_Z_L = 0x7D,
    FIFO_DATA_OUT_Z_H = 0x7E,
}

// === WHO_AM_I (0Fh) ===
pub const DEVICE_ID: u8 = 0b01101100;
// === CTRL1_XL (10h) ===
// Acclerometer control register 1
pub const LPF2_XL_EN: u8 = 0b0000_0010;

/// Accelerometer Output Data Rate
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum AccOdr {
    /// Power_down
    PowerDown = 0b0000_0000,
    /// 1.6 Hz
    Hz1_6 = 0b1011_0000,
    /// 12.5 Hz
    Hz12_5 = 0b0001_0000,
    /// 26 Hz
    Hz26 = 0b0010_0000,
    /// 52 Hz
    Hz52 = 0b0011_0000,
    /// 104 Hz
    Hz104 = 0b0100_0000,
    /// 208 Hz
    Hz208 = 0b0101_0000,
    /// 416 Hz
    Hz416 = 0b0110_0000,
    /// 833 Hz
    Hz833 = 0b0111_0000,
    /// 1.66 kHz
    KHz1_66 = 0b1000_0000,
    /// 3.33 kHz
    KHz3_33 = 0b1001_0000,
    /// 6.66 kHz
    KHz6_66 = 0b1010_0000,
}

/// Accelerometer full-scale selection
/// If XL_FS_MODE = '1' in CTRL8_XL max scale 8g, g16 becomes 2g see reference manual
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum AccScale {
    /// 2g
    G2 = 0b0000_0000,
    /// 16g
    G16 = 0b0000_0100,
    /// 4g
    G4 = 0b0000_1000,
    /// 8g
    G8 = 0b0000_1100,
}

// === CTRL2_G (11h) ===
// Acclerometer control register 1
pub const FS_125: u8 = 0b0000_0010;

/// Gyroscope Output Data Rate
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum GyrOdr {
    /// Power_down
    PowerDown = 0b0000_0000,
    /// 1.6 Hz
    Hz1_6 = 0b1011_0000,
    /// 12.5 Hz
    Hz12_5 = 0b0001_0000,
    /// 26 Hz
    Hz26 = 0b0010_0000,
    /// 52 Hz
    Hz52 = 0b0011_0000,
    /// 104 Hz
    Hz104 = 0b0100_0000,
    /// 208 Hz
    Hz208 = 0b0101_0000,
    /// 416 Hz
    Hz416 = 0b0110_0000,
    /// 833 Hz
    Hz833 = 0b0111_0000,
    /// 1.66 kHz
    KHz1_66 = 0b1000_0000,
    /// 3.33 kHz
    KHz3_33 = 0b1001_0000,
    /// 6.66 kHz
    KHz6_66 = 0b1010_0000,
}

/// Gyroscope full-scale selection
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum GyrScale {
    /// 250 dps
    Dps250 = 0b0000_0000,
    /// 500 dps
    Dps500 = 0b0000_0100,
    /// 1000 dps
    Dps1000 = 0b0000_1000,
    /// 2000 dps
    Dps2000 = 0b0000_1100,
}

// === CTRL3_C (12h) ===
pub const BOOT: u8 = 0b1000_0000;
pub const BDU: u8 = 0b0100_0000;
pub const H_LACTIVE: u8 = 0b0010_0000;
pub const PP_OD: u8 = 0b0001_0000;
pub const SIM: u8 = 0b0000_1000;
pub const IF_INC: u8 = 0b0000_0100;
pub const SW_RESET: u8 = 0b0000_0001;

// === CTRL4_C (13h) ===
pub const SLEEP_G: u8 = 0b0100_0000;
pub const INT2_on_INT1: u8 = 0b0010_0000;
pub const DRDY_MASK: u8 = 0b0000_1000;
pub const I2C_dis: u8 = 0b0000_0100;
pub const LPF1_SEL_G: u8 = 0b0000_0010;

// === CTRL5_C (14h) ===
pub const XL_ULP_EN: u8 = 0b1000_0000;

/// Circular burst-mode (rounding) read from the output registers. Default value: 00
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum Rounding {
    /// no rounding default
    NoRounding = 0b0000_0000,
    /// accelerometer only
    AccOnly = 0b0010_0000,
    /// gyroscope only
    GyrOnly = 0b0100_0000,
    /// gyroscope + accelerometer
    Both = 0b0110_0000,
}

/// Linear acceleration sensor self-test mode selection
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum AccSeflTest {
    /// Normal mode
    Normal = 0b0000_0000,
    /// Positive sign self-test
    Positive = 0b0000_0001,
    /// Negative sign self-test
    Negative = 0b0000_0010,
}

/// Angular rate sensor self-test mode selection
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum GyroSeflTest {
    /// Normal mode
    Normal = 0b0000_0000,
    /// Positive sign self-test
    Positive = 0b0000_0100,
    /// Negative sign self-test
    Negative = 0b0000_1100,
}

/// === CTRL6_C (15h) ===
pub const XL_HM_MODE: u8 = 0b0001_0000;
pub const USR_OFF_W: u8 = 0b0000_1000;

/// Trigger mode selection
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum TriggerMode {
    /// Edge-sensitive trigger mode is selected
    Edge = 0b0010_0000,
    /// Level-sensitive trigger mode is selected
    LevelTrigger = 0b0100_0000,
    /// Level-sensitive latched mode is selected
    LevelLatched = 0b0110_0000,
    /// Level-sensitive FIFO enable mode is selected
    LevelFifo = 0b1100_0000,
}

/// Gyroscope LPF1 bandwith
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum GyroLpfLevel {
    /// Hz        12.5  26   52  104  208  416  833   1.67k 3.33k 6.67k
    /// Low pass1 4.2  8.3 16.6 33.0 67.0 136.6 239.2 304.2 328.5 335.5
    LevelOne = 0b0000_0000,
    /// Level two 4.2 8.3 16.6 33.0 67.0 130.5 192.4 220.7 229.6 232.0
    LevelTwo = 0b0000_0001,
    /// Level     4.2 8.3 16.6 33.0 67.0 120.3 154.2 166.6 170.1 171.1
    LevelThree = 0b0000_0010,
    /// Level     4.2 8.3 16.6 33.0 67.0 137.1 281.8 453.2 559.2 609.0
    LevelFour = 0b0000_0011,
    /// Level     4.2 8.3 16.7 33.0 62.4  86.7  96.6  99.6    NA    NA
    LevelFive = 0b0000_0100,
    /// Level     4.2 8.3 16.8 31.0 43.2  48.0  49.4  49.8    NA    NA
    LevelSix = 0b0000_0101,
    /// Level     4.1 7.8 13.4 19.0 23.1  24.6  25.0  25.1    NA    NA
    LevelEight = 0b0000_0110,
    /// Level     3.9 6.7  9.7 11.5 12.2  12.4  12.5  12.5    NA    NA
    LevelNine = 0b0000_0111,
}

/// === CTRL7_G (16h) ===
pub const G_HM_MODE: u8 = 0b1000_0000;
pub const HP_EN_G: u8 = 0b0100_0000;
pub const OIS_ON_EN: u8 = 0b0000_0100;
pub const USR_OFF_ON_OUT: u8 = 0b0000_0010;
pub const OIS_ON: u8 = 0b0000_0001;

/// Gyroscope High Pass filter
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum GyroHpfCutoff {
    /// Low pass1 4.2  8.3 16.6 33.0 67.0 136.6 239.2 304.2 328.5 335.5
    MilliHz16 = 0b0000_0000,
    /// Level two 4.2 8.3 16.6 33.0 67.0 130.5 192.4 220.7 229.6 232.0
    MilliHz65 = 0b0001_0000,
    /// Level     4.2 8.3 16.6 33.0 67.0 120.3 154.2 166.6 170.1 171.1
    MilliHz260 = 0b0010_0000,
    /// Level     4.2 8.3 16.6 33.0 67.0 137.1 281.8 453.2 559.2 609.0
    Hz1 = 0b0011_0000,
}

/// === CTRL8_XL (17h) ===
pub const HP_REF_MODE_XL: u8 = 0b0001_0000;
pub const XL_FS_MODE: u8 = 0b0000_0010;
pub const LOW_PASS_ON_6D: u8 = 0b0000_0001;

/// Accelerometer bandwidth configuration
#[derive(Copy, Clone)]
#[cfg_attr(feature = "out_f32", derive(FromPrimitive))]
pub enum AccBandwidth {
    /// Low pass
    /// HP_SLOPE_XL_EN = 0
    /// ODR/2 must set LPF2_XL_EN to 0 in CTRL1_XL
    /// LPF2_XL_EN to 1 in CTRL1_XL
    /// ODR/4
    LowOdr4 = 0b0000_0000,
    /// ODR/10
    LowOdr10 = 0b0010_0000,
    /// ODR/20
    LowOdr20 = 0b0100_0000,
    /// ODR/45
    LowOdr45 = 0b0110_0000,
    /// ODR/100
    LowOdr100 = 0b1000_0000,
    /// ODR/200
    LowOdr200 = 0b1010_0000,
    /// ODR/400
    LowOdr400 = 0b1100_0000,
    /// ODR/800
    LowOdr800 = 0b1110_0000,
    /// High pass HP_SLOPE_XL_EN = 1
    /// ODR/4
    HighOdr4 = 0b0000_0100,
    /// ODR/10
    HighOdr10 = 0b0010_0100,
    /// ODR/20
    HighOdr20 = 0b0100_0100,
    /// ODR/45
    HighOdr45 = 0b0110_0100,
    /// ODR/100
    HighOdr100 = 0b1000_0100,
    /// ODR/200
    HighOdr200 = 0b1010_0100,
    /// ODR/400
    HighOdr400 = 0b1100_0100,
    /// ODR/800
    HighOdr800 = 0b1110_0100,
}

/// === CTRL9_XL (18h) ===
pub const DEN_X: u8 = 0b1000_0000;
pub const DEN_Y: u8 = 0b0100_0000;
pub const DEN_Z: u8 = 0b0010_0000;
pub const DEN_XL_G: u8 = 0b0001_0000;
pub const DEN_XL_EN: u8 = 0b0000_1000;
pub const DEN_LH: u8 = 0b0000_0100;
pub const I3C_disable: u8 = 0b0000_0010;

/// === CTRL10_C (19h) ===
pub const TIMESTAMP_EN: u8 = 0b0010_0000;

/// === ALL_INT_SRC (1Ah) ===
pub const TIMESTAMP_ENDCOUNT: u8 = 0b1000_0000;
pub const SLEEP_CHANGE_IA: u8 = 0b0010_0000;
pub const D6D_IA: u8 = 0b0001_0000;
pub const DOUBLE_TAP: u8 = 0b0000_1000;
pub const SINGLE_TAP: u8 = 0b0000_0100;
pub const WU_IA: u8 = 0b0000_0010;
pub const FF_IA: u8 = 0b0000_0001;
