use hmac_sha256::Hash;
use tinyrand::{RandRange, Seeded, StdRand};

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn make_random_string(rand: &mut StdRand) -> String {
    (0..70)
        .map(|_| {
            let idx = rand.next_range(0..ALPHABET.len());
            ALPHABET[idx] as char
        })
        .collect()
}

pub fn hash_string(in_str: &str) -> String {
    String::from_utf8_lossy(&Hash::hash(in_str.as_bytes())).into_owned()
}

pub fn solve_pow_challenge(diff: usize, seed: u64) -> (String, String) {
    let mut rand = StdRand::seed(seed);
    let prefix = "0".repeat(diff);
    loop {
        let attempt = make_random_string(&mut rand);
        let hash = hash_string(&attempt);
        if hash.starts_with(&prefix) {
            return (attempt, hash);
        }
    }
}
