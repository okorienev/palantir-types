use std::convert::TryFrom;
use std::ops::Deref;

use super::super::bitmap::BITMAP;
use super::super::error::CharacterError;
use super::super::Character;

impl Deref for Character {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<u8> for Character {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl TryFrom<u8> for Character {
    type Error = CharacterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !BITMAP.check(value) {
            return Err(CharacterError::DisallowedCharCode(value));
        }

        Ok(Self(value))
    }
}
