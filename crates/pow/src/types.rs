use serde::{Deserialize, Serialize};

pub const MAGIC: u64 = 0x0123456789abcded;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Proof of work difficulty (number of bits required to be 0).
    n_bits: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsentCommitment {
    nonce: u64,
}
