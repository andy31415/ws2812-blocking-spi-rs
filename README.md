## WS2812 driver for embedded-hal blocking spi traits

This crate adds an implementation using `embedded_hal::spi::blocking`
for [smart-leds](https://github.com/smart-leds-rs/smart-leds)

The ws2821 crate that is part of the smart-leds project is using a 
`FullDuplex` implementation of SPI which is not implemented on
all platforms.

This crate pre-generates constants for all 8-bit patterns required to
send to LEDs and this adds a 1KB flash space overhead.

## Usage example


TODO