use serde::{Deserialize, Serialize};
use std::fmt;

pub const API_PREFIX: &str = "/api";
pub const API_DIFFICULTY: &str = "/api/difficulty";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordType {
    Link,
    Note,
}

impl RecordType {
    pub const fn route_prefix(self) -> &'static str {
        match self {
            RecordType::Link => "/l",
            RecordType::Note => "/n",
        }
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordType::Link => write!(f, "link"),
            RecordType::Note => write!(f, "note"),
        }
    }
}
