use super::{config, interaction_elements, unsent_commitment};
use crate::{layout::recursive::global_values::InteractionElements, trace::Commitment};
use cairovm_verifier_commitment::{table, vector};
use starknet_crypto::Felt;

pub fn get() -> Commitment<InteractionElements> {
    let unsent_commitment = unsent_commitment::get();
    let traces_config = config::get();

    Commitment {
        original: table::types::Commitment {
            config: traces_config.original,
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
            config: traces_config.interaction,
            vector_commitment: vector::types::Commitment {
                config: vector::config::Config {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
                commitment_hash: unsent_commitment.interaction,
            },
        },
    }
}
