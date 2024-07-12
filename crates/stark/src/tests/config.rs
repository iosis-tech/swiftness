use cairovm_verifier_commitment::{
    table::config::Config as TableCommitmentConfig,
    vector::config::Config as VectorCommitmentConfig,
};
use starknet_crypto::Felt;

use crate::config::StarkConfig;

pub fn get() -> StarkConfig {
    StarkConfig {
        traces: cairovm_verifier_air::fixtures::config::get(),
        composition: TableCommitmentConfig {
            n_columns: Felt::from_hex_unchecked("0x2"),
            vector: VectorCommitmentConfig {
                height: Felt::from_hex_unchecked("0x14"),
                n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
            },
        },
        fri: cairovm_verifier_fri::fixtures::config::get(),
        proof_of_work: cairovm_verifier_pow::tests::config::get(),
        log_trace_domain_size: Felt::from_hex_unchecked("0x12"),
        n_queries: Felt::from_hex_unchecked("0xa"),
        log_n_cosets: Felt::from_hex_unchecked("0x2"),
        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
    }
}
