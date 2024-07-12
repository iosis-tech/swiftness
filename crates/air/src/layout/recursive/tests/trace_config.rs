use cairovm_verifier_commitment::{table, vector};
use starknet_crypto::Felt;

use crate::trace::config::Config;

pub fn get() -> Config {
    return Config {
        original: table::config::Config {
            n_columns: Felt::from_hex_unchecked("0x7"),
            vector: vector::config::Config {
                height: Felt::from_hex_unchecked("0x14"),
                n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
            },
        },
        interaction: table::config::Config {
            n_columns: Felt::from_hex_unchecked("0x3"),
            vector: vector::config::Config {
                height: Felt::from_hex_unchecked("0x14"),
                n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
            },
        },
    };
}
