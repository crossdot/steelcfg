use super::MouseProtocol;
use hidapi::HidDevice;
use utils;

const NUM_PROFILES: u16 = 1;
const NUM_BUTTONS: u16 = 6;
const NUM_DPI: u16 = 2;
const NUM_LED: u16 = 2;
const DPI_MIN: u16 = 100;
const DPI_MAX: u16 = 12000;

/* not sure these are used for */
const REPORT_ID_1: u8 =	0x01;
const REPORT_ID_2: u8 =	0x02;

const REPORT_SIZE: usize = 64;

const ID_DPI: u8 = 0x53;
const ID_REPORT_RATE: u8 = 0x54;
const ID_LED: u8 = 0x5b;
const ID_SAVE: u8 = 0x59;

pub struct Rival310 {}
impl MouseProtocol for Rival310 {
    const NAME: &'static str = "rival 310";
    fn write_led(hid_device: &HidDevice, led_index: u8, colors: &[u8], speed: u16) -> Result<(), &'static str> {
        // not sure why the rate is set for the steady color or why it is different for the two LEDs
        let rate = if led_index == 0 { 10000 } else { 5000 }; /* 0x1027 or 0x8813 */
        let rate_le = utils::to_little_endian(rate);

        let mut buf: [u8; REPORT_SIZE] = [0; REPORT_SIZE];
        buf[0] = ID_LED;
        buf[2] = led_index;

        buf[3] = rate_le[0];
        buf[4] = rate_le[1];
        buf[5] = rate_le[2];

        // not sure if these four are needed either
        buf[15] = 0x01;
	    buf[17] = 0x01;
	    buf[19] = 0x01;
        buf[27] = 0x01;

        buf[28] = colors[0];
        buf[29] = colors[1];
        buf[30] = colors[2];

        buf[31] = colors[0];
        buf[32] = colors[1];
        buf[33] = colors[2];

        // data.extend(colors);

        let mut data = vec![
            0x00, // init report
        ];
        data.extend(&buf[..]);

        let res = hid_device.send_feature_report(&data).unwrap();
        Ok(res)
    }
    
    fn write_dpi(hid_device: &HidDevice, res_index: u8, dpi: u16) -> Result<(), &'static str> {
        let mut buf: [u8; REPORT_SIZE] = [0; REPORT_SIZE];
        buf[0] = ID_DPI;
        buf[2] = res_index + 1;
        buf[3] = (dpi / 100 - 1) as u8;
        buf[6] = 0x42; /* not sure if needed */

        let res = hid_device.write(&buf).unwrap();
        Ok(())
    }
    
    fn write_report_rate(hid_device: &HidDevice, hz: u16) -> Result<(), &'static str> {
        let mut buf: [u8; REPORT_SIZE] = [0; REPORT_SIZE];
        buf[0] = ID_REPORT_RATE;
        buf[2] = (1000 / hz) as u8;

        let res = hid_device.write(&buf).unwrap();
        Ok(())
    }
    
    fn save(hid_device: &HidDevice) -> Result<(), &'static str> {
        Ok(())
    }
}