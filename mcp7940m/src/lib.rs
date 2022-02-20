#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

macro_rules! read_write_register {
    ($read_fn:ident, $write_fn:ident, $reg:expr, $reg_type:ident) => {
        pub fn $read_fn(&mut self) -> Result<$reg_type, E> {
            self.read($reg).map(|b| $reg_type(b))
        }

        pub fn $write_fn(&mut self, reg: &$reg_type) -> Result<(), E> {
            self.write($reg, reg.0)
        }
    };
}

pub struct Device<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, E> Device<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Device { i2c, addr }
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }

    read_write_register!(read_rtcsec, write_rtcsec, Reg::RtcSec, RtcSec);
    read_write_register!(read_rtcmin, write_rtcmin, Reg::RtcMin, RtcMin);
    read_write_register!(read_rtchour, write_rtchour, Reg::RtcHour, RtcHour);
    read_write_register!(read_rtcwkday, write_rtcwkday, Reg::RtcWkday, RtcWkday);
    read_write_register!(read_rtcdate, write_rtcdate, Reg::RtcDate, RtcDate);
    read_write_register!(read_rtcmth, write_rtcmth, Reg::RtcMth, RtcMth);
    read_write_register!(read_rtcyear, write_rtcyear, Reg::RtcYear, RtcYear);
    read_write_register!(read_control, write_control, Reg::Control, Control);
    read_write_register!(read_osctrim, write_osctrim, Reg::OscTrim, OscTrim);
    read_write_register!(read_alm0sec, write_alm0sec, Reg::Alm0Sec, Alm0Sec);
    read_write_register!(read_alm0min, write_alm0min, Reg::Alm0Min, Alm0Min);
    read_write_register!(read_alm0hour, write_alm0hour, Reg::Alm0Hour, Alm0Hour);
    read_write_register!(read_alm0wkday, write_alm0wkday, Reg::Alm0Wkday, Alm0Wkday);
    read_write_register!(read_alm0date, write_alm0date, Reg::Alm0Date, Alm0Date);
    read_write_register!(read_alm0mth, write_alm0mth, Reg::Alm0Mth, Alm0Mth);
    read_write_register!(read_alm1sec, write_alm1sec, Reg::Alm1Sec, Alm1Sec);
    read_write_register!(read_alm1min, write_alm1min, Reg::Alm1Min, Alm1Min);
    read_write_register!(read_alm1hour, write_alm1hour, Reg::Alm1Hour, Alm1Hour);
    read_write_register!(read_alm1wkday, write_alm1wkday, Reg::Alm1Wkday, Alm1Wkday);
    read_write_register!(read_alm1date, write_alm1date, Reg::Alm1Date, Alm1Date);
    read_write_register!(read_alm1mth, write_alm1mth, Reg::Alm1Mth, Alm1Mth);

    fn read(&mut self, reg: Reg) -> Result<u8, E> {
        let mut buf = [0];
        self.i2c
            .write_read(self.addr, &[reg as u8], &mut buf)
            .and(Ok(buf[0]))
    }

    fn write(&mut self, reg: Reg, byte: u8) -> Result<(), E> {
        let bytes = &[(reg as u8) << 1, byte];
        self.i2c.write(self.addr, bytes)
    }
}

pub const DEFAULT_DEVICE_ADDR: u8 = 0x6f;

enum Reg {
    RtcSec = 0x00,
    RtcMin = 0x01,
    RtcHour = 0x02,
    RtcWkday = 0x03,
    RtcDate = 0x04,
    RtcMth = 0x05,
    RtcYear = 0x06,
    Control = 0x07,
    OscTrim = 0x08,
    Alm0Sec = 0x0A,
    Alm0Min = 0x0B,
    Alm0Hour = 0x0C,
    Alm0Wkday = 0x0D,
    Alm0Date = 0x0E,
    Alm0Mth = 0x0F,
    Alm1Sec = 0x11,
    Alm1Min = 0x12,
    Alm1Hour = 0x13,
    Alm1Wkday = 0x14,
    Alm1Date = 0x15,
    Alm1Mth = 0x16,
}

pub struct RtcSec(u8);
pub struct RtcMin(u8);
pub struct RtcHour(u8);
pub struct RtcWkday(u8);
pub struct RtcDate(u8);
pub struct RtcMth(u8);
pub struct RtcYear(u8);
pub struct Control(u8);
pub struct OscTrim(u8);
pub struct Alm0Sec(u8);
pub struct Alm0Min(u8);
pub struct Alm0Hour(u8);
pub struct Alm0Wkday(u8);
pub struct Alm0Date(u8);
pub struct Alm0Mth(u8);
pub struct Alm1Sec(u8);
pub struct Alm1Min(u8);
pub struct Alm1Hour(u8);
pub struct Alm1Wkday(u8);
pub struct Alm1Date(u8);
pub struct Alm1Mth(u8);

