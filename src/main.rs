extern crate hidapi;

mod devices;

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
        if device_info.vendor_id == VID && device_info.product_id == PID {
            println!("Found {:?} on {} interface {}", device_info.product_string, device_info.path, device_info.interface_number);
        }
    }
    let device = hid.open(VID, PID).unwrap();

    // let device = open_device(VID, PID);

    let data = [
        0x00,
        0x05, 0x00, 0x00, 
        0x10, 0x10, 0x10,
        0xff, 0x32, 0xc8, 0xc8, 0x00, 0x00, 0x01];
    let res = device.send_feature_report(&data).unwrap();
    println!("{:#?}", res);
}
