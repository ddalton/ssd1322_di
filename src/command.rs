use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

/// ssd1322 Commands

/// Commands - subset of the supported commands
#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    SetColumnAddress(u8, u8),
    SetRowAddress(u8, u8),
    WriteRAM,
    OscFreq(u8),
}

impl Command {
    /// Send command to ssd1322
    pub fn send<DI>(self, iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: WriteOnlyDataCommand,
    {
        // Some commands are not supported, so the maximum array size needed is only 3 u8 along with the
        // real length
        let (data, len) = match self {
            // Set the bounding box
            Command::SetColumnAddress(start, end) => ([0x15, start, end], 3),
            Command::SetRowAddress(start, end) => ([0x75, start, end], 3),

            // Write the data following this command
            Command::WriteRAM => ([0x5C, 0, 0], 1),

            // Increase display frame rate
            Command::OscFreq(fosc) => ([0xB3, ((0xF & fosc) << 4), 0], 2),
        };

        // Send command over the interface
        iface.send_commands(DataFormat::U8(&data[0..len]))
    }
}
