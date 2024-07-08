use cairovm_verifier_commitment::{
    table::{config::Config as TableCommitmentConfig, types::Commitment as TableCommitment},
    vector::{config::Config as VectorCommitmentConfig, types::Commitment as VectorCommitment},
};
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

use crate::{
    fri::{fri_commit, FriCommitment},
    tests::config,
};

use super::*;

#[test]
fn test_fri_commit() {
    let mut channel = Transcript::new_with_counter(
        Felt::from_hex("0x3612d68f9f68b263d83b0854b812018fd1b7ba0359d7514fffdabd44f0696e6")
            .unwrap(),
        Felt::from_hex("0x1").unwrap(),
    );

    let fri_config = config::get();
    let unsent_commitment = unsent_commitment::get();

    assert_eq!(fri_commit(&mut channel, unsent_commitment, fri_config), commit::get())
}

pub fn get() -> FriCommitment {
    FriCommitment {
        config: config::get(),
        inner_layers: vec![
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex("0x10").unwrap(),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex("0x12").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex("0x12").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                    commitment_hash: Felt::from_hex(
                        "0x137de087f31f4e6f54222fc3cebb3c162469083196999e6ee4bb8ceb4d6b786",
                    )
                    .unwrap(),
                },
            },
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex("0x8").unwrap(),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex("0xf").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex("0xf").unwrap(),
                        n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
                    },
                    commitment_hash: Felt::from_hex(
                        "0x3bb3c75d228842edce6f6bf6fd6706ce51f5d83c6842a3ab4b4d89fad6f07b",
                    )
                    .unwrap(),
                },
            },
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex("0x4").unwrap(),
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
                        "0xb606d3c2b341ff9de5ead44f00121fdc4113f3720feb162eeaecb511e73d4f",
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
                        "0x787b0937a4cd02e0143e93979bb79139ca9546fc1654b4f755f8642c989ba20",
                    )
                    .unwrap(),
                },
            },
        ],
        eval_points: vec![
            Felt::from_hex("0x2318111dbaa02700a1ac0d1ce605b756010af6c39b4e85422e9e8c848ec05ca")
                .unwrap(),
            Felt::from_hex("0xe32c017cfa9260ed2130df2d513340c4a5aaee766696beb2f640ad261e0261")
                .unwrap(),
            Felt::from_hex("0x4103675a55bf63ad036370ded26f12e273026699c056d578c6b01dff2c3e9e0")
                .unwrap(),
            Felt::from_hex("0x2cda81790074e40739eb81556de82ebc000056aafcc09c34f5ba52d6d0ff1ba")
                .unwrap(),
        ],
        last_layer_coefficients: last_layer_coefficients::get(),
    }
}
