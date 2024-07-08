use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StarkProof {
    config: StarkConfig,
    public_input: cairovm_verifier_air::types::PublicInput,
    unsent_commitment: StarkUnsentCommitment,
    witness: StarkWitness,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StarkConfig {
    traces: cairovm_verifier_air::trace::config::Config,
    composition: cairovm_verifier_commitment::table::config::Config,
    fri: cairovm_verifier_fri::config::Config,
    proof_of_work: cairovm_verifier_fri::config::Config,
    // Log2 of the trace domain size.
    #[serde_as(as = "UfeHex")]
    log_trace_domain_size: Felt,
    // Number of queries to the last component, FRI.
    #[serde_as(as = "UfeHex")]
    n_queries: Felt,
    // Log2 of the number of cosets composing the evaluation domain, where the coset size is the
    // trace length.
    #[serde_as(as = "UfeHex")]
    log_n_cosets: Felt,
    // Number of layers that use a verifier friendly hash in each commitment.
    #[serde_as(as = "UfeHex")]
    n_verifier_friendly_commitment_layers: Felt,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StarkUnsentCommitment {
    traces: cairovm_verifier_air::trace::UnsentCommitment,
    #[serde_as(as = "UfeHex")]
    composition: Felt,
    // n_oods_values elements. The i-th value is the evaluation of the i-th mask item polynomial at
    // the OODS point, where the mask item polynomial is the interpolation polynomial of the
    // corresponding column shifted by the corresponding row_offset.
    #[serde_as(as = "Vec<UfeHex>")]
    oods_values: Vec<Felt>,
    fri: cairovm_verifier_fri::types::UnsentCommitment,
    proof_of_work: cairovm_verifier_pow::pow::UnsentCommitment,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StarkCommitment {
    traces: cairovm_verifier_air::trace::Commitment,
    composition: cairovm_verifier_commitment::table::types::Commitment,
    #[serde_as(as = "UfeHex")]
    interaction_after_composition: Felt,
    #[serde_as(as = "Vec<UfeHex>")]
    oods_values: Vec<Felt>,
    #[serde_as(as = "Vec<UfeHex>")]
    interaction_after_oods: Vec<Felt>,
    fri: cairovm_verifier_fri::types::Commitment,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StarkWitness {
    traces_decommitment: cairovm_verifier_air::trace::Decommitment,
    traces_witness: cairovm_verifier_air::trace::Witness,
    composition_decommitment: cairovm_verifier_commitment::table::types::Decommitment,
    composition_witness: cairovm_verifier_commitment::table::types::Witness,
    fri_witness: cairovm_verifier_fri::types::Witness,
}
