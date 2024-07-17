use num_bigint::BigUint;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct StarkProof {
    pub config: StarkConfig,
    pub public_input: CairoPublicInput,
    pub unsent_commitment: StarkUnsentCommitment,
    pub witness: StarkWitness,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StarkConfig {
    pub traces: TracesConfig,
    pub composition: TableCommitmentConfig,
    pub fri: FriConfig,
    pub proof_of_work: ProofOfWorkConfig,
    pub log_trace_domain_size: u32,
    pub n_queries: u32,
    pub log_n_cosets: u32,
    pub n_verifier_friendly_commitment_layers: u32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct TracesConfig {
    pub original: TableCommitmentConfig,
    pub interaction: TableCommitmentConfig,
}
#[derive(Debug, Clone, PartialEq)]
pub struct TableCommitmentConfig {
    pub n_columns: u32,
    pub vector: VectorCommitmentConfig,
}
#[derive(Debug, Clone, PartialEq)]
pub struct VectorCommitmentConfig {
    pub height: u32,
    pub n_verifier_friendly_commitment_layers: u32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FriConfig {
    pub log_input_size: u32,
    pub n_layers: u32,
    pub inner_layers: Vec<TableCommitmentConfig>,
    pub fri_step_sizes: Vec<u32>,
    pub log_last_layer_degree_bound: u32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProofOfWorkConfig {
    pub n_bits: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StarkUnsentCommitment {
    pub traces: TracesUnsentCommitment,
    pub composition: BigUint,
    pub oods_values: Vec<BigUint>,
    pub fri: FriUnsentCommitment,
    pub proof_of_work: ProofOfWorkUnsentCommitment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TracesUnsentCommitment {
    pub original: BigUint,
    pub interaction: BigUint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FriUnsentCommitment {
    pub inner_layers: Vec<BigUint>,
    pub last_layer_coefficients: Vec<BigUint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProofOfWorkUnsentCommitment {
    pub nonce: BigUint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StarkWitness {
    pub traces_decommitment: TracesDecommitment,
    pub traces_witness: TracesWitness,
    pub composition_decommitment: TableDecommitment,
    pub composition_witness: TableCommitmentWitness,
    pub fri_witness: FriWitness,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TracesDecommitment {
    pub original: TableDecommitment,
    pub interaction: TableDecommitment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableDecommitment {
    pub n_values: usize,
    pub values: Vec<BigUint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TracesWitness {
    pub original: TableCommitmentWitness,
    pub interaction: TableCommitmentWitness,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCommitmentWitness {
    pub vector: VectorCommitmentWitness,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VectorCommitmentWitness {
    pub n_authentications: usize,
    pub authentications: Vec<BigUint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCommitmentWitnessFlat {
    pub vector: VectorCommitmentWitnessFlat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VectorCommitmentWitnessFlat {
    pub n_authentications: usize,
    pub authentications: Vec<BigUint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FriWitness {
    pub layers: Vec<FriLayerWitness>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FriLayerWitness {
    pub n_leaves: usize,
    pub leaves: Vec<BigUint>,
    pub table_witness: TableCommitmentWitnessFlat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CairoPublicInput {
    pub log_n_steps: u32,
    pub range_check_min: u32,
    pub range_check_max: u32,
    pub layout: BigUint,
    pub dynamic_params: BTreeMap<String, BigUint>,
    pub n_segments: usize,
    pub segments: Vec<SegmentInfo>,
    pub padding_addr: u32,
    pub padding_value: BigUint,
    pub main_page_len: usize,
    pub main_page: Vec<PubilcMemoryCell>,
    pub n_continuous_pages: usize,
    pub continuous_page_headers: Vec<BigUint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PubilcMemoryCell {
    pub address: u32,
    pub value: BigUint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SegmentInfo {
    pub begin_addr: u32,
    pub stop_ptr: u32,
}
