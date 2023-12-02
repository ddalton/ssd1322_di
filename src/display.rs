//! main display module
use core::convert::TryInto;

use crate::command::Command;
use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_graphics::{
    draw_target::DrawTarget, geometry::OriginDimensions, pixelcolor::Gray4, prelude::*, Pixel,
};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

const DISPLAY_WIDTH: usize = 256;
const DISPLAY_HEIGHT: usize = 64;
const BUFFER_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT / 2;

/// Represents the SSD1322 Display.
///
/// Use this struct to initialize the driver.
pub struct Ssd1322<DI> {
    display: DI,
    buffer: [u8; BUFFER_SIZE],
    bounding_box: Option<([u8; 2], [u8; 2])>,
    num_changed: u16,
}

/// Provides an optimized way to capture changes to the framebuffer.
pub trait BoundingBox {
    /// Updates the bounding_box field to the modified area. The bounding_box unit is in bytes.
    fn update_box(&mut self, x: u8, y: u8);
}

impl<DI: WriteOnlyDataCommand> Ssd1322<DI> {
    /// Creates the SSD1322 Display.
    ///
    /// The device needs to be reset before use.
    pub fn new(display: DI) -> Self {
        Self {
            display,
            buffer: [0; BUFFER_SIZE],
            bounding_box: None,
            num_changed: 0,
        }
    }

    /// Resets the display.
    pub fn reset<RST, DELAY>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), DisplayError>
    where
        RST: OutputPin,
        DELAY: DelayMs<u8>,
    {
        rst.set_low().map_err(|_| DisplayError::BusWriteError)?;
        delay.delay_ms(10);

        rst.set_high().map_err(|_| DisplayError::BusWriteError)?;
        delay.delay_ms(200);

        Ok(())
    }

    /// Initializes the display.
    pub fn init(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::Unlock)?;
        self.send_command(Command::DisplayOff)?;
        self.send_command(Command::SetColumnAddress(0x1C, 0x5B))?;
        self.send_command(Command::SetRowAddress(0x00, 0x3F))?;
        self.send_command(Command::SetDisplayClock(0x91))?;
        self.send_command(Command::SetMuxRatio(0x3F))?;
        self.send_command(Command::SetDisplayOffset(0x00))?;
        self.send_command(Command::SetStartLine(0x00))?;
        self.send_command(Command::SetRemapFormat(0x14, 0x11))?;
        self.send_command(Command::SetGPIO(0x00))?;
        self.send_command(Command::SetFunctionSelection(0x01))?;
        self.send_command(Command::SetDisplayEnhancementA(0xA0, 0xFD))?;
        self.send_command(Command::SetContrastCurrent(0xCF))?;
        self.send_command(Command::SetMasterCurrent(0x0F))?;
        self.send_command(Command::SetLinearGrayScaleTable)?;
        self.send_command(Command::SetPhaseLength(0xE2))?;
        self.send_command(Command::SetDisplayEnhancementB(0xA2, 0x20))?;
        self.send_command(Command::SetPrechargeVoltage(0x1F))?;
        self.send_command(Command::SetPrechargePeriod(0x08))?;
        self.send_command(Command::SetVCOMH(0x07))?;
        self.send_command(Command::NormalDisplayMode)?;
        //self.send_command(Command::AllPixelsOn)?;
        self.send_command(Command::DisplayOn)?;

        Ok(())
    }

    /// Allows to send custom commands to the display.
    pub fn send_command(&mut self, command: Command) -> Result<(), DisplayError> {
        command.send(&mut self.display)
    }

    /// Flushes the entire display, and makes the output visible on the screen.
    pub fn flush_all(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::SetColumnAddress(0x1C, 0x5B))?;
        self.send_command(Command::SetRowAddress(0x00, 0x3F))?;
        self.send_command(Command::WriteRAM)?;
        self.display.send_data(U8(&self.buffer))
    }

    /// Flushes only the changed portion of the display.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        if let Some((mut col_addr, row_addr)) = self.bounding_box {
            col_addr[0] -= col_addr[0] % 2;
            col_addr[1] -= col_addr[1] % 2;
            let num_col_bytes: usize = (col_addr[1] - col_addr[0] + 2).into();

            // Convert bytes to column address
            self.send_command(Command::SetColumnAddress(
                col_addr[0] / 2 + 0x1C,
                col_addr[1] / 2 + 0x1C,
            ))?;
            self.send_command(Command::SetRowAddress(row_addr[0], row_addr[1]))?;
            self.send_command(Command::WriteRAM)?;

            for i in row_addr[0]..=row_addr[1] {
                let start_col_byte: usize = col_addr[0] as usize + (i as usize * DISPLAY_WIDTH / 2);
                let end_col_byte: usize = start_col_byte + num_col_bytes;
                self.display
                    .send_data(U8(&self.buffer[start_col_byte..end_col_byte]))?;
            }

            // Reset the bounding_box
            self.bounding_box = None;
            self.num_changed = 0;
        }

        Ok(())
    }
}

