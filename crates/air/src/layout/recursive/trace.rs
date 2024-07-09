// Reads the traces commitment from the transcript.
// Returns the commitment, along with GlobalValue required to evaluate the constraint polynomial.

use cairovm_verifier_commitment::table::{commit::table_commit, decommit::table_decommit};
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

use crate::trace::{
    config::Config, decommit::Error, Commitment, Decommitment, UnsentCommitment, Witness,
};

use super::global_values::InteractionElements;

pub fn traces_commit(
    transcript: &mut Transcript,
    unsent_commitment: &UnsentCommitment,
    config: Config,
) -> Commitment {
    // Read original commitment.
    let original_commitment = table_commit(transcript, unsent_commitment.original, config.original);

    // Generate interaction elements for the first interaction.
    let interaction_elements = InteractionElements {
        memory_multi_column_perm_perm_interaction_elm: transcript.random_felt_to_prover(),
        memory_multi_column_perm_hash_interaction_elm0: transcript.random_felt_to_prover(),
        range_check16_perm_interaction_elm: transcript.random_felt_to_prover(),
        diluted_check_permutation_interaction_elm: transcript.random_felt_to_prover(),
        diluted_check_interaction_z: transcript.random_felt_to_prover(),
        diluted_check_interaction_alpha: transcript.random_felt_to_prover(),
    };

    // Read interaction commitment.
    let interaction_commitment =
        table_commit(transcript, unsent_commitment.interaction, config.interaction);

    Commitment {
        original: original_commitment,
        interaction_elements,
        interaction: interaction_commitment,
    }
}

// Verifies a decommitment for the traces at the query indices.
// decommitment - holds the commited values of the leaves at the query_indices.
pub fn traces_decommit(
    queries: &[Felt],
    commitment: Commitment,
    decommitment: Decommitment,
    witness: Witness,
) -> Result<(), Error> {
    Ok(table_decommit(commitment.original, queries, decommitment.original, witness.original).and(
        table_decommit(
            commitment.interaction,
            queries,
            decommitment.interaction,
            witness.interaction,
        ),
    )?)
}
