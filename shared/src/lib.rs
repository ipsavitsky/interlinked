use hmac_sha256::Hash;
use serde::{Deserialize, Serialize};
use tinyrand::{RandRange, Seeded, StdRand};

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[derive(Serialize, Deserialize)]
pub struct NewRecordScheme {
    pub payload: url::Url,
    pub challenge: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewNoteScheme {
    pub payload: String,
    pub challenge: String,
}

pub trait RecordPayload {
    fn as_str(&self) -> &str;
    fn record_type(&self) -> &'static str;
    fn challenge(&self) -> &str;
    fn with_challenge(&self, challenge: String) -> Self
    where
        Self: Sized;
}

impl RecordPayload for NewRecordScheme {
    fn as_str(&self) -> &str {
        self.payload.as_str()
    }

    fn record_type(&self) -> &'static str {
        "link"
    }

    fn challenge(&self) -> &str {
        &self.challenge
    }

    fn with_challenge(&self, challenge: String) -> Self
    where
        Self: Sized,
    {
        NewRecordScheme {
            payload: self.payload.clone(),
            challenge,
        }
    }
}

impl RecordPayload for NewNoteScheme {
    fn as_str(&self) -> &str {
        &self.payload
    }

    fn record_type(&self) -> &'static str {
        "note"
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

pub fn get_hash(in_str: &str) -> String {
    String::from_utf8_lossy(&Hash::hash(in_str.as_bytes())).into_owned()
}

fn make_random_string(rand: &mut StdRand) -> String {
    (0..70)
        .map(|_| {
            let idx = rand.next_range(0..ALPHABET.len());
            ALPHABET[idx] as char
        })
        .collect()
}

pub fn come_up_with_solution(diff: usize, seed: u64) -> (String, String) {
    let mut rand = StdRand::seed(seed);
    let prefix = "0".repeat(diff);
    loop {
        let attempt = make_random_string(&mut rand);
        let hash = get_hash(&attempt);
        if hash.starts_with(&prefix) {
            return (attempt, hash);
        }
    }
}
