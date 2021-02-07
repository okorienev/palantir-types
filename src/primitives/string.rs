#[macro_use]
use std::str::{Utf8Error, from_utf8};

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
/// ```
/// Required imports:
/// ```rust
///   use std::str::{Utf8Error, from_utf8};
///   use deku::prelude::*;
///   use deku::ctx::Size;
///   use palantir_types::primitives::character::{Character};
/// ```
/// represents structure where first byte means array length (and it's max value is 32)
/// and up to 32 next bytes are checked u8 values
macro_rules! sized_string {
    ($struct_name:ident, $max_value: literal) => {
        #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
        pub struct $struct_name {
            #[deku(
                        reader = "sized_u8_read(deku::rest, Size::Bits(8), $max_value)"
                        writer = "sized_u8_write(*size, Size::Bits(8), $max_value, deku::output"
                    )]
            #[deku(update = "self.data.len()")]
            count: u8,
            #[deku(count = "count")]
            data: Vec<Character>,
        }
        // TODO test this impl
        impl $struct_name {
            const MAX: u8 = $max_value;

            #[inline(always)]
            pub fn to_string(&self) -> Result<String, Utf8Error> {
                let mut char_codes: Vec<u8> = Vec::with_capacity(self.count as usize);
                for char_code in &self.data {
                    char_codes.push(**char_code);
                }

                let s = from_utf8(&char_codes)?;
                Ok(s.to_owned())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::primitives::character::Character;
    use deku::ctx::Size;
    use deku::prelude::*;
    use std::str::{from_utf8, Utf8Error};

    sized_string!(SizedString, 10);

    #[test]
    fn test_sized_string() {
        let mut data = vec![3u8, 0x30, 0x30, 0x30];
        let (_rest, mut val) = SizedString::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(
            val,
            SizedString {
                count: 3,
                data: vec![Character(0x30), Character(0x30), Character(0x30)],
            }
        );

        let raw = val.to_bytes().unwrap();
        assert_eq!(data, raw);
    }

    #[test]
    fn test_sized_string_update() {
        let mut data = vec![3u8, 0x30, 0x30, 0x30];
        let (_rest, mut val) = SizedString::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(
            val,
            SizedString {
                count: 3,
                data: vec![Character(0x30), Character(0x30), Character(0x30)]
            }
        );

        val.data.push(Character(0x30));
        assert_eq!(
            val,
            SizedString {
                count: 3,
                data: vec![
                    Character(0x30),
                    Character(0x30),
                    Character(0x30),
                    Character(0x30)
                ]
            }
        );

        val.update().unwrap();
        assert_eq!(
            val,
            SizedString {
                count: 4,
                data: vec![
                    Character(0x30),
                    Character(0x30),
                    Character(0x30),
                    Character(0x30)
                ]
            }
        );
    }
}
