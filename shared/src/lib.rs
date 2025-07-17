use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Serialize, Deserialize)]
pub struct NewRecord {
    pub payload: String,
    pub challenge: String,
}

pub fn get_hash(in_str: &str) -> String {
    digest(in_str)
}
