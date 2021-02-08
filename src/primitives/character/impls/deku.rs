use deku::prelude::*;

use super::super::bitmap::BITMAP;
use super::super::Character;
use crate::primitives::character::error::CharacterError;
use std::convert::TryFrom;

impl DekuRead<'_> for Character {
    fn read<'a>(
        input: &'a BitSlice<Msb0, u8>,
        ctx: (),
    ) -> Result<(&BitSlice<Msb0, u8>, Self), DekuError>
    where
        Self: Sized,
    {
        let (rest, val) = u8::read(input, ctx)?;

        match &Character::try_from(val) {
            Ok(c) => Ok((rest, *c)),
            Err(e) => Err(DekuError::InvalidParam(e.to_string())),
        }
    }
}

impl DekuWrite for Character {
    fn write(&self, output: &mut BitVec<Msb0, u8>, ctx: ()) -> Result<(), DekuError> {
        self.0.write(output, ctx)
    }
}
