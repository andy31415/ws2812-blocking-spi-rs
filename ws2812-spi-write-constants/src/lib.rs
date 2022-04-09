//! Contains macros for constant generation
//!
//! WS2812 SPI data uses SPI writes to define high/low patterns
//! to sent to the neopixel chip to be interpreted as values
//!
//! This crate pre-computes the specific patterns requires.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::vec;
use syn::{parse_macro_input, Ident};

/// Returns the SPI 4-bit pattern required to be sent so that
/// WS2812 interprets the given bit as either high or low.
///
/// Only the lowest 4 bits of the return value are relevant. The
/// value also assumes there will be a followup 0 written to the SPI since
/// the logic assumes that the MOSI (Master-In-Slave-Out) goes back to 0
/// in an idle state.
fn bit_4_u32(v: bool) -> u32 {
    // awkward pattern to FORCE highest bit to always be 0 because
    // STM32 will have MOSI idle state equal to MSB of one of the bytes:
    //
    // https://michaeltien8901.github.io/stm32/2019/01/06/STM32F072-MOSI-Idle-State.html
    if v {
        0b0111_u32
    } else {
        0b0100_u32
    }
}

/// Given a 4-bit nibble value, return the full 4 bytes
/// that are to be written for WS2812 to recognize this value.
fn nibble_16_u32(nibble: u8) -> u32 {
    (bit_4_u32((nibble & 0b1000) != 0) << 12)
        | (bit_4_u32((nibble & 0b0100) != 0) << 8)
        | (bit_4_u32((nibble & 0b0010) != 0) << 4)
        | (bit_4_u32((nibble & 0b0001) != 0))
}

/// Proc macro that generates a constant with the given name
/// that contains all 256 combinations of patters to send a byte to WS2812
///
/// #Examples:
///
/// ```
///    // This creates a `const WRITE_4_BYTE_CONSTANTS: [[u8;4]; 256]
///    // Note that this requires 1K flash
///    use ws2812_spi_write_constants::ws2812_constants;
///
///    ws2812_constants!(WRITE_4_BYTE_CONSTANTS);
///
///    assert_eq!(
///        WRITE_4_BYTE_CONSTANTS[0b0000_0000],
///        [0b0100_0100, 0b0100_0100, 0b0100_0100, 0b0100_0100]
///    );
///
///    assert_eq!(
///        WRITE_4_BYTE_CONSTANTS[0b0100_1100],
///        [0b0100_0111, 0b0100_0100, 0b0111_0111, 0b0100_0100]
///    );
///    
/// ```
#[proc_macro]
pub fn ws2812_constants(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);

    let mut values = vec![];

    for v in 0..=0xFFu8 {
        let data = (nibble_16_u32((v >> 4) & 0x0F) << 16) | nibble_16_u32(v & 0x0F);

        values.push([
            ((data >> 24) & 0xFF) as u8,
            ((data >> 16) & 0xFF) as u8,
            ((data >> 8) & 0xFF) as u8,
            (data & 0xFF) as u8,
        ]);
    }

    let values = values.iter().map(|[a, b, c, d]| {
        quote! {
            [#a, #b, #c, #d ]
        }
    });

    quote! {
        const #ident : [[u8;4]; 256] = [
          #(#values), *
        ];
    }
    .into()
}
