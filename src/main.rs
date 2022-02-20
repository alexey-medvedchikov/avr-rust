#![no_std]
#![no_main]

use atmega_hal::i2c::I2c;
use atmega_hal::usart::Usart;
use atmega_hal::Peripherals;
use avr_hal_generic::prelude::*;
use avr_hal_generic::usart::Baudrate;
use avr_hal_generic::{clock, delay};

use panic_halt as _;

type Clock = clock::MHz8;
const I2C_SPEED: u32 = 400_000;
const USART_BAUDRATE: u32 = 57_600;

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    let i2c = I2c::<Clock>::new(
        dp.TWI,
        pins.pc4.into_pull_up_input(),
        pins.pc5.into_pull_up_input(),
        I2C_SPEED,
    );
    let mut _serial = Usart::new(
        dp.USART0,
        pins.pd0.into_pull_up_input(),
        pins.pd1.into_output(),
        Baudrate::<Clock>::new(USART_BAUDRATE),
    );
    let shared_i2c = shared_bus::BusManagerSimple::new(i2c);

    let mut lcd = pcf8574lcd::Device::new(
        shared_i2c.acquire_i2c(),
        pcf8574lcd::DEFAULT_DEVICE_ADDR,
        20,
    );
    lcd.init(&mut delay::Delay::<Clock>::new()).unwrap();
    lcd.func_set(pcf8574lcd::FUNC_SET_2LINE | pcf8574lcd::FUNC_SET_5X8FONT)
        .unwrap();
    lcd.set_flags(pcf8574lcd::FLAG_BACKLIGHT);
    lcd.entry_mode_set(pcf8574lcd::ENTRY_MODE_SET_RIGHT | pcf8574lcd::ENTRY_MODE_SET_NOSHIFT)
        .unwrap();
    lcd.display_control(pcf8574lcd::DISPLAY_CONTROL_ON).unwrap();
    lcd.clear_display().unwrap();

    let mut rtc = mcp7940m::Device::new(shared_i2c.acquire_i2c(), mcp7940m::DEFAULT_DEVICE_ADDR);
    let mut rtcsec = rtc.read_rtcsec().unwrap();
    rtcsec.set_osc_enabled(1);
    rtc.write_rtcsec(&rtcsec).unwrap();

    loop {
        let sec = rtc.read_rtcsec().unwrap();
        let min = rtc.read_rtcmin().unwrap();
        let hour = rtc.read_rtchour().unwrap();
        let day = rtc.read_rtcdate().unwrap();
        let month = rtc.read_rtcmth().unwrap();
        let year = rtc.read_rtcyear().unwrap();

        let line = &[
            '0' as u8 + hour.get_hour_tens_24(),
            '0' as u8 + hour.get_hour_ones(),
            ':' as u8,
            '0' as u8 + min.get_min_tens(),
            '0' as u8 + min.get_min_ones(),
            ':' as u8,
            '0' as u8 + sec.get_sec_tens(),
            '0' as u8 + sec.get_sec_ones(),
            ' ' as u8,
            '0' as u8 + year.get_year_tens(),
            '0' as u8 + year.get_year_ones(),
            '.' as u8,
            '0' as u8 + month.get_month_tens(),
            '0' as u8 + month.get_month_ones(),
            '.' as u8,
            '0' as u8 + day.get_date_tens(),
            '0' as u8 + day.get_date_ones(),
        ];

        let mut delay = delay::Delay::<Clock>::new();

        lcd.set_cursor(0, 0).unwrap();
        delay.delay_ms(1 as u16);

        for b in line {
            lcd.write_char(*b).unwrap();
            delay.delay_ms(1 as u16);
        }
    }
}
