# mono-ssd1322
A monochrome driver for the ssd1322 OLED display chip.

This driver is designed for situations where the frame buffer size needs to be minimized. Because of this reason only monochrome display mode is supported.
Without this restriction the frame buffer size increases to 4x with 4 bits representing the color.

The other benefit with having a frame buffer is the ability to optimize the redraw region. The bounding box surrounding the changed pixels is redrawn from the previous flush call. 
If only a small part of the display is being redrawn, this can optimize the time spent in updating the display.

This can be seen in the demo - stop watch. The left button is reset and right button is start/stop. A counter is incremented by a timer and this counter value is displayed. If the display drawing is slow then the stop watch is not smooth.

# High level design
The following steps are the high level design:
1. Use a frame buffer supporting monochrome display.
2. A bounding box initially None is set based on the setPixel calls.
   The bounding box is updated (increases in size) only if the value changes from the existing value in the frame buffer.
3. If a bounding box is initialized, it's dimensions are sent to the display using the set column address and set row address commands.
4. Flush will send the data to the device. After the data is sent, the bounding box is reset.
   The boundingbox needs to implement an iterator to feed the send method.
   The next call with match against the bit pattern to product the corresponding nibble pattern.
5. Create Mock DrawTarget to check the changes to the frame buffer by API.
6. Create a parallel interface driver implementation. This implementation will bypass bit-banding.
7. Test with both SPI and parallel interfaces.

