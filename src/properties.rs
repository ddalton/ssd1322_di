//! Container to store and set display properties

use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

use crate::{command::Command, displaysize::DisplaySize};

/// Display properties struct
pub struct DisplayProperties<DI> {
    iface: DI,
    display_size: DisplaySize,
    draw_area_start: (u8, u8),
    draw_area_end: (u8, u8),
}

impl<DI> DisplayProperties<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Create new DisplayProperties instance
    pub fn new(iface: DI, display_size: DisplaySize) -> DisplayProperties<DI> {
        DisplayProperties {
            iface,
            display_size,
            draw_area_start: (0, 0),
            draw_area_end: (0, 0),
        }
    }

    /// Set the position in the framebuffer of the display where any sent data should be
    /// drawn. This method can be used for changing the affected area on the screen as well
    /// as (re-)setting the start point of the next `draw` call.
    pub fn set_draw_area(&mut self, start: (u8, u8), end: (u8, u8)) -> Result<(), DisplayError> {
        self.draw_area_start = start;
        self.draw_area_end = end;

        self.send_draw_address()
    }

    /// Send the data to the display for drawing at the current position in the framebuffer
    /// and advance the position accordingly. Cf. `set_draw_area` to modify the affected area by
    /// this method.
    pub fn draw(&mut self, mut buffer: &[u8]) -> Result<(), DisplayError> {
        self.iface.send_data(DataFormat::U8(&buffer[..]))?;

        Ok(())
    }

    fn send_draw_address(&mut self) -> Result<(), DisplayError> {
        Command::SetColumnAddress(self.draw_area_start.0, self.draw_area_end.0)
            .send(&mut self.iface)?;
        Command::SetRowAddress(self.draw_area_start.1, self.draw_area_end.1).send(&mut self.iface)
    }

    /// Get the configured display size
    pub fn get_size(&self) -> DisplaySize {
        self.display_size
    }

    /// Get display dimensions, taking into account the current rotation of the display
    ///
    /// ```rust
    /// # struct FakeInterface;
    /// #
    /// # impl DisplayInterface for FakeInterface {
    /// #     fn send_command(&mut self, cmd: u8) -> Result<(), ()> { Ok(()) }
    /// #     fn send_data(&mut self, buf: &[u8]) -> Result<(), ()> { Ok(()) }
    /// # }
    /// #
    /// # let interface = FakeInterface {};
    /// #
    /// let disp = DisplayProperties::new(
    ///     interface,
    ///     DisplaySize::Display128x64,
    /// );
    /// assert_eq!(disp.get_dimensions(), (128, 64));
    ///
    /// # let interface = FakeInterface {};
    /// let rotated_disp = DisplayProperties::new(
    ///     interface,
    ///     DisplaySize::Display128x64,
    /// );
    /// assert_eq!(rotated_disp.get_dimensions(), (64, 128));
    /// ```
    pub fn get_dimensions(&self) -> (u8, u8) {
        self.display_size.dimensions()
    }
}
