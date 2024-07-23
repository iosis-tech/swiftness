use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_crypto::Felt;

use crate::config;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkProof {
    pub config: config::StarkConfig,
    pub public_input: swiftness_air::public_memory::PublicInput,
    pub unsent_commitment: StarkUnsentCommitment,
    pub witness: StarkWitness,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkUnsentCommitment {
    pub traces: swiftness_air::trace::UnsentCommitment,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub composition: Felt,
    // n_oods_values elements. The i-th value is the evaluation of the i-th mask item polynomial at
    // the OODS point, where the mask item polynomial is the interpolation polynomial of the
    // corresponding column shifted by the corresponding row_offset.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub oods_values: Vec<Felt>,
    pub fri: swiftness_fri::types::UnsentCommitment,
    pub proof_of_work: swiftness_pow::pow::UnsentCommitment,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkCommitment<InteractionElements> {
    pub traces: swiftness_air::trace::Commitment<InteractionElements>,
    pub composition: swiftness_commitment::table::types::Commitment,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub interaction_after_composition: Felt,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub oods_values: Vec<Felt>,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub interaction_after_oods: Vec<Felt>,
    pub fri: swiftness_fri::types::Commitment,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkWitness {
    pub traces_decommitment: swiftness_air::trace::Decommitment,
    pub traces_witness: swiftness_air::trace::Witness,
    pub composition_decommitment: swiftness_commitment::table::types::Decommitment,
    pub composition_witness: swiftness_commitment::table::types::Witness,
    pub fri_witness: swiftness_fri::types::Witness,
}
