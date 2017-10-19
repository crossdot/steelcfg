/// The `Type` of command data.
pub enum Type {
	/// Command has no data.
	None,
	Color
}

pub struct Command {
    /// Command `name`.
	pub name: &'static str,
	///Command `description`.
	pub description: &'static str,
    /// Console `command`.
	command: &'static str,
    /// Bytes of usb command beginning.
    pub usb_command_begin: &'static [u8],
    /// Bytes of usb command beginning.
    pub usb_command_end: Option<&'static [u8]>,
	/// `Type` of command data.
	pub data_type: Type
}

pub struct Device {
    /// Vendor id
    vid: u16,
    /// Product id
    pid: u16,
    /// Device supported command list
    commands: &'static [Command],
}

/// list of available `devices`.
pub static DEVICE : &'static [Device] = &[
    Device {
        vid: 0x1038,
        pid: 0x1700,
        commands: &[
            Command {
                name: "Wheel color",
                description: "Changes wheel color to specified.",
                command: "wheel-color",
                usb_command_begin: &[0x05_u8, 0x00_u8, 0x01_u8],
                usb_command_end: Some(&[0xff_u8, 0x32_u8, 0xc8_u8, 0xc8_u8, 0x00_u8, 0x01_u8, 0x01_u8]),
                data_type: Type::Color,
            },
            Command {
                name: "Logo color",
                description: "Changes logo color to specified.",
                command: "logo-color",
                usb_command_begin: &[0x05_u8, 0x00_u8, 0x00_u8],
                usb_command_end: Some(&[0xff_u8, 0x32_u8, 0xc8_u8, 0xc8_u8, 0x00_u8, 0x00_u8, 0x01_u8]),
                data_type: Type::Color,
            }
        ]
    }
];
