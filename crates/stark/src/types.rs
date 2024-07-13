use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

use crate::config;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkProof {
    pub config: config::StarkConfig,
    pub public_input: cairovm_verifier_air::public_memory::PublicInput,
    pub unsent_commitment: StarkUnsentCommitment,
    pub witness: StarkWitness,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkUnsentCommitment {
    pub traces: cairovm_verifier_air::trace::UnsentCommitment,
    #[serde_as(as = "UfeHex")]
    pub composition: Felt,
    // n_oods_values elements. The i-th value is the evaluation of the i-th mask item polynomial at
    // the OODS point, where the mask item polynomial is the interpolation polynomial of the
    // corresponding column shifted by the corresponding row_offset.
    #[serde_as(as = "Vec<UfeHex>")]
    pub oods_values: Vec<Felt>,
    pub fri: cairovm_verifier_fri::types::UnsentCommitment,
    pub proof_of_work: cairovm_verifier_pow::pow::UnsentCommitment,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkCommitment<InteractionElements> {
    pub traces: cairovm_verifier_air::trace::Commitment<InteractionElements>,
    pub composition: cairovm_verifier_commitment::table::types::Commitment,
    #[serde_as(as = "UfeHex")]
    pub interaction_after_composition: Felt,
    #[serde_as(as = "Vec<UfeHex>")]
    pub oods_values: Vec<Felt>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub interaction_after_oods: Vec<Felt>,
    pub fri: cairovm_verifier_fri::types::Commitment,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkWitness {
    pub traces_decommitment: cairovm_verifier_air::trace::Decommitment,
    pub traces_witness: cairovm_verifier_air::trace::Witness,
    pub composition_decommitment: cairovm_verifier_commitment::table::types::Decommitment,
    pub composition_witness: cairovm_verifier_commitment::table::types::Witness,
    pub fri_witness: cairovm_verifier_fri::types::Witness,
}
