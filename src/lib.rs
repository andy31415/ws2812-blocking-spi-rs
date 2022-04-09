#![no_std]

use embedded_hal::blocking::spi::Write;
use smart_leds_trait::{SmartLedsWrite, RGB8};
use ws2812_spi_write_constants::ws2812_constants;


ws2812_constants!(WRITE_3_BYTE_CONSTANTS);

pub struct SpiWrapper<SPI> {
    spi: SPI,
}

impl<SPI: Write<u8>> SpiWrapper<SPI> {
    pub fn new(spi: SPI) -> Self {
        SpiWrapper { spi }
    }

    fn flush(&mut self) -> Result<(), SPI::Error> {
        // Should be > 300μs, so for an SPI Freq. of 3.8MHz, we have to send at least 1140 low bits or 140 low bytes
        self.spi.write(&[0u8; 140])
    }
}

impl<SPI: Write<u8>> SmartLedsWrite for SpiWrapper<SPI> {
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

            buffer[0..4].copy_from_slice(&WRITE_3_BYTE_CONSTANTS[item.g as usize]);
            buffer[4..8].copy_from_slice(&WRITE_3_BYTE_CONSTANTS[item.r as usize]);
            buffer[8..12].copy_from_slice(&WRITE_3_BYTE_CONSTANTS[item.b as usize]);

            self.spi.write(&buffer)?
        }
        self.flush()?;

        Ok(())
    }
}





