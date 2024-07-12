use cairovm_verifier_commitment::{table, vector};
use starknet_crypto::Felt;

use crate::trace::Commitment;

use super::{interaction_elements, trace_config, trace_unsent_commitment};

pub fn get() -> Commitment {
    let unsent_commitment = trace_unsent_commitment::get();
    let config = trace_config::get();

    return Commitment {
        original: table::types::Commitment {
            config: config.original,
            vector_commitment: vector::types::Commitment {
                config: vector::config::Config {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
                commitment_hash: unsent_commitment.original,
            },
        },
        interaction_elements: interaction_elements::get(),
        interaction: table::types::Commitment {
            config: config.interaction,
            vector_commitment: vector::types::Commitment {
                config: vector::config::Config {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
                commitment_hash: unsent_commitment.interaction,
            },
        },
    };
}
