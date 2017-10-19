extern crate hidapi;
extern crate byteorder;

mod devices;
mod protocol;
mod utils;

const VID: u16 = 0x1038;
const PID: u16 = 0x1700;

// fn open_device<'a>(vid: u16, pid: u16) -> hidapi::HidDevice<'a> {
//     let hid = hidapi::HidApi::new().unwrap();
//     let device = hid.open(VID, PID).unwrap();
//     device
// }

fn main() {
    let hid = hidapi::HidApi::new().unwrap();
    for device_info in hid.devices() {
        if (device_info.vendor_id == VID && device_info.product_id == PID) {
            if let Some(product_string) = device_info.product_string {
                println!("Found {} on {} interface {}", product_string, device_info.path, device_info.interface_number);
            }
        }
    }
    let device = hid.open(VID, PID).unwrap();

    // let device = open_device(VID, PID);

    let data = [
        0x00, // init?
        0x05, // command
        0x00,
        0x00, // led index
        0x40, 0x40, 0x10, // color
        0x10, 0x40, 0x10, // color
        0x10, 0x10, 0x10, // ?? 3rd color not works
        0xff, 0xff, // 0xff, 0x38
        0xc8, 0xc8, 0x00,
        0x00, // led index
        0x01];
    let res = device.send_feature_report(&data).unwrap();
    println!("{:#?}", data);
    println!("{:#?}", res);
}
