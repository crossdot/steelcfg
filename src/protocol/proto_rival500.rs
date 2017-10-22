use super::MouseProtocol;
use hidapi::HidDevice;
use utils;

const NUM_PROFILES: u16 = 1;
const NUM_BUTTONS: u16 = 6;
const NUM_DPI: u16 = 2;
const NUM_LED: u16 = 2;
const DPI_MIN: u16 = 100;
const DPI_MAX: u16 = 16000;

/* not sure these are used for */
const REPORT_ID_1: u8 =	0x01;
const REPORT_ID_2: u8 =	0x02;

const REPORT_SIZE: usize =	64;

const ID_DPI: u8 = 0x03;
const ID_REPORT_RATE: u8 = 0x04;
const ID_LED: u8 = 0x05;
const ID_SAVE: u8 = 0x09;

pub enum ColorMode {
    Steady,
    ColorShift,
    MultiModeBreath,
    Trigger,
    DisableIllumination,
}

pub struct Rival500 {}
impl MouseProtocol for Rival500 {
    const NAME: &'static str = "rival 500, 700";
    fn write_led(hid_device: &HidDevice, led_index: u8, colors: &[u8], speed: u16) -> Result<(), &'static str> {
        // buf[0] = 0x05;
        // buf[1] = 0x00;
        // buf[2] = 0x00;
        // buf[3] = 0x50;
        // buf[4] = 0x10;
        // buf[5] = 0x10;
        // buf[6] = 0x70;
        // buf[7] = 0x00;
        // buf[8] = 0x00;
        // buf[9] = 0xc8;
        // buf[10] = 0x00;
        // buf[11] = 0x00;
        // buf[12] = 0x01;



        let mut data = vec![
            0x00, // init report
            ID_LED, // command
            0x00, // ? [00]
            led_index, // led index
            // 0x10, 0x10, 0x10, // color 1
            // 0x40, 0x40, 0x40, // color 2 [optional]
            // 0x70, 0x00, // speed (little endian)
        ];
        data.extend(colors);
        data.extend(utils::to_little_endian((speed / 10) as u16));
        data.extend(&[
            0x00, // ? [any]
            0xc8, // if other value - color 2 is not usable
            0x00, // [enable]
            led_index, // led index
            0x01, // [enable flag]
        ]);
        let res = hid_device.send_feature_report(&data).unwrap();

        // // 0 - 2 colors (blue/red ; purpur/green ; yellow?/cyan?)
        // let mut buf: [u8; 512] = [0; 512];
        // buf[0] = ID_LED;
        // buf[1] = 0x00;
        // buf[2] = led_index;
        // buf[3] = 0xff;
        // buf[4] = 0x52;
        // buf[5] = 0x00;
        // buf[6] = 0xff;
        // buf[7] = 0x32;
        // buf[8] = 0xc8;
        // buf[9] = 0xc8;
        // buf[10] = 0x00;
        // buf[11] = led_index;
        // buf[12] = 0x00;

        // // transitions

        // // some color diff
        // buf[13] = (-2i8) as u8;
        // buf[14] = (2i8) as u8;
        // buf[15] = (0i8) as u8;

        // // transition 01
        // buf[16] = 0x00;
        // buf[17] = 0x55; // time (little endian)
        // buf[18] = 0x01; //
        // buf[19] = 0x01; // goto
        // buf[20] = (0i8) as u8; // brihtness? diff

        // // some color diff
        // buf[21] = (0i8) as u8;
        // buf[22] = (-2i8) as u8;
        // buf[23] = (2i8) as u8;

        // // transition 02
        // buf[24] = 0x00; // wat? above 80 - white blinking light
        // buf[25] = 0x55; // time (little endian)
        // buf[26] = 0x01; //
        // buf[27] = 0x02; // goto
        // buf[28] = (0i8) as u8; // brihtness? diff

        // // some color diff
        // buf[29] = (2i8) as u8;
        // buf[30] = (0i8) as u8;
        // buf[31] = (-2i8) as u8;
        
        // // transition 03 to origin
        // buf[32] = 0x00; // wat? above 80 - white blinking light
        // buf[33] = 0x55; // time (little endian)
        // buf[34] = 0x01; //
        // buf[35] = (0i8) as u8; // brihtness? diff

        // // accumulator
        // buf[141] = 0x80;
        // buf[142] = 0x02; // starting color
        // buf[143] = 0x80;
        // buf[144] = 0x00; // starting color
        // buf[145] = 0x80;
        // buf[146] = 0x00; // starting color
        // buf[147] = 0xff; // brightness
        // buf[148] = 0x00; // wat? above 80 - white blinking light

        // buf[149] = 0xdc;
        // buf[150] = 0x05;
        // buf[151] = 0x8a;
        // buf[152] = 0x02;
        // buf[153] = 0x00;
        // buf[154] = 0x00;
        // buf[155] = 0x00;
        // buf[156] = 0x00;
        // buf[157] = 0x01;
        // buf[158] = 0x00;
        // buf[159] = 0x03;
        // buf[160] = 0x00;
        // buf[161] = 0xe8; // full time (little endian)
        // buf[162] = 0x03; //
        // buf[163] = 0x00;

        // let res = hid_device.send_feature_report(&buf).unwrap();
        Ok(())
    }
    fn write_dpi(hid_device: &HidDevice, res_index: u8, dpi: u16) -> Result<(), &'static str> {
        let mut buf: [u8; 32] = [0; 32];
        buf[0] = ID_DPI;
        buf[1] = 0x00;
        buf[2] = res_index + 1;
        buf[3] = (dpi / 100 - 1) as u8;
        buf[4] = 0x00;
        buf[5] = 0x42; /* not sure if needed */
        let res = hid_device.write(&buf).unwrap();

        // let mut buf: [u8; 32] = [0; 32];
        // buf[0] = ID_DPI;
        // buf[1] = 0x00;
        // buf[2] = res_index + 1;
        // buf[3] = 0x10;
        // buf[4] = 0x00;
        // buf[5] = 0x33;
        // let res = hid_device.write(&buf).unwrap();

        // let mut buf: [u8; 32] = [0; 32];
        // buf[0] = ID_DPI;
        // buf[1] = 0x00;
        // buf[2] = res_index + 1;
        // buf[3] = 0x10;
        // buf[4] = 0x00;
        // buf[5] = 0x3d;
        // let res = hid_device.write(&buf).unwrap();
        Ok(())
    }
    fn write_report_rate(hid_device: &HidDevice, hz: u16) -> Result<(), &'static str> {
        let mut buf: [u8; 32] = [0; 32];
        buf[0] = ID_REPORT_RATE;
        buf[1] = 0x00;
        buf[2] = (1000 / hz) as u8;
        let res = hid_device.write(&buf).unwrap();
        Ok(())
    }
    fn save(hid_device: &HidDevice) -> Result<(), &'static str> {
        let mut buf: [u8; 32] = [0; 32];
        buf[0] = 0x09;
        let res = hid_device.write(&buf).unwrap();
        Ok(())
    }
}

// looks like rival 100

// led1(mousewheel)
// 0x05 0x00 [led_index] [r] [g] [b] 0xFF 0x32 0xC8 0xC8 0x00 [led_index] 0x01
// 0x05 0x00 [led_index]                       0xC8 0xC8 0x00 [led_index] 0x01

// sens1 1200
// 0x03 0x00 0x01 0b 00 42