// Reads the traces commitment from the transcript.
// Returns the commitment, along with GlobalValue required to evaluate the constraint polynomial.

use cairovm_verifier_commitment::table::commit::table_commit;
use cairovm_verifier_transcript::transcript::Transcript;

use crate::types::trace::{Commitment, Config, UnsentCommitment};

use super::global_values::InteractionElements;

pub fn traces_commit(
    transcript: &mut Transcript,
    unsent_commitment: UnsentCommitment,
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
