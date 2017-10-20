use super::MouseProtocol;
use hidapi::HidDevice;

const NUM_PROFILES: u16 = 1;
const NUM_BUTTONS: u16 = 6;
const NUM_DPI: u16 = 2;
const NUM_LED: u16 = 2;
const DPI_MIN: u16 = 50;
const DPI_MAX: u16 = 6500;
const DPI_MULTIPLIER: u8 = 50;

/* not sure these are used for */
const REPORT_ID_1: u8 =	0x01;
const REPORT_ID_2: u8 =	0x02;

const REPORT_SIZE: u8 =	64;

const ID_DPI: u8 = 0x03;
const ID_REPORT_RATE: u8 = 0x04;
const ID_LED_EFFECT: u8 = 0x07;
const ID_LED: u8 = 0x08;
const ID_SAVE: u8 = 0x09;

pub struct Rival310 {}
impl MouseProtocol for Rival310 {
    const NAME: &'static str = "rival, 100, 300, 300csgofadeedition, 300csgohyperbeastedition";
    fn write_led(hid_device: HidDevice, led_index: u8, colors: &[u8], speed: u16) -> Result<(), &'static str> {
        Ok(())
    }
    fn write_dpi(hid_device: HidDevice, res_index: u8, dpi: u16) -> Result<(), &'static str> {
        Ok(())
    }
    fn write_report_rate(hid_device: HidDevice, hz: u16) -> Result<(), &'static str> {
        Ok(())
    }
}