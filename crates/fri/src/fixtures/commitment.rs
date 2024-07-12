use cairovm_verifier_commitment::{
    table::{config::Config as TableCommitmentConfig, types::Commitment as TableCommitment},
    vector::{config::Config as VectorCommitmentConfig, types::Commitment as VectorCommitment},
};
use starknet_crypto::Felt;

use crate::types::Commitment as FriCommitment;

use super::{config, last_layer_coefficients};

pub fn get() -> FriCommitment {
    FriCommitment {
        config: config::get(),
        inner_layers: vec![
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex("0x10").unwrap(),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex("0x10").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex("0x10").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                    commitment_hash: Felt::from_hex(
                        "0x31b917291bbb3d38f7bc196dee1f3638ca197512162a4bdeb1ce814619c1625",
                    )
                    .unwrap(),
                },
            },
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex("0x8").unwrap(),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex("0xd").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex("0xd").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                    commitment_hash: Felt::from_hex(
                        "0x6945b2895872a701b3451cdf93dca9cba3cad8f250d5866ca5c263e41c8f2b2",
                    )
                    .unwrap(),
                },
            },
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex("0x4").unwrap(),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex("0xb").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex("0xb").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                    commitment_hash: Felt::from_hex(
                        "0x786c3ebbd4cab0c782d36860cd51887712953c48ce72c8d10acf5698c5a1213",
                    )
                    .unwrap(),
                },
            },
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex("0x4").unwrap(),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex("0x9").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex("0x9").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                    commitment_hash: Felt::from_hex(
                        "0x1e9b0fa29ebe52b9c9a43a1d44e555ce42da3199370134d758735bfe9f40269",
                    )
                    .unwrap(),
                },
            },
        ],
        eval_points: vec![
            Felt::from_hex("0x3fa22931f1e5f47eb6273e90ee38c37a21730bb432f6ef09c7c8f8c4e7b7fff")
                .unwrap(),
            Felt::from_hex("0x72089def6cbdc9a9ad42dab128c499727e36b05d40df74bbccff39650569bd")
                .unwrap(),
            Felt::from_hex("0x311de180838f0ad6e82a20d03978ddefb1c73bcee966471479a6c70fdc05f34")
                .unwrap(),
            Felt::from_hex("0x18e241a9c138d318daa567510ba31c4ebeecdaab9076b3a8828dbb4b3303e3")
                .unwrap(),
        ],
        last_layer_coefficients: last_layer_coefficients::get(),
    }
}
