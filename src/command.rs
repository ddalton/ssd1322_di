//! ssd1322 Commands
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

/// Commands - subset of the supported commands
#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    /// Set command unlock
    Unlock,
    /// Set the column address
    SetColumnAddress(u8, u8),
    /// Set the row address
    SetRowAddress(u8, u8),
    /// Set the divide and osc freq
    SetDisplayClock(u8),
    /// Set the multiplex ratio
    SetMuxRatio(u8),
    /// Shift mapping RAM counter
    SetDisplayOffset(u8),
    /// Shift mapping RAM display start line
    SetStartLine(u8),
    /// Set horizontal address increment
    SetRemapFormat(u8, u8),
    /// GPIO pins
    SetGPIO(u8),
    /// Function selection
    SetFunctionSelection(u8),
    /// Set Display Enhancement A
    SetDisplayEnhancementA(u8, u8),
    /// Set Contrast current
    SetContrastCurrent(u8),
    /// Set Master current
    SetMasterCurrent(u8),
    /// Set linear gray scale table
    SetLinearGrayScaleTable,
    /// Set phase length
    SetPhaseLength(u8),
    /// Set Display Enhancement B
    SetDisplayEnhancementB(u8, u8),
    /// Set pre-charge voltage
    SetPrechargeVoltage(u8),
    /// Set pre-charge period
    SetPrechargePeriod(u8),
    /// Set common pins voltage level
    SetVCOMH(u8),
    /// Set normal display mode
    NormalDisplayMode,
    /// Set all pixels on
    AllPixelsOn,
    /// Set all pixels off
    AllPixelsOff,
    /// Exit partial display
    ExitPartialDisplay,
    /// Write the data following this command
    WriteRAM,
    /// Sleep mode off
    DisplayOn,
    /// Sleep mode on
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
            let _ = iface.send_commands(DataFormat::U8(&data[0..1]));

            // If the command has any data portion then send that also
            if data.len() > 1 {
                let _ = iface.send_data(DataFormat::U8(&data[1..data.len()]));
            }
        };

        match self {
            Command::Unlock => handle_command(&[0xFD, 0x12]),
            Command::SetColumnAddress(a, b) => handle_command(&[0x15, a, b]),
            Command::SetRowAddress(a, b) => handle_command(&[0x75, a, b]),
            Command::SetDisplayClock(a) => handle_command(&[0xB3, a]),
            Command::SetMuxRatio(a) => handle_command(&[0xCA, a]),
            Command::SetDisplayOffset(a) => handle_command(&[0xA2, a]),
            Command::SetStartLine(a) => handle_command(&[0xA1, a]),
            Command::SetRemapFormat(a, b) => handle_command(&[0xA0, a, b]),
            Command::SetGPIO(a) => handle_command(&[0xB5, a]),
            Command::SetFunctionSelection(a) => handle_command(&[0xAB, a]),
            Command::SetDisplayEnhancementA(a, b) => handle_command(&[0xB4, a, b]),
            Command::SetContrastCurrent(a) => handle_command(&[0xC1, a]),
            Command::SetMasterCurrent(a) => handle_command(&[0xC7, a]),
            Command::SetLinearGrayScaleTable => handle_command(&[0xB9]),
            Command::SetPhaseLength(a) => handle_command(&[0xB1, a]),
            Command::SetDisplayEnhancementB(a, b) => handle_command(&[0xD1, a, b]),
            Command::SetPrechargeVoltage(a) => handle_command(&[0xBB, a]),
            Command::SetPrechargePeriod(a) => handle_command(&[0xB6, a]),
            Command::SetVCOMH(a) => handle_command(&[0xBE, a]),
            Command::NormalDisplayMode => handle_command(&[0xA6]),
            Command::AllPixelsOff => handle_command(&[0xA4]),
            Command::AllPixelsOn => handle_command(&[0xA5]),
            Command::ExitPartialDisplay => handle_command(&[0xA9]),
            Command::WriteRAM => handle_command(&[0x5C]),
            Command::DisplayOn => handle_command(&[0xAF]),
            Command::DisplayOff => handle_command(&[0xAE]),
        };

        Ok(())
    }
}
