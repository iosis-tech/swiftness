use cairo_felt::Felt252;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StarkProof {
    config: StarkConfig,
    public_input: cairovm_verifier_air::types::PublicInput,
    unsent_commitment: StarkUnsentCommitment,
    witness: StarkWitness,
}

#[derive(Debug, Serialize, Deserialize)]
struct StarkConfig {
    traces: cairovm_verifier_air::types::trace::Config,
    composition: cairovm_verifier_commitment::table::types::Config,
    fri: cairovm_verifier_fri::types::Config,
    proof_of_work: cairovm_verifier_fri::types::Config,
    // Log2 of the trace domain size.
    log_trace_domain_size: Felt252,
    // Number of queries to the last component, FRI.
    n_queries: Felt252,
    // Log2 of the number of cosets composing the evaluation domain, where the coset size is the
    // trace length.
    log_n_cosets: Felt252,
    // Number of layers that use a verifier friendly hash in each commitment.
    n_verifier_friendly_commitment_layers: Felt252,
}

#[derive(Debug, Serialize, Deserialize)]
struct StarkUnsentCommitment {
    traces: cairovm_verifier_air::types::trace::UnsentCommitment,
    composition: Felt252,
    // n_oods_values elements. The i-th value is the evaluation of the i-th mask item polynomial at
    // the OODS point, where the mask item polynomial is the interpolation polynomial of the
    // corresponding column shifted by the corresponding row_offset.
    oods_values: Vec<Felt252>,
    fri: cairovm_verifier_fri::types::UnsentCommitment,
    proof_of_work: cairovm_verifier_pow::types::UnsentCommitment,
}

#[derive(Debug, Serialize, Deserialize)]
struct StarkCommitment {
    traces: cairovm_verifier_air::types::trace::Commitment,
    composition: cairovm_verifier_commitment::table::types::Commitment,
    interaction_after_composition: Felt252,
    oods_values: Vec<Felt252>,
    interaction_after_oods: Vec<Felt252>,
    fri: cairovm_verifier_fri::types::Commitment,
}

#[derive(Debug, Serialize, Deserialize)]
struct StarkWitness {
    traces_decommitment: cairovm_verifier_air::types::trace::Decommitment,
    traces_witness: cairovm_verifier_air::types::trace::Witness,
    composition_decommitment: cairovm_verifier_commitment::table::types::Decommitment,
    composition_witness: cairovm_verifier_commitment::table::types::Witness,
    fri_witness: cairovm_verifier_fri::types::Witness,
}
