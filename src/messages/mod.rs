use deku::prelude::*;

pub mod apm_v1;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bytes = 1)]
pub enum Message {
    #[deku(id = "0x01")]
    ApmV1(apm_v1::APMV1Message),
}
