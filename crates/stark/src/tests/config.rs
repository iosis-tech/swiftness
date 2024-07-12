use cairovm_verifier_commitment::{table, vector};
use starknet_crypto::Felt;

use crate::config::StarkConfig;

pub fn get() -> StarkConfig {
    return StarkConfig {
        traces: cairovm_verifier_air::layout::recursive::tests::trace_config::get(),
        composition: table::config::Config {
            n_columns: Felt::from_hex_unchecked("0x2"),
            vector: vector::config::Config {
                height: Felt::from_hex_unchecked("0x14"),
                n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
            },
        },
        fri: cairovm_verifier_fri::tests::config::get(),
        proof_of_work: cairovm_verifier_pow::tests::config::get(),
        log_trace_domain_size: Felt::from_hex_unchecked("0x12"),
        n_queries: Felt::from_hex_unchecked("0xa"),
        log_n_cosets: Felt::from_hex_unchecked("0x2"),
        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
    };
}