pub enum SquareWaveFreq {
    Hz1,
    Hz4096,
    Hz8192,
    Hz32768,
}

impl Into<u16> for SquareWaveFreq {
    fn into(self) -> u16 {
        match self {
            SquareWaveFreq::Hz1 => 1,
            SquareWaveFreq::Hz4096 => 4096,
            SquareWaveFreq::Hz8192 => 8192,
            SquareWaveFreq::Hz32768 => 32768,
        }
    }
}

macro_rules! sec_funcs {
    () => {
        pub fn get_sec_ones(&self) -> u8 {
            self.0 & 0b0000_1111
        }

        pub fn get_sec_tens(&self) -> u8 {
            (self.0 >> 4) & 0b0000_0111
        }

        pub fn set_sec_ones(&mut self, v: u8) {
            self.0 = (self.0 & 0b1111_0000) | v;
        }

        pub fn set_sec_tens(&mut self, v: u8) {
            self.0 = (self.0 & 0b0111_0000) | (v << 4);
        }
    };
}

macro_rules! min_funcs {
    () => {
        pub fn get_min_ones(&self) -> u8 {
            self.0 & 0b0000_1111
        }

        pub fn get_min_tens(&self) -> u8 {
            (self.0 >> 4) & 0b0000_0111
        }

        pub fn set_min_ones(&mut self, v: u8) {
            self.0 = (self.0 & 0b1111_0000) | v;
        }

        pub fn set_min_tens(&mut self, v: u8) {
            self.0 = (self.0 & 0b0111_0000) | (v << 4);
        }
    };
}

macro_rules! hour_funcs {
    () => {
        pub fn get_hour_ones(&self) -> u8 {
            self.0 & 0b0000_1111
        }

        pub fn get_hour_tens_24(&self) -> u8 {
            (self.0 >> 4) & 0b0000_0011
        }

        pub fn get_hour_tens_12(&self) -> u8 {
            (self.0 >> 4) & 0b0000_0001
        }

        pub fn get_24fmt(&self) -> u8 {
            (self.0 >> 6) & 0b0000_0001
        }

        pub fn get_is_pm(&self) -> u8 {
            (self.0 >> 5) & 0b0000_0001
        }

        pub fn set_hour_ones(&mut self, v: u8) {
            self.0 = (self.0 & 0b1111_0000) | v;
        }

        pub fn set_hour_tens_24(&mut self, v: u8) {
            self.0 = (self.0 & 0b1100_1111) | (v << 4);
        }

        pub fn set_hour_tens_12(&mut self, v: u8) {
            self.0 = (self.0 & 0b1110_1111) | (v << 4);
        }

        pub fn set_24fmt(&mut self, v: u8) {
            self.0 = (self.0 & 0b1011_1111) | (v << 6);
        }

        pub fn set_is_pm(&mut self, v: u8) {
            self.0 = (self.0 & 0b1101_1111) | (v << 5);
        }
    };
}

macro_rules! wkday_funcs {
    () => {
        pub fn get_wkday(&self) -> u8 {
            self.0 & 0b0000_0111
        }

        pub fn set_wkday(&mut self, v: u8) {
            self.0 = (self.0 & 0b1111_1000) | v;
        }
    };
}

macro_rules! date_funcs {
    () => {
        pub fn get_date_ones(&self) -> u8 {
            self.0 & 0b0000_1111
        }

        pub fn get_date_tens(&self) -> u8 {
            (self.0 >> 4) & 0b0000_0011
        }

        pub fn set_date_ones(&mut self, v: u8) {
            self.0 = (self.0 & 0b1111_0000) | v;
        }

        pub fn set_date_tens(&mut self, v: u8) {
            self.0 = (self.0 & 0b1100_1111) | (v << 4);
        }
    };
}

macro_rules! month_funcs {
    () => {
        pub fn get_month_ones(&self) -> u8 {
            self.0 & 0b0000_1111
        }

        pub fn get_month_tens(&self) -> u8 {
            (self.0 >> 4) & 0b0000_0001
        }

        pub fn set_month_ones(&mut self, v: u8) {
            self.0 = (self.0 & 0b1111_0000) | v;
        }

        pub fn set_month_tens(&mut self, v: u8) {
            self.0 = (self.0 & 0b1110_1111) | (v << 4);
        }
    };
}

