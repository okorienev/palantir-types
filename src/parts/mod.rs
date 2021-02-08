use deku::prelude::*;

use crate::primitives::character::Character;

pub mod string;

pub type Realm = string::String32;
pub type Application = string::String32;
pub type Status = string::String8;
