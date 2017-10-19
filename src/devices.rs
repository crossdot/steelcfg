/// The `Type` of command data.
pub enum Type {
	/// Command has no data.
	None,
	Color
}

/// The `ReportType` of command data.
pub enum ReportType {
	Command,
	ReportFeature
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
	pub data_type: Type,
	/// `ReportType` of command data.
	pub report_type: ReportType
}

pub struct Device {
    /// Name
    name: &'static str,
    /// Vendor id
    vid: u16,
    /// Product id
    pid: u16,
    /// Product id
    interface_number: u16,
    /// Device supported command list
    commands: &'static [Command],
}

/// list of available `devices`.
pub static DEVICE : &'static [Device] = &[
    Device {
        name: "SteelSeries Rival 700 (Experimental)",
        vid: 0x1038,
        pid: 0x1700,
        interface_number: 0,
        commands: &[
            Command {
                name: "Wheel color",
                description: "Changes wheel color to specified.",
                command: "wheel-color",
                usb_command_begin: &[0x05_u8, 0x00_u8, 0x01_u8],
                usb_command_end: Some(&[0xff_u8, 0x32_u8, 0xc8_u8, 0xc8_u8, 0x00_u8, 0x01_u8, 0x01_u8]),
                data_type: Type::Color,
                report_type: ReportType::ReportFeature,
            },
            Command {
                name: "Logo color",
                description: "Changes logo color to specified.",
                command: "logo-color",
                usb_command_begin: &[0x05_u8, 0x00_u8, 0x00_u8],
                usb_command_end: Some(&[0xff_u8, 0x32_u8, 0xc8_u8, 0xc8_u8, 0x00_u8, 0x00_u8, 0x01_u8]),
                data_type: Type::Color,
                report_type: ReportType::ReportFeature,
            }
        ]
    }
];
