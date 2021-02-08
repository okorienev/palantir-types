use crate::primitives::character::Character;
use std::convert::{TryFrom, TryInto};
use std::string::FromUtf8Error;

use crate::primitives::character::error::CharacterError;
use deku::prelude::*;

macro_rules! try_into_string {
    ($name: ident) => {
        impl TryInto<String> for $name {
            type Error = FromUtf8Error;

            fn try_into(self) -> Result<String, Self::Error> {
                let mut char_codes: Vec<u8> = Vec::with_capacity(self.data.len() as usize);

                for char_code in &self.data {
                    char_codes.push(**char_code)
                }

                Ok(String::from_utf8(char_codes)?)
            }
        }
    };
}

macro_rules! try_from_u8_vec {
    ($name: ident) => {
        impl TryFrom<Vec<u8>> for $name {
            type Error = CharacterError;

            fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
                let mut data: Vec<Character> = Vec::with_capacity(value.len());
                for char_code in &value {
                    data.push(Character::try_from(*char_code)?);
                }
                Ok(Self {
                    count: value.len() as u8,
                    data,
                })
            }
        }
    };
}

macro_rules! string_impl {
    ($name: ident) => {
        try_into_string!($name);
        try_from_u8_vec!($name);
    };
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct String8 {
    #[deku(pad_bits_before = "5", bits = "3", update = "self.data.len()")]
    pub count: u8,
    #[deku(count = "count")]
    pub data: Vec<Character>,
}
string_impl!(String8);

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct String16 {
    #[deku(pad_bits_before = "4", bits = "4", update = "self.data.len()")]
    pub count: u8,
    #[deku(count = "count")]
    pub data: Vec<Character>,
}
string_impl!(String16);

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct String32 {
    #[deku(pad_bits_before = "3", bits = "5", update = "self.data.len()")]
    pub count: u8,
    #[deku(count = "count")]
    pub data: Vec<Character>,
}
string_impl!(String32);

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct String64 {
    #[deku(pad_bits_before = "2", bits = "6", update = "self.data.len()")]
    pub count: u8,
    #[deku(count = "count")]
    pub data: Vec<Character>,
}
string_impl!(String64);

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct String128 {
    #[deku(pad_bits_before = "1", bits = "7", update = "self.data.len()")]
    pub count: u8,
    #[deku(count = "count")]
    pub data: Vec<Character>,
}
string_impl!(String128);

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct String256 {
    pub count: u8,
    #[deku(count = "count")]
    pub data: Vec<Character>,
}
string_impl!(String256);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::character::Character;
    use deku::prelude::*;
    use std::convert::TryFrom;

    #[test]
    fn test_serialize_and_deserialize() {
        let mut data = vec![7u8];
        data.append(&mut vec![0x30u8; 7]);

        let (rest, mut val) = String16::from_bytes((data.as_ref(), 0)).unwrap();

        assert_eq!(
            val,
            String16 {
                count: 7,
                data: vec![Character::try_from(0x30).unwrap(); 7]
            }
        );

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    #[should_panic]
    fn test_invalid_unicode() {
        let mut data = vec![7u8];
        data.append(&mut vec![0xff; 7]);

        String16::from_bytes((data.as_ref(), 0)).unwrap();
    }

    #[test]
    fn test_update() {
        let mut data = vec![6u8];
        data.append(&mut vec![0x30; 6]);

        let (rest, mut val) = String16::from_bytes((data.as_ref(), 0)).unwrap();

        val.data.push(Character::try_from(0x30).unwrap());
        assert_eq!(
            val,
            String16 {
                count: 6,
                data: vec![Character::try_from(0x30).unwrap(); 7]
            }
        );
        val.update();
        assert_eq!(
            val,
            String16 {
                count: 7,
                data: vec![Character::try_from(0x30).unwrap(); 7]
            }
        );
    }

    #[test]
    fn test_try_from_u8_vec() {
        let data = vec![0x30u8; 6];
        let val = String8::try_from(data).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_try_from_u8_vec_invalid() {
        let data = vec![0xffu8];
        let val = String8::try_from(data).unwrap();
    }
}