impl<DI> BoundingBox for Ssd1322<DI> {
    fn update_box(&mut self, x: u8, y: u8) {
        match self.bounding_box {
            Some((col_addr, row_addr)) => {
                let mut new_col_addr: [u8; 2] = col_addr;
                let mut new_row_addr: [u8; 2] = row_addr;

                // Column address update
                if x / 2 < col_addr[0] {
                    new_col_addr = [x / 2, col_addr[1]];
                } else if x / 2 > col_addr[1] {
                    new_col_addr = [col_addr[0], x / 2];
                }

                // Row address update
                if y < row_addr[0] {
                    new_row_addr = [y, row_addr[1]];
                } else if y > row_addr[1] {
                    new_row_addr = [row_addr[0], y];
                }

                self.bounding_box = Some((new_col_addr, new_row_addr));
            }
            None => self.bounding_box = Some(([x / 2, x / 2], [y, y])),
        }
    }
}

impl<DI> DrawTarget for Ssd1322<DI> {
    type Color = Gray4;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (255,63)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if let (x @ 0..=255, y @ 0..=63) = (coord.x as usize, coord.y as usize) {
                // Calculate the index in the framebuffer.
                let index = (x / 2) + (y * (DISPLAY_WIDTH / 2));
                let new_val: u8 = if x % 2 == 0 {
                    update_upper_nibble(self.buffer[index], color.luma())
                } else {
                    update_lower_nibble(self.buffer[index], color.luma())
                };

                // Update only if changed
                if new_val != self.buffer[index] {
                    self.num_changed += 1;
                    self.update_box(x as u8, y as u8);
                    self.buffer[index] = new_val;
                }
            }
        }

        Ok(())
    }

    fn clear(&mut self, fill: Self::Color) -> Result<(), Self::Error> {
        let luma = fill.luma();
        let byte = (luma << 4) | luma;
        self.buffer.fill(byte);

        Ok(())
    }
}

impl<DI> OriginDimensions for Ssd1322<DI> {
    fn size(&self) -> Size {
        Size::new(
            DISPLAY_WIDTH.try_into().unwrap(),
            DISPLAY_HEIGHT.try_into().unwrap(),
        )
    }
}

#[inline]
fn update_upper_nibble(input: u8, color: u8) -> u8 {
    ((color << 4) & 0xF0) | (input & 0x0F)
}

