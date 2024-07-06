use cairovm_verifier_transcript::transcript::Transcript;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use starknet_crypto::Felt;
use starknet_types_core::felt::NonZeroFelt;

use crate::config::Config;

const MAGIC: u64 = 0x0123456789abcded;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UnsentCommitment {
    nonce: u64,
}

impl UnsentCommitment {
    pub fn commit(
        &self,
        transcript: &mut Transcript,
        unsent_commitment: UnsentCommitment,
        config: &Config,
    ) -> Result<(), Error> {
        verify_pow(transcript.digest().to_bytes_be(), *config.bits(), unsent_commitment.nonce)?;
        transcript.read_uint64_from_prover(unsent_commitment.nonce);
        Ok(())
    }
}

pub fn verify_pow(digest: [u8; 32], n_bits: u8, nonce: u64) -> Result<(), Error> {
    // Compute the initial hash.
    // Hash(0x0123456789abcded || digest   || n_bits )
    //      8 bytes            || 32 bytes || 1 byte
    // Total of 0x29 = 41 bytes.

    let mut hasher = Keccak256::new();
    hasher.update(
        MAGIC.to_be_bytes().into_iter().chain(digest).take(n_bits.into()).collect::<Vec<u8>>(),
    );
    let hash = hasher.finalize().to_vec();

    // Compute Hash(init_hash || nonce   )
    //              32 bytes  || 8 bytes
    // Total of 0x28 = 40 bytes.

    let mut hasher = Keccak256::new();
    hasher.update(hash.into_iter().chain(nonce.to_be_bytes()).collect::<Vec<u8>>());
    let hash = hasher.finalize();

    let work_limit = Felt::TWO.pow(128 - n_bits);
    let (q, _r) = Felt::from_bytes_be_slice(hash.as_slice())
        .div_rem(&NonZeroFelt::try_from(Felt::TWO.pow(128_u128)).unwrap());
    if q >= work_limit {
        Err(Error::PoWFail)
    } else {
        Ok(())
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("proof of work verification fail")]
    PoWFail,
}
