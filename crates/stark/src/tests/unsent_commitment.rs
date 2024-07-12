use crate::types::StarkUnsentCommitment;
use starknet_crypto::Felt;

use super::oods_values;

pub fn get() -> StarkUnsentCommitment {
    StarkUnsentCommitment {
        traces: cairovm_verifier_air::layout::recursive::tests::trace_unsent_commitment::get(),
        composition: Felt::from_hex_unchecked(
            "0x30b93bbd6b193eb57d9f818202b899b7e8e09b0c7d183537fe85f4e6b6f4373",
        ),
        oods_values: oods_values::get(),
        fri: cairovm_verifier_fri::tests::unsent_commitment::get(),
        proof_of_work: cairovm_verifier_pow::tests::unsent_commitment::get(),
    }
}
