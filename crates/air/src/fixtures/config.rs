use cairovm_verifier_commitment::{
    table::config::Config as TableConfig, vector::config::Config as VectorConfig,
};
use starknet_crypto::Felt;

use crate::trace::config::Config;

pub fn get() -> Config {
    Config {
        original: TableConfig {
            n_columns: Felt::from_hex_unchecked("0x7"),
            vector: VectorConfig {
                height: Felt::from_hex_unchecked("0x14"),
                n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
            },
        },
        interaction: TableConfig {
            n_columns: Felt::from_hex_unchecked("0x3"),
            vector: VectorConfig {
                height: Felt::from_hex_unchecked("0x14"),
                n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
            },
        },
    }
}
