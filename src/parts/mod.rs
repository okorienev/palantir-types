use deku::prelude::*;

pub mod string;

pub type Realm = string::String32;
pub type Application = string::String32;
pub type Status = string::String16;
pub type Action = string::String256;
pub type ApplicationHash = string::String32;

pub type PartName = string::String32;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct TrackingPart {
    pub name: PartName,
    pub hits: u32,
    pub total_duration: u64,
}
