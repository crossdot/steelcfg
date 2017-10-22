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
    /*
    let data = [
        0x00, // init report
        0x05, // command
        0x00, // ? [00]
        0x00, // led index
        0x10, 0x10, 0x10, // color 1
        // 0x40, 0x40, 0x40, // color 2 [optional]
        0x70, 0x00, // speed (little endian)
        0x00, // ? [any]
        0xc8, // if other value - color 2 is not usable
        0x00, // [enable]
        0x00, // led index
        0x01, // [enable flag]
    ];
    let res = device.send_feature_report(&data).unwrap();
    println!("{:?}", data);
    println!("{:?}", res);
    */

    use protocol::MouseProtocol;
    use protocol::proto_rival500::Rival500;
    println!("{:?}", Rival500::write_led(&device, 0, &[0x10, 0x10, 0x10, 0x30, 0x30, 0x30], 80));
    println!("{:?}", Rival500::write_led(&device, 1, &[0x10, 0x10, 0x10, 0x30, 0x30, 0x30], 80));
    println!("{:?}", Rival500::write_dpi(&device, 0, 1600));
    println!("{:?}", Rival500::write_dpi(&device, 1, 3200));
    println!("{:?}", Rival500::write_report_rate(&device, 1000));
    println!("{:?}", Rival500::save(&device));
}

#[test]
fn testProto() {
    use protocol::MouseProtocol;
    use protocol::proto_rival500::Rival500;

    let proto = Rival500{};
    println!("{:?}", Rival500::NAME)
}