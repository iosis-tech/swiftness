use cairovm_verifier_transcript::transcript::Transcript;
use serde::{Deserialize, Serialize};

use crate::config::Config;

const MAGIC: u64 = 0x0123456789abcded;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsentCommitment {
    nonce: u64,
}

impl UnsentCommitment {
    pub fn commit(&self, config: Config, transcript: &mut Transcript) {}
}
