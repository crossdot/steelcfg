use super::PROTOCOL;

const STEELSERIES_NUM_PROFILES: u16 = 1;
const STEELSERIES_NUM_BUTTONS: u16 = 6;
const STEELSERIES_NUM_DPI: u16 = 2;
const STEELSERIES_NUM_LED: u16 = 2;
const STEELSERIES_DPI_MIN: u16 = 100;
const STEELSERIES_DPI_MAX: u16 = 12000;

/* not sure these are used for */
const STEELSERIES_REPORT_ID_1: u8 =	0x01;
const STEELSERIES_REPORT_ID_2: u8 =	0x02;

const STEELSERIES_REPORT_SIZE: u8 =	64;

const STEELSERIES_ID_DPI: u8 = 0x53;
const STEELSERIES_ID_REPORT_RATE: u8 = 0x54;
const STEELSERIES_ID_LED: u8 = 0x5b;
const STEELSERIES_ID_SAVE: u8 = 0x59;

pub static protocol : PROTOCOL = PROTOCOL {
    name: "rival500, rival700",
};