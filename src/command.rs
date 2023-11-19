use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

/// ssd1322 Commands

/// Commands - subset of the supported commands
#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    Unlock,
    SetColumnAddress(u8, u8),
    SetRowAddress(u8, u8),
    SetDisplayClock(u8),
    SetMuxRatio(u8),
    SetDisplayOffset(u8),
    SetStartLine(u8),
    SetRemapFormat(u8, u8),
    SetGPIO(u8),
    SetFunctionSelection(u8),
    SetDisplayEnhancementA(u8, u8),
    SetContrastCurrent(u8),
    SetMasterCurrent(u8),
    SetLinearGrayScaleTable,
    SetPhaseLength(u8),
    SetDisplayEnhancementB(u8, u8),
    SetPrechargeVoltage(u8),
    SetPrechargePeriod(u8),
    SetVCOMH(u8),
    NormalDisplayMode,
    ExitPartialDisplay,
    WriteRAM,
    DisplayOn,
    DisplayOff,
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
            // Set command unlock
            Command::Unlock => ([0xFD, 0x12, 0], 2),

            // Set the bounding box
            Command::SetColumnAddress(a, b) => ([0x15, a, b], 3),
            Command::SetRowAddress(a, b) => ([0x75, a, b], 3),

            // Set the divide and osc freq
            Command::SetDisplayClock(a) => ([0xB3, a, 0], 2),

            // Set the Multiplex ratio
            Command::SetMuxRatio(a) => ([0xCA, a, 0], 2),

            // Shift mapping RAM counter
            Command::SetDisplayOffset(a) => ([0xA2, a, 0], 2),

            // Shift mapping RAM display start line
            Command::SetStartLine(a) => ([0xA1, a, 0], 2),

            // Set horizontal address increment
            Command::SetRemapFormat(a, b) => ([0xA0, a, b], 3),

            // GPIO pins
            Command::SetGPIO(a) => ([0xB5, a, 0], 2),

            // Function selection
            Command::SetFunctionSelection(a) => ([0xAB, a, 0], 2),

            // Set Display Enhancement A
            Command::SetDisplayEnhancementA(a, b) => ([0xB4, a, b], 3),

            // Set Contrast current
            Command::SetContrastCurrent(a) => ([0xC1, a, 0], 2),

            // Set Master current
            Command::SetMasterCurrent(a) => ([0xC7, a, 0], 2),

            // Set linear gray scale table
            Command::SetLinearGrayScaleTable => ([0xB9, 0, 0], 1),

            // Set phase length
            Command::SetPhaseLength(a) => ([0xB1, a, 0], 2),

            // Set Display Enhancement B
            Command::SetDisplayEnhancementB(a, b) => ([0xD1, a, b], 3),

            // Set pre-charge voltage
            Command::SetPrechargeVoltage(a) => ([0xBB, a, 0], 2),

            // Set pre-charge period
            Command::SetPrechargePeriod(a) => ([0xB6, a, 0], 2),

            // Set common pins voltage level
            Command::SetVCOMH(a) => ([0xBE, a, 0], 2),

            // Set normal display mode
            Command::NormalDisplayMode => ([0xA6, 0, 0], 1),

            // Exit partial display
            Command::ExitPartialDisplay => ([0xA9, 0, 0], 1),

            // Write the data following this command
            Command::WriteRAM => ([0x5C, 0, 0], 1),

            // Sleep mode off
            Command::DisplayOn => ([0xAF, 0, 0], 1),

            // Sleep mode on
            Command::DisplayOff => ([0xAE, 0, 0], 1),
        };

        // Send command over the interface
        iface.send_commands(DataFormat::U8(&data[0..len]))
    }
}
