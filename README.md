## WS2812 driver for embedded-hal blocking spi traits

This crate adds an implementation using `embedded_hal::spi::blocking`
for [smart-leds](https://github.com/smart-leds-rs/smart-leds)

The ws2821 crate that is part of the smart-leds project is using a 
`FullDuplex` implementation of SPI which is not implemented on
all platforms.

This crate pre-generates constants for all 8-bit patterns required to
send to LEDs and this adds a 1KB flash space overhead.

## Usage example

```rust
use ws2812_blocking_spi::Ws2812BlockingWriter;

// Requires a SPI interface. LEDs data pin should be
// connected to the MOSI pin (master-in-slave-out)
let spi: embedded_hal::blocking::spi::Write<u8> = /*... */;

// setup some data to write
let mut data = [RGB8::default(); 3];
data[0] = [0xFF_u8, 0_u8, 0_u8].into();  // Full RED
data[1] = [0_u8, 0xFF_u8, 0_u8].into();  // Full GREEN
data[2] = [0_u8, 0_u8, 0xFF_u8].into();  // Full BLUE

// Create a writer
let mut leds = Ws2812BlockingWriter::new(spi);

// does the data write
leds.write(data.iter().cloned());
```