impl RtcSec {
    sec_funcs!();

    pub fn get_osc_enabled(&self) -> u8 {
        self.0 >> 7
    }

    pub fn set_osc_enabled(&mut self, v: u8) {
        self.0 = (self.0 & 0b0111_1111) | v << 7;
    }
}

impl RtcMin {
    min_funcs!();
}

impl RtcHour {
    hour_funcs!();
}

impl RtcWkday {
    wkday_funcs!();

    pub fn get_osc_run(&self) -> u8 {
        (self.0 >> 5) & 0b0000_0001
    }
}

impl RtcDate {
    date_funcs!();
}

impl RtcMth {
    month_funcs!();

    pub fn get_leap_year(&self) -> u8 {
        (self.0 >> 5) & 0b0000_0001
    }
}

impl RtcYear {
    pub fn get_year_ones(&self) -> u8 {
        self.0 & 0b0000_1111
    }

    pub fn get_year_tens(&self) -> u8 {
        self.0 >> 4
    }

    pub fn set_year_ones(&mut self, v: u8) {
        self.0 = (self.0 & 0b1111_0000) | v;
    }

    pub fn set_year_tens(&mut self, v: u8) {
        self.0 = (self.0 & 0b0000_1111) | (v << 4);
    }
}

impl Control {
    pub fn get_out(&self) -> u8 {
        self.0 >> 7
    }

    pub fn get_sqw_enabled(&self) -> u8 {
        (self.0 >> 6) & 0b0000_0001
    }

    pub fn get_alm1_enabled(&self) -> u8 {
        (self.0 >> 5) & 0b0000_0001
    }

    pub fn get_alm0_enabled(&self) -> u8 {
        (self.0 >> 4) & 0b0000_0001
    }

    pub fn get_ext_osc(&self) -> u8 {
        (self.0 >> 3) & 0b0000_0001
    }

    pub fn get_coarse_trim(&self) -> u8 {
        (self.0 >> 2) & 0b0000_0001
    }

    pub fn get_sqw_freq(&self) -> SquareWaveFreq {
        let v = self.0 & 0b0000_0011;
        match v {
            0b00 => SquareWaveFreq::Hz1,
            0b01 => SquareWaveFreq::Hz4096,
            0b10 => SquareWaveFreq::Hz8192,
            _ => SquareWaveFreq::Hz32768,
        }
    }

    pub fn set_out(&mut self, v: u8) {
        self.0 = (self.0 & 0b0111_1111) | (v << 7);
    }

    pub fn set_sqw_enabled(&mut self, v: u8) {
        self.0 = (self.0 & 0b1011_1111) | (v << 6);
    }

    pub fn set_alm1_enabled(&mut self, v: u8) {
        self.0 = (self.0 & 0b1101_1111) | (v << 5);
    }

    pub fn set_alm0_enabled(&mut self, v: u8) {
        self.0 = (self.0 & 0b1110_1111) | (v << 4);
    }

    pub fn set_ext_osc(&mut self, v: u8) {
        self.0 = (self.0 & 0b1111_0111) | (v << 3);
    }

    pub fn set_coarse_trim(&mut self, v: u8) {
        self.0 = (self.0 & 0b1111_1011) | (v << 2);
    }

    pub fn set_sqw_freq(&mut self, v: SquareWaveFreq) {
        let bits: u8 = match v {
            SquareWaveFreq::Hz1 => 0b00,
            SquareWaveFreq::Hz4096 => 0b01,
            SquareWaveFreq::Hz8192 => 0b10,
            SquareWaveFreq::Hz32768 => 0b11,
        };
        self.0 = (self.0 & 0b1111_1100) & bits;
    }
}

impl OscTrim {
    pub fn get_sign(&self) -> u8 {
        self.0 >> 7
    }

    pub fn get_trim_val(&self) -> u8 {
        self.0 & 0b0111_1111
    }
}

impl Alm0Sec {
    sec_funcs!();
}

impl Alm0Min {
    min_funcs!();
}

impl Alm0Hour {
    hour_funcs!();
}

impl Alm0Wkday {
    wkday_funcs!();
}

impl Alm0Date {
    date_funcs!();
}

impl Alm0Mth {
    month_funcs!();
}

impl Alm1Sec {
    sec_funcs!();
}

impl Alm1Min {
    min_funcs!();
}

impl Alm1Hour {
    hour_funcs!();
}

impl Alm1Wkday {
    wkday_funcs!();
}

impl Alm1Date {
    date_funcs!();
}

impl Alm1Mth {
    month_funcs!();
}