#[inline]
fn update_lower_nibble(input: u8, color: u8) -> u8 {
    color & 0x0F | (input & 0xF0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use display_interface::DataFormat;
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
        pixelcolor::Gray4,
        text::{Baseline, Text},
    };
    type Result = core::result::Result<(), DisplayError>;

    pub struct TestInterface1 {}

    impl WriteOnlyDataCommand for TestInterface1 {
        fn send_commands(&mut self, _cmds: DataFormat<'_>) -> Result {
            Ok(())
        }

        fn send_data(&mut self, buf: DataFormat<'_>) -> Result {
            match buf {
                U8(_slice) => Ok(()),
                _ => Err(DisplayError::DataFormatNotImplemented),
            }
        }
    }

    #[test]
    /// Tests the character '|'. The framebuffer looks like starting from beginning of row 0
    /// where each '.' represents a pixel.
    /// ......
    /// ..x...
    /// ..x...
    /// ..x...
    /// ..x...
    /// ..x...
    /// ..x...
    /// ..x...
    ///
    fn single_char_one_col() {
        let s = TestInterface1 {};
        let mut disp = Ssd1322::new(s);

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Gray4::new(0b0000_1111))
            .build();

        Text::with_baseline("|", Point::new(0, 0), text_style, Baseline::Top)
            .draw(&mut disp)
            .unwrap();

        assert_eq!(disp.bounding_box.unwrap().0[0], 1);
        assert_eq!(disp.bounding_box.unwrap().0[1], 1);
        assert_eq!(disp.bounding_box.unwrap().1[0], 1);
        assert_eq!(disp.bounding_box.unwrap().1[1], 7);
        assert_eq!(disp.num_changed, 7);

        for i in 1..8 {
            let start = i * 128;
            assert_eq!(&disp.buffer[start..start + 3], [0, 0xf0, 0]);
        }

        let _ = disp.flush_changed();
    }

    #[test]
    /// Tests the character 'A'. The framebuffer looks like starting from beginning of row 0
    /// where each '.' represents a pixel.
    /// ......
    /// ..x...
    /// .x.x..
    /// x...x.
    /// x...x.
    /// xxxxx.
    /// x...x.
    /// x...x.
    ///
    fn single_char_multi_col() {
        let s = TestInterface1 {};
        let mut disp = Ssd1322::new(s);

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Gray4::new(0b0000_1111))
            .build();

        Text::with_baseline("A", Point::new(0, 0), text_style, Baseline::Top)
            .draw(&mut disp)
            .unwrap();

        assert_eq!(disp.bounding_box.unwrap().0[0], 0);
        assert_eq!(disp.bounding_box.unwrap().0[1], 2);
        assert_eq!(disp.bounding_box.unwrap().1[0], 1);
        assert_eq!(disp.bounding_box.unwrap().1[1], 7);
        assert_eq!(disp.num_changed, 16);

        let _ = disp.flush_changed();
    }

    #[test]
    /// Tests the character 'A' at an offset.
    /// .......
    /// .......
    /// .......
    /// .......
    /// .......
    /// .......
    /// ...x...
    /// ..x.x..
    /// .x...x.
    /// .x...x.
    /// .xxxxx.
    /// .x...x.
    /// .x...x.
    ///
    fn single_char_offset() {
        let s = TestInterface1 {};
        let mut disp = Ssd1322::new(s);

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Gray4::new(0b0000_1111))
            .build();

        Text::with_baseline("A", Point::new(1, 5), text_style, Baseline::Top)
            .draw(&mut disp)
            .unwrap();

        assert_eq!(disp.bounding_box.unwrap().0[0], 0);
        assert_eq!(disp.bounding_box.unwrap().0[1], 2);
        assert_eq!(disp.bounding_box.unwrap().1[0], 6);
        assert_eq!(disp.bounding_box.unwrap().1[1], 12);
        assert_eq!(disp.num_changed, 16);

        let _ = disp.flush_changed();
    }

    #[test]
    /// Tests the character 'A' clipped at the right.
    /// .......
    /// ....... x
    /// .......x x
    /// ......x   x
    /// ......x   x
    /// ......xxxxx
    /// ......x   x
    /// ......x   x
    ///
    fn single_char_clipped() {
        let s = TestInterface1 {};
        let mut disp = Ssd1322::new(s);

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Gray4::new(0b0000_1111))
            .build();

        Text::with_baseline("A", Point::new(255, 0), text_style, Baseline::Top)
            .draw(&mut disp)
            .unwrap();

        assert_eq!(disp.bounding_box.unwrap().0[0], 127);
        assert_eq!(disp.bounding_box.unwrap().0[1], 127);
        assert_eq!(disp.bounding_box.unwrap().1[0], 3);
        assert_eq!(disp.bounding_box.unwrap().1[1], 7);
        assert_eq!(disp.num_changed, 5);

        let _ = disp.flush_changed();
    }
}
