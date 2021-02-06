#[macro_use]

use std::vec::Vec;

use deku::prelude::*;
use deku::ctx::Size;
use super::character::{ Character };

fn sized_u8_read(
    rest: &BitSlice<Msb0, u8>,
    bit_size: Size,
    max_value: u8,
) -> Result<(&BitSlice<Msb0, u8>, u8), DekuError> {

    let (rest, value) = u8::read(rest, bit_size)?;

    if value > max_value {
        return Err(DekuError::InvalidParam(format!(
            "Value {} is greater than max allowed {}", value, max_value
        )))
    }

    Ok((rest, value))
}

fn sized_u8_write(
    value: u8,
    bit_size: Size,
    max_value: u8,
    output: &mut BitVec<Msb0, u8>,
) -> Result<(), DekuError> {
    if value > max_value {
        return Err(DekuError::InvalidParam(format!(
            "Value {} is greater than max allowed {}", value, max_value
        )))
    }

    value.write(output, bit_size)
}

/// Purpose of this macro is to keep protocol byte-aligned to simplify
/// non-rust (or, to be honest, non-deku implementations) and improve human readability
/// but to still have ability limiting length of byte arrays which represent strings
///
/// ```rust
///   sized_string!(32, "tag_length", "tag_name")
/// ```
/// represents structure where first byte means array length (and it's max value is 32)
/// and up to 32 next bytes are checked u8 values
macro_rules! sized_string {
    (
        $max_value:literal,
        $size_field_name:literal,
        $string_field_name:literal,
    ) => {
        #[deku(
            reader = "sized_u8_read(deku::rest, Size::Bits(8), $max_value)",
            writer = "sized_u8_write(*$size_field_name, Size::Bits(8), $max_value, deku::output"
            update = "self.$string_field_name.len()"
        )]
        $size_field_name: u8,
        #[deku(count = "$size_field_name")]
        $string_field_name: Vec<Character>,
    };
}