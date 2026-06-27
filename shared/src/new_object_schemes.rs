use serde::{Deserialize, Serialize};
use url::Url;

use crate::routes::RecordType;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewLinkScheme {
    pub payload: url::Url,
    pub challenge: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewNoteScheme {
    pub payload: String,
    pub challenge: String,
}

pub trait RecordPayload {
    fn from_parts(payload: String, challenge: String) -> Self;
    fn payload(&self) -> &str;
    fn record_type() -> RecordType;
    fn challenge(&self) -> &str;
}

impl RecordPayload for NewNoteScheme {
    fn from_parts(payload: String, challenge: String) -> Self {
        NewNoteScheme { payload, challenge }
    }

    fn payload(&self) -> &str {
        &self.payload
    }

    fn record_type() -> RecordType {
        RecordType::Note
    }

    fn challenge(&self) -> &str {
        &self.challenge
    }
}

impl RecordPayload for NewLinkScheme {
    fn from_parts(payload: String, challenge: String) -> Self {
        NewLinkScheme {
            payload: Url::parse(&payload).unwrap(),
            challenge,
        }
    }

    fn payload(&self) -> &str {
        self.payload.as_str()
    }

    fn record_type() -> RecordType {
        RecordType::Link
    }

    fn challenge(&self) -> &str {
        &self.challenge
    }
}
