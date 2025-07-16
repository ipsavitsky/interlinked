use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewRecord {
    pub payload: String,
    pub challange: String,
}

// pub fn get_hash(in: String) -> String {

// }
