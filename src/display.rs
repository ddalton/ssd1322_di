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
}

impl<DI: WriteOnlyDataCommand> Ssd1322<DI> {
    /// Creates the SSD1322 Display.
    ///
    /// The device needs to be reset before use.
    pub fn new(display: DI) -> Self {
        Self {
            display,
            buffer: [0; BUFFER_SIZE],
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
        delay.delay_ms(10);

        Ok(())
    }

    /// Initializes the display.
    pub fn init(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOff)?;
        self.send_command(Command::OscFreq(0xF))?;
        self.send_command(Command::DisplayOn)?;

        Ok(())
    }

    /// Allows to send custom commands to the display.
    pub fn send_command(&mut self, command: Command) -> Result<(), DisplayError> {
        command.send(&mut self.display)
    }

    /// Flushes the display, and makes the output visible on the screen.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::WriteRAM)?;
        self.display.send_data(U8(&self.buffer))
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
            if let (x @ 0..=255, y @ 0..=63) = (coord.x as u8, coord.y as u8) {
                // Calculate the index in the framebuffer.
                let index: u8 = x / 2 + y * 128;
                if x % 2 == 0 {
                    self.buffer[index as usize] =
                        update_upper_half(self.buffer[index as usize], color.luma());
                } else {
                    self.buffer[index as usize] =
                        update_lower_half(self.buffer[index as usize], color.luma());
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
fn update_upper_half(input: u8, color: u8) -> u8 {
    color << 4 | (input & 0x0F)
}

#[inline]
fn update_lower_half(input: u8, color: u8) -> u8 {
    color & 0x0f | (input & 0xF0)
}
