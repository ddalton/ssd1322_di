//! Buffered display module for use with the [embedded_graphics] crate
//!
//! ```rust,ignore
//! let interface = /* your preferred `display-interface` implementor */;
//! let display: GraphicsMode<_> = Builder::new().connect(interface).into();
//! let image = include_bytes!("image_16x16.raw");
//!
//! display.init().unwrap();
//! display.flush().unwrap();
//! display.draw(Line::new(Coord::new(0, 0), (16, 16), 1.into()).into_iter());
//! display.draw(Rect::new(Coord::new(24, 0), (40, 16), 1u8.into()).into_iter());
//! display.draw(Circle::new(Coord::new(64, 8), 8, 1u8.into()).into_iter());
//! display.draw(Image1BPP::new(image, 0, 24));
//! display.draw(Font6x8::render_str("Hello Rust!", 1u8.into()).translate(Coord::new(24, 24)).into_iter());
//! display.flush().unwrap();
//! ```

use display_interface::{DisplayError, WriteOnlyDataCommand};
use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::properties::DisplayProperties;

const BUFFER_SIZE: usize = 128 * 64 / 8;

/// Graphics mode handler
pub struct GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    properties: DisplayProperties<DI>,
    buffer: [u8; BUFFER_SIZE],
}

impl<DI> GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Create new GraphicsMode instance
    pub fn new(properties: DisplayProperties<DI>) -> Self {
        GraphicsMode {
            properties,
            buffer: [0; BUFFER_SIZE],
        }
    }

    /// Release all resources used by GraphicsMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI> GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Clear the display buffer. You need to call `disp.flush()` for any effect on the screen
    pub fn clear(&mut self) {
        self.buffer = [0; BUFFER_SIZE];
    }

    /// Reset display. This is very important on the SSD1309!
    ///
    /// This should be called before `init` or any other methods.
    pub fn reset<RST, DELAY, PinE>(&mut self, rst: &mut RST, delay: &mut DELAY) -> Result<(), PinE>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        rst.set_high()?;
        delay.delay_ms(10);
        rst.set_low()?;
        delay.delay_ms(10);
        rst.set_high()?;
        delay.delay_ms(10);
        Ok(())
    }

    /// Write out data to display
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        let display_size = self.properties.get_size();

        // Ensure the display buffer is at the origin of the display before we send the full frame
        // to prevent accidental offsets
        let (display_width, display_height) = display_size.dimensions();
        let column_offset = display_size.column_offset();
        self.properties.set_draw_area(
            (column_offset, 0),
            (display_width + column_offset, display_height),
        )?;

        let length = (display_width as usize) * (display_height as usize) / 8;

        self.properties.draw(&self.buffer[..length])
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    ///
    /// TODO: Need to map bit to nibble
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let (display_width, _) = self.properties.get_size().dimensions();
    }

    /// Get display dimensions, taking into account the current rotation of the display
    pub fn get_dimensions(&self) -> (u8, u8) {
        self.properties.get_dimensions()
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

#[cfg(feature = "graphics")]
impl<DI> DrawTarget for GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    type Color = BinaryColor;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(pos, _color)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                self.set_pixel(pos.x as u32, pos.y as u32, color.is_on().into())
            });

        Ok(())
    }
}

#[cfg(feature = "graphics")]
impl<DI> OriginDimensions for GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    fn size(&self) -> Size {
        let (w, h) = self.get_dimensions();

        Size::new(w.into(), h.into())
    }
}
