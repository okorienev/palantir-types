use std::ops::Deref;

use deku::prelude::*;
use deku::ctx::Size;

use super::{Character};
use super::bitmap::{BITMAP};


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


impl DekuRead<'_> for Character {
    fn read<'a>(input: &'a BitSlice<Msb0, u8>, ctx: ()) -> Result<(&BitSlice<Msb0, u8>, Self), DekuError> where
        Self: Sized {
        let (rest, val) = u8::read(input, ctx)?;

        if !BITMAP.check(val) {
            return Err(DekuError::InvalidParam(format!(
                "character code {} is not allowed", val
            )));
        };

        Ok((rest, Character(val)))
    }
}

impl DekuWrite for Character {
    fn write(&self, output: &mut BitVec<Msb0, u8>, ctx: ()) -> Result<(), DekuError> {
        if !BITMAP.check(self.0) {
            return Err(DekuError::InvalidParam(format!(
                "character code {} is not allowed", self.0
            )))
        };

        self.0.write(output,  ctx)
    }
}