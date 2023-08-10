//! Interface factory
//!
//! This is the easiest way to create a driver instance. You can set various parameters of the
//! driver and give it an interface to use. The builder will return a
//! [`mode::RawMode`](../mode/raw/struct.RawMode.html) object which you should coerce to a richer
//! display mode, like [mode::Graphics](../mode/graphics/struct.GraphicsMode.html) for drawing
//! primitives and text.
//!
//! # Examples
//!
//! Connect over SPI with default rotation (0 deg) and size (128x64):
//!
//! ```rust,ignore
//! use display_interface_spi::SPIInterface;
//!
//! let spi = /* SPI interface from your HAL of choice */;
//! let dc = /* GPIO data/command select pin */;
//!
//! let spi_interface = SPIInterfaceNoCS::new(spi, dc);
//!
//! Builder::new().connect(spi_interface);
//! ```
//!
//! The above examples will produce a [RawMode](../mode/raw/struct.RawMode.html) instance
//! by default. You need to coerce them into a mode by specifying a type on assignment. For
//! example, to use [`GraphicsMode` mode](../mode/graphics/struct.GraphicsMode.html):
//!
//! ```rust,ignore
//! let display: GraphicsMode<_> = Builder::new().connect(interface).into();
//! ```

use crate::{displaysize::DisplaySize, graphics::GraphicsMode, properties::DisplayProperties};

/// Builder struct. Driver options and interface are set using its methods.
#[derive(Clone, Copy)]
pub struct Builder {
    display_size: DisplaySize,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Create new builder with a default size of 128 x 64 pixels and no rotation.
    pub fn new() -> Builder {
        Builder {
            display_size: DisplaySize::Display128x64,
        }
    }
}

impl Builder {
    /// Set the size of the display. Supported sizes are defined by [DisplaySize].
    pub fn with_size(self, display_size: DisplaySize) -> Self {
        Self {
            display_size,
            ..self
        }
    }

    /// Finish the builder and use the given interface to communicate with the display.
    pub fn connect<DI>(self, interface: DI) -> GraphicsMode<DI>
    where
        DI: display_interface::WriteOnlyDataCommand,
    {
        let properties = DisplayProperties::new(interface, self.display_size);
        GraphicsMode::<DI>::new(properties)
    }
}
