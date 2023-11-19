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
        let mut handle_command = |data: &[u8]| {
            // Send command over the interface
            iface.send_commands(DataFormat::U8(data))
        };
        // Some commands are not supported, so the maximum array size needed is only 3 u8 along with the
        // real length
        match self {
            // Set command unlock
            Command::Unlock => handle_command(&[0xFD, 0x12]),

            // Set the bounding box
            Command::SetColumnAddress(a, b) => handle_command(&[0x15, a, b]),
            Command::SetRowAddress(a, b) => handle_command(&[0x75, a, b]),

            // Set the divide and osc freq
            Command::SetDisplayClock(a) => handle_command(&[0xB3, a]),

            // Set the Multiplex ratio
            Command::SetMuxRatio(a) => handle_command(&[0xCA, a]),

            // Shift mapping RAM counter
            Command::SetDisplayOffset(a) => handle_command(&[0xA2, a]),

            // Shift mapping RAM display start line
            Command::SetStartLine(a) => handle_command(&[0xA1, a]),

            // Set horizontal address increment
            Command::SetRemapFormat(a, b) => handle_command(&[0xA0, a, b]),

            // GPIO pins
            Command::SetGPIO(a) => handle_command(&[0xB5, a]),

            // Function selection
            Command::SetFunctionSelection(a) => handle_command(&[0xAB, a]),

            // Set Display Enhancement A
            Command::SetDisplayEnhancementA(a, b) => handle_command(&[0xB4, a, b]),

            // Set Contrast current
            Command::SetContrastCurrent(a) => handle_command(&[0xC1, a]),

            // Set Master current
            Command::SetMasterCurrent(a) => handle_command(&[0xC7, a]),

            // Set linear gray scale table
            Command::SetLinearGrayScaleTable => handle_command(&[0xB9]),

            // Set phase length
            Command::SetPhaseLength(a) => handle_command(&[0xB1, a]),

            // Set Display Enhancement B
            Command::SetDisplayEnhancementB(a, b) => handle_command(&[0xD1, a, b]),

            // Set pre-charge voltage
            Command::SetPrechargeVoltage(a) => handle_command(&[0xBB, a]),

            // Set pre-charge period
            Command::SetPrechargePeriod(a) => handle_command(&[0xB6, a]),

            // Set common pins voltage level
            Command::SetVCOMH(a) => handle_command(&[0xBE, a]),

            // Set normal display mode
            Command::NormalDisplayMode => handle_command(&[0xA6]),

            // Exit partial display
            Command::ExitPartialDisplay => handle_command(&[0xA9]),

            // Write the data following this command
            Command::WriteRAM => handle_command(&[0x5C]),

            // Sleep mode off
            Command::DisplayOn => handle_command(&[0xAF]),

            // Sleep mode on
            Command::DisplayOff => handle_command(&[0xAE]),
        }?;

        Ok(())
    }
}
