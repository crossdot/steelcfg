pub mod proto_rival;
pub mod proto_rival310;
pub mod proto_rival500;

use hidapi::HidDevice;

pub enum ColorDepth {
    RGB_888,
}
pub enum LedType {
    UNKNOWN
}

pub struct Led {
    refcount: u8,
    // profile
    index: u8,
    colordepth: ColorDepth,
    ledtype: LedType,
}

pub trait MouseProtocol {
    const NAME: &'static str;
    fn write_led(hid_device: HidDevice, led_index: u8, colors: &[u8], speed: u16) -> Result<(), &'static str>;
    fn write_dpi(hid_device: HidDevice, res_index: u8, dpi: u16) -> Result<(), &'static str>;
    fn write_report_rate(hid_device: HidDevice, hz: u16) -> Result<(), &'static str>;
}