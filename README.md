# ssd1322_di
A driver for the ssd1322 OLED display chip that supports display_inteface. This is a work in progress.

This driver is successfully tested against the Newhaven Display NHD-3.12-25664UCB2 using a TM4C123x evaluation board.
A project containing this example can be found here: https://github.com/ddalton/frecount/tree/main/spi_display

![Screenshot with text output](embedded_examples/IMG_2456.JPG?raw=true "Screenshot with text output")

It has 2 flush methods. The ``flush_all`` method flushes the entire screen. This is needed only if the entire contents of the screen needs to be flushed to the display and should be rarely used since it is an expensive call. Prefer the ``flush`` method which sends only the changed pixels from the last flush call.

# Credits
Inspired by ssd1322 and ssd1327 drivers.
