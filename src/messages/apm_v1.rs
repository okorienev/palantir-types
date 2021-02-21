use deku::prelude::*;

use crate::parts::{Action, Application, ApplicationHash, Realm, Status, TrackingPart};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct APMV1Message {
    pub realm: Realm,
    pub application: Application,
    pub application_hash: ApplicationHash,
    pub action: Action,
    pub status: Status,
    pub duration: u64,

    #[deku(update = "self.parts.len()")]
    pub parts_count: u8,
    #[deku(count = "parts_count")]
    pub parts: Vec<TrackingPart>,
}
