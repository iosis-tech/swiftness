use cairovm_verifier_commitment::{
    table::types::Commitment as TableCommitment,
    vector::{config::Config as VectorCommitmentConfig, types::Commitment as VectorCommitment},
};
use starknet_crypto::Felt;

use crate::trace::Commitment;

use super::{config, interaction_elements, unsent_commitment};

pub fn get() -> Commitment {
    let unsent_commitment = unsent_commitment::get();
    let traces_config = config::get();

    Commitment {
        original: TableCommitment {
            config: traces_config.original,
            vector_commitment: VectorCommitment {
                config: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
                commitment_hash: unsent_commitment.original,
            },
        },
        interaction_elements: interaction_elements::get(),
        interaction: TableCommitment {
            config: traces_config.interaction,
            vector_commitment: VectorCommitment {
                config: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
                commitment_hash: unsent_commitment.interaction,
            },
        },
    }
}
