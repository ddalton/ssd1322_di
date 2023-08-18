# ssd1322_di
A driver for the ssd1322 OLED display chip. This is a work in progress.

The driver utilizes a frame buffer to optimize the redraw region. The bounding box surrounding the changed pixels is redrawn from the previous flush call. 
If only a small part of the display is being redrawn, this can optimize the time spent in updating the display.

# High level design
The following steps are the high level design:
1. Use a frame buffer supporting 4-bit grayscale display.
2. A bounding box initially None is set based on the setPixel calls.
   The bounding box is updated (increases in size) only if the value changes from the existing value in the frame buffer.
3. If a bounding box is initialized, it's dimensions are sent to the display using the set column address and set row address commands.
4. Flush will send the data to the device. After the data is sent, the bounding box is reset.
   The boundingbox needs to implement an iterator to feed the send method.
   The next call with match against the bit pattern to product the corresponding nibble pattern.
5. Create Mock DrawTarget to check the changes to the frame buffer by API.
6. Create a parallel interface driver implementation. This implementation will bypass bit-banding.
7. Test with both SPI and parallel interfaces.


Modularized design
==================
0. Draw a simple rectange and check all the communication interfaces, including parallel interface.
   Test with bytes less than bounding box size
   and with bytes more than bounding box size
   Test with different clock speeds
1. Write the bounding box logic and iterator
2. Write the byte to nibble converter

# Credits
This driver is heavily inspired by the ssd1327 driver.
