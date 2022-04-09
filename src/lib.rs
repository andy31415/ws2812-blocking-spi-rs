//! # Use ws2812 leds via the `embedded_hal::blocking::spi::Write` trait
//!
//! - For usage with `smart-leds`
//! - Implements the `SmartLedsWrite` trait
//!
//! Needs a type implementing the `blocking::spi::Write` trait.

#![no_std]

use embedded_hal::blocking::spi::Write;
use smart_leds_trait::{SmartLedsWrite, RGB8};
use ws2812_spi_write_constants::ws2812_constants;

ws2812_constants!(WRITE_4_BYTE_CONSTANTS);

/// Wraps a SPI Writer to represent a WS2821 LED array.
///
/// # Examples:
///
/// ```
/// use ws2812_blocking_spi::Ws2812BlockingWriter;
///
/// use embedded_hal::blocking::spi::Write;
/// use smart_leds_trait::{RGB8, SmartLedsWrite};
///
/// //
/// // Displays 3 LEDs: red, green and blue
/// //
/// fn show<SPI: Write<u8>>(spi: SPI) {
///    let mut leds = Ws2812BlockingWriter::new(spi);
///
///    let mut data = [RGB8::default(); 3];
///
///    data[0] = [0xFF_u8, 0_u8, 0_u8].into();  // Full RED
///    data[1] = [0_u8, 0xFF_u8, 0_u8].into();  // Full GREEN
///    data[2] = [0_u8, 0_u8, 0xFF_u8].into();  // Full BLUE
///
///    leds.write(data.iter().cloned());
/// }
///
/// ```
///
pub struct Ws2812BlockingWriter<SPI> {
    spi: SPI,
}

impl<SPI: Write<u8>> Ws2812BlockingWriter<SPI> {
    pub fn new(spi: SPI) -> Self {
        Ws2812BlockingWriter { spi }
    }

    fn flush(&mut self) -> Result<(), SPI::Error> {
        // Should be > 300μs, so for an SPI Freq. of 3.8MHz, we have to send at least 1140 low bits or 140 low bytes
        self.spi.write(&[0u8; 140])
    }
}

impl<SPI: Write<u8>> SmartLedsWrite for Ws2812BlockingWriter<SPI> {
    type Error = SPI::Error;
    type Color = RGB8;

    /// Write all the items of an iterator to a ws2812 strip
    fn write<T, I>(&mut self, iterator: T) -> Result<(), SPI::Error>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>,
    {
        let mut buffer = [0u8; 12];
        for item in iterator {
            let item = item.into();

            buffer[0..4].copy_from_slice(&WRITE_4_BYTE_CONSTANTS[item.g as usize]);
            buffer[4..8].copy_from_slice(&WRITE_4_BYTE_CONSTANTS[item.r as usize]);
            buffer[8..12].copy_from_slice(&WRITE_4_BYTE_CONSTANTS[item.b as usize]);

            self.spi.write(&buffer)?
        }
        self.flush()?;

        Ok(())
    }
}
