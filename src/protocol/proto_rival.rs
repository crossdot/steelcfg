use super::PROTOCOL;

const STEELSERIES_NUM_PROFILES: u16 = 1;
const STEELSERIES_NUM_BUTTONS: u16 = 6;
const STEELSERIES_NUM_DPI: u16 = 2;
const STEELSERIES_NUM_LED: u16 = 2;
const STEELSERIES_DPI_MIN: u16 = 50;
const STEELSERIES_DPI_MAX: u16 = 6500;

/* not sure these are used for */
const STEELSERIES_REPORT_ID_1: u8 =	0x01;
const STEELSERIES_REPORT_ID_2: u8 =	0x02;

const STEELSERIES_REPORT_SIZE: u8 =	64;

const STEELSERIES_ID_DPI: u8 = 0x03;
const STEELSERIES_ID_REPORT_RATE: u8 = 0x04;
const STEELSERIES_ID_LED_EFFECT: u8 = 0x07;
const STEELSERIES_ID_LED: u8 = 0x08;
const STEELSERIES_ID_SAVE: u8 = 0x09;

pub static protocol : PROTOCOL = PROTOCOL {
    name: "rival, rival100, rival300, rival300csgofadeedition, rival300csgohyperbeastedition",
};