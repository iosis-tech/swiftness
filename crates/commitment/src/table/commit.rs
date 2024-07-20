use super::{config::Config, types::Commitment};
use crate::vector::commit::vector_commit;
use alloc::borrow::ToOwned;
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

pub fn table_commit(
    transcript: &mut Transcript,
    unsent_commitment: Felt,
    config: Config,
) -> Commitment {
    let vector_commitment = vector_commit(transcript, unsent_commitment, config.vector.to_owned());
    Commitment { config, vector_commitment }
}
