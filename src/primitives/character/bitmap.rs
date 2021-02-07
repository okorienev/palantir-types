use std::vec::Vec;

use lazy_static::lazy_static;

pub const ALLOWED_UTF8_CHARACTERS: [u8; 65] = [
    0x2d,  // -     HYPHEN-MINUS
    0x2e,  // .     FULL STOP
    0x5f,  // _     LOW LINE

    0x30,  // 0     DIGIT ZERO
    0x31,  // 1     DIGIT ONE
    0x32,  // 2     DIGIT TWO
    0x33,  // 3     DIGIT THREE
    0x34,  // 4     DIGIT FOUR
    0x35,  // 5     DIGIT FIVE
    0x36,  // 6     DIGIT SIX
    0x37,  // 7     DIGIT SEVEN
    0x38,  // 8     DIGIT EIGHT
    0x39,  // 9     DIGIT NINE

    0x41,  // A     LATIN CAPITAL LETTER A
    0x42,  // B     LATIN CAPITAL LETTER B
    0x43,  // C     LATIN CAPITAL LETTER C
    0x44,  // D     LATIN CAPITAL LETTER D
    0x45,  // E     LATIN CAPITAL LETTER E
    0x46,  // F     LATIN CAPITAL LETTER F
    0x47,  // G     LATIN CAPITAL LETTER G
    0x48,  // H     LATIN CAPITAL LETTER H
    0x49,  // I     LATIN CAPITAL LETTER I
    0x4a,  // J     LATIN CAPITAL LETTER J
    0x4b,  // K     LATIN CAPITAL LETTER K
    0x4c,  // L     LATIN CAPITAL LETTER L
    0x4d,  // M     LATIN CAPITAL LETTER M
    0x4e,  // N     LATIN CAPITAL LETTER N
    0x4f,  // O     LATIN CAPITAL LETTER O
    0x50,  // P     LATIN CAPITAL LETTER P
    0x51,  // Q     LATIN CAPITAL LETTER Q
    0x52,  // R     LATIN CAPITAL LETTER R
    0x53,  // S     LATIN CAPITAL LETTER S
    0x54,  // T     LATIN CAPITAL LETTER T
    0x55,  // U     LATIN CAPITAL LETTER U
    0x56,  // V     LATIN CAPITAL LETTER V
    0x57,  // W     LATIN CAPITAL LETTER W
    0x58,  // X     LATIN CAPITAL LETTER X
    0x59,  // Y     LATIN CAPITAL LETTER Y
    0x5a,  // Z     LATIN CAPITAL LETTER Z

    0x61,  // a 	LATIN SMALL LETTER A
    0x62,  // b 	LATIN SMALL LETTER B
    0x63,  // c 	LATIN SMALL LETTER C
    0x64,  // d 	LATIN SMALL LETTER D
    0x65,  // e 	LATIN SMALL LETTER E
    0x66,  // f 	LATIN SMALL LETTER F
    0x67,  // g 	LATIN SMALL LETTER G
    0x68,  // h 	LATIN SMALL LETTER H
    0x69,  // i 	LATIN SMALL LETTER I
    0x6a,  // j 	LATIN SMALL LETTER J
    0x6b,  // k 	LATIN SMALL LETTER K
    0x6c,  // l 	LATIN SMALL LETTER L
    0x6d,  // m 	LATIN SMALL LETTER M
    0x6e,  // n 	LATIN SMALL LETTER N
    0x6f,  // o 	LATIN SMALL LETTER O
    0x70,  // p 	LATIN SMALL LETTER P
    0x71,  // q 	LATIN SMALL LETTER Q
    0x72,  // r 	LATIN SMALL LETTER R
    0x73,  // s 	LATIN SMALL LETTER S
    0x74,  // t 	LATIN SMALL LETTER T
    0x75,  // u 	LATIN SMALL LETTER U
    0x76,  // v 	LATIN SMALL LETTER V
    0x77,  // w 	LATIN SMALL LETTER W
    0x78,  // x 	LATIN SMALL LETTER X
    0x79,  // y   	LATIN SMALL LETTER Y
    0x7a,  // z 	LATIN SMALL LETTER Z
];

const U8_BITS: u8 = 8;
const MAP_SIZE: usize = 64;

const MASK_1:   u8 = 0b00000001;
const MASK_2:   u8 = 0b00000010;
const MASK_4:   u8 = 0b00000100;
const MASK_8:   u8 = 0b00001000;
const MASK_16:  u8 = 0b00010000;
const MASK_32:  u8 = 0b00100000;
const MASK_64:  u8 = 0b01000000;
const MASK_128: u8 = 0b10000000;

const MASKS: [u8; U8_BITS as usize] = [
    MASK_1,
    MASK_2,
    MASK_4,
    MASK_8,
    MASK_16,
    MASK_32,
    MASK_64,
    MASK_128,
];


/// Represents data structure to make character allowance check with constant complexity  
/// Main idea:
/// * Only 0-255 character codes are theoretically allowed
/// * We form a bit vector with 256 bit (64 byte) length
/// * If bit value at char code index is 1 then it's allowed and disallowed otherwise
pub struct BitMap {
    pub map: Vec<u8>
}

impl BitMap {
    pub fn generate() -> BitMap {
        let mut map: Vec<u8> = vec![0; MAP_SIZE];

        for char_code in ALLOWED_UTF8_CHARACTERS.iter() {
            let byte_idx = char_code / U8_BITS;
            let bit_idx = char_code % U8_BITS;

            map[byte_idx as usize] = map[byte_idx as usize] | MASKS[bit_idx as usize];
        }

        BitMap { map }
    }

    #[inline(always)]
    pub fn check(&self, val: u8) -> bool {
        let byte_idx = val / U8_BITS;
        let bit_idx = val % U8_BITS;

        (self.map[byte_idx as usize] & MASKS[bit_idx as usize]) != 0
    }
}

lazy_static! {
    pub static ref BITMAP: BitMap = BitMap::generate();
}

#[cfg(test)]
mod tests {
    use deku::prelude::*;
    use crate::primitives::character::bitmap::{BitMap, ALLOWED_UTF8_CHARACTERS};

    #[test]
    fn test_bitmap_valid() {
        let bitmap = BitMap::generate();

        for char_code in ALLOWED_UTF8_CHARACTERS.iter() {
            assert!(bitmap.check(*char_code))
        }
    }

    #[test]
    fn test_bitmap_invalid() {
        let bitmap = BitMap::generate();

        for char_code in 0u8..0x20 {  // control symbols
            assert!(!bitmap.check(char_code))
        }

        for char_code in 0x3au8..0x41 {  // different signs
            assert!(!bitmap.check(char_code))
        }
    }
}