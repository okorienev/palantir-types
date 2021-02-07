#[macro_use]
use std::vec::Vec;

use super::character::Character;
use deku::ctx::Size;
use deku::prelude::*;

fn sized_u8_read(
    rest: &BitSlice<Msb0, u8>,
    bit_size: Size,
    max_value: u8,
) -> Result<(&BitSlice<Msb0, u8>, u8), DekuError> {
    let (rest, value) = u8::read(rest, bit_size)?;

    if value > max_value {
        return Err(DekuError::InvalidParam(format!(
            "Value {} is greater than max allowed {}",
            value, max_value
        )));
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
            "Value {} is greater than max allowed {}",
            value, max_value
        )));
    }

    value.write(output, bit_size)
}

/// Purpose of this macro is to keep protocol byte-aligned to simplify
/// non-rust (or, to be honest, non-deku implementations) and improve human readability
/// but to still have ability limiting length of byte arrays which represent strings
///
/// ```rust
///   sized_string!(SizedString, 32);
///   // or
///   sized_string!(SizedString, 32, tag_length, tag_name);
/// ```
/// represents structure where first byte means array length (and it's max value is 32)
/// and up to 32 next bytes are checked u8 values
macro_rules! sized_string {
    ($struct_name:ident, $max_value: literal) => {
        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
        pub struct $struct_name {
            #[deku(
                        reader = "sized_u8_read(deku::rest, Size::Bits(8), $max_value)",
                        writer = "sized_u8_write(*size, Size::Bits(8), $max_value, deku::output"
                        update = "self.data.len()"
                    )]
            count: u8,
            #[deku(count = "count")]
            data: Vec<Character>,
        }
    }; /*    (
           $struct_name:ident,
           $max_value:literal,
           $size_field_name:ident,
           $string_field_name:ident,
       ) => {
               #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
               pub struct $struct_name {
               #[deku(
                   reader = "sized_u8_read(deku::rest, Size::Bits(8), $max_value)",
                   writer = "sized_u8_write(*$size_field_name, Size::Bits(8), $max_value, deku::output"
                   update = "self.$string_field_name.len()"
               )]
               $size_field_name: u8,
               #[deku(count = "$size_field_name")]
               $string_field_name: Vec<Character>,
           }
       };*/
}

#[cfg(test)]
mod tests {
    use super::super::character::Character;
    use deku::prelude::*;

    sized_string!(SizedString, 10);

    #[test]
    fn test_sized_string() {
        let s = SizedString {
            count: 3,
            data: vec![Character(0x30), Character(0x30), Character(0x30)],
        };
    }
}
