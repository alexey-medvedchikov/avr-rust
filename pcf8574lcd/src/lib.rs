#![no_std]

use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c::Write;

pub struct Device<I2C> {
    i2c: I2C,
    addr: u8,
    display_cols: u8,
    flags: Flag,
}

impl<I2C, E> Device<I2C>
where
    I2C: Write<Error = E>,
{
    pub fn new(i2c: I2C, addr: u8, display_cols: u8) -> Self {
        Device {
            i2c,
            addr,
            display_cols,
            flags: 0,
        }
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }

    pub fn init(&mut self, delay: &mut impl DelayUs<u16>) -> Result<(), E> {
        delay.delay_us(20_000 as u16);
        self.write_nibble(0x03, 0x00)?;
        delay.delay_us(5_000 as u16);
        self.write_nibble(0x03, 0x00)?;
        delay.delay_us(150 as u16);
        self.write_nibble(0x03, 0x00)?;
        self.write_nibble(0x02, 0x00)
    }

    fn write_nibble(&mut self, nibble: u8, flags: Flag) -> Result<(), E> {
        let nibble_high = (nibble << 4) | flags | FLAG_ENABLE;
        self.i2c.write(self.addr, &[nibble_high])?;
        let nibble_low = (nibble << 4) | flags;
        self.i2c.write(self.addr, &[nibble_low])
    }

    fn write(&mut self, data: u8, flags: Flag) -> Result<(), E> {
        self.write_nibble(data >> 4, flags)?;
        self.write_nibble(data, flags)
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn clear_display(&mut self) -> Result<(), E> {
        self.write(0x01, self.flags)
    }

    pub fn return_home(&mut self) -> Result<(), E> {
        self.write(0x02, self.flags)
    }

    pub fn entry_mode_set(&mut self, mode: EntryMode) -> Result<(), E> {
        self.write(0x04 | mode, self.flags)
    }

    pub fn display_control(&mut self, mode: DisplayControl) -> Result<(), E> {
        self.write(0x08 | mode, self.flags)
    }

    pub fn shift_mode(&mut self, mode: ShiftMode) -> Result<(), E> {
        self.write(0x10 | mode, self.flags)
    }

    pub fn func_set(&mut self, mode: FuncSet) -> Result<(), E> {
        self.write(0x20 | mode, self.flags)
    }

    pub fn write_char(&mut self, ch: u8) -> Result<(), E> {
        self.write(ch, REG_DATA | self.flags)
    }

    pub fn write_str(&mut self, s: &[u8]) -> Result<(), E> {
        for ch in s {
            self.write(*ch, self.flags)?;
        }
        Ok(())
    }

    pub fn set_cursor(&mut self, col: u8, row: u8) -> Result<(), E> {
        let base_addr = match row {
            1 => 0x40,
            2 => 0x00 + self.display_cols,
            _ => 0x00,
        };
        let nibble = 0x80 | (base_addr + col);
        self.write(nibble, self.flags)
    }
}

pub const DEFAULT_DEVICE_ADDR: u8 = 0x27;

type Flag = u8;
const REG_DATA: Flag = 0x01;
const FLAG_ENABLE: Flag = 0x04;
pub const FLAG_BACKLIGHT: Flag = 0x08;
pub const FLAG_NOBACKLIGHT: Flag = 0x00;

type EntryMode = u8;
pub const ENTRY_MODE_SET_LEFT: EntryMode = 0x00;
pub const ENTRY_MODE_SET_RIGHT: EntryMode = 0x02;
pub const ENTRY_MODE_SET_NOSHIFT: EntryMode = 0x00;
pub const ENTRY_MODE_SET_SHIFT: EntryMode = 0x01;

type DisplayControl = u8;
pub const DISPLAY_CONTROL_OFF: DisplayControl = 0x00;
pub const DISPLAY_CONTROL_ON: DisplayControl = 0x04;
pub const DISPLAY_CONTROL_NOCURSOR: DisplayControl = 0x00;
pub const DISPLAY_CONTROL_CURSOR: DisplayControl = 0x02;
pub const DISPLAY_CONTROL_NOBLINK: DisplayControl = 0x00;
pub const DISPLAY_CONTROL_BLINK: DisplayControl = 0x02;

type ShiftMode = u8;
pub const SHIFT_MODE_CURSOR: ShiftMode = 0x00;
pub const SHIFT_MODE_DISPLAY: ShiftMode = 0x08;
pub const SHIFT_MODE_LEFT: ShiftMode = 0x00;
pub const SHIFT_MODE_RIGHT: ShiftMode = 0x04;

type FuncSet = u8;
pub const FUNC_SET_1LINE: FuncSet = 0x00;
pub const FUNC_SET_2LINE: FuncSet = 0x08;
pub const FUNC_SET_5X8FONT: FuncSet = 0x00;
pub const FUNC_SET_5X10FONT: FuncSet = 0x04;
