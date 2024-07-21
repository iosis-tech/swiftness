use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

use super::{config::Config, types::Commitment};

pub fn vector_commit(
    transcript: &mut Transcript,
    unsent_commitment: Felt,
    config: Config,
) -> Commitment {
    transcript.read_felt_from_prover(&unsent_commitment);
    Commitment { commitment_hash: unsent_commitment, config }
}
