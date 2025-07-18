use hmac_sha256::Hash;
use serde::{Deserialize, Serialize};
use tinyrand::{Rand, RandRange, StdRand, Seeded};
use tinyrand_std::ClockSeed;

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[derive(Serialize, Deserialize)]
pub struct NewRecordScheme {
    pub payload: String,
    pub challenge: String,
}

pub fn get_hash(in_str: &str) -> String {
    String::from_utf8_lossy(&Hash::hash(in_str.as_bytes())).into_owned()
}

fn make_random_string() -> String {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    (0..70).map(|_| {
        let idx = rand.next_range(0..ALPHABET.len());
        ALPHABET[idx] as char
    }).collect()
}

pub fn come_up_with_solution(diff: usize) -> (String, String) {
    let prefix = "0".repeat(diff);
    loop {
        let attempt = make_random_string();
        let hash = get_hash(&attempt);
        if hash.starts_with(&prefix) {
            return (attempt, hash);
        }
    }
}
