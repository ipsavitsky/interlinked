use serde::{Deserialize, Serialize};

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
    fn payload(&self) -> &str;
    fn record_type(&self) -> RecordType;
    fn challenge(&self) -> &str;
    fn with_challenge(&self, challenge: String) -> Self
    where
        Self: Sized;
}

impl RecordPayload for NewNoteScheme {
    fn payload(&self) -> &str {
        &self.payload
    }

    fn record_type(&self) -> RecordType {
        RecordType::Note
    }

    fn challenge(&self) -> &str {
        &self.challenge
    }

    fn with_challenge(&self, challenge: String) -> Self
    where
        Self: Sized,
    {
        NewNoteScheme {
            payload: self.payload.clone(),
            challenge,
        }
    }
}

impl RecordPayload for NewLinkScheme {
    fn payload(&self) -> &str {
        self.payload.as_str()
    }

    fn record_type(&self) -> RecordType {
        RecordType::Link
    }

    fn challenge(&self) -> &str {
        &self.challenge
    }

    fn with_challenge(&self, challenge: String) -> Self
    where
        Self: Sized,
    {
        NewLinkScheme {
            payload: self.payload.clone(),
            challenge,
        }
    }
}
