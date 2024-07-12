use crate::config::Config;
use cairovm_verifier_commitment::{table, vector};
use starknet_crypto::Felt;

pub fn get() -> Config {
    Config {
        log_input_size: Felt::from_hex("0x14").unwrap(),
        n_layers: Felt::from_hex("0x5").unwrap(),
        inner_layers: vec![
            table::config::Config {
                n_columns: Felt::from_hex("0x10").unwrap(),
                vector: vector::config::Config {
                    height: Felt::from_hex("0x10").unwrap(),
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex("0x8").unwrap(),
                vector: vector::config::Config {
                    height: Felt::from_hex("0xd").unwrap(),
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex("0x4").unwrap(),
                vector: vector::config::Config {
                    height: Felt::from_hex("0xb").unwrap(),
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex("0x4").unwrap(),
                vector: vector::config::Config {
                    height: Felt::from_hex("0x9").unwrap(),
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                },
            },
        ],
        fri_step_sizes: vec![
            Felt::from_hex("0x0").unwrap(),
            Felt::from_hex("0x4").unwrap(),
            Felt::from_hex("0x3").unwrap(),
            Felt::from_hex("0x2").unwrap(),
            Felt::from_hex("0x2").unwrap(),
        ],
        log_last_layer_degree_bound: Felt::from_hex("0x7").unwrap(),
    }
}
