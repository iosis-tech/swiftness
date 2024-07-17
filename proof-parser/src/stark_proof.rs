use std::collections::BTreeMap;

use num_bigint::BigUint;

use crate::ast::{Expr, Exprs};

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

pub trait IntoAst {
    fn into_ast(self) -> Vec<Expr>;
}

impl IntoAst for u32 {
    fn into_ast(self) -> Vec<Expr> {
        vec![Expr::Value(format!("{self}"))]
    }
}

impl IntoAst for usize {
    fn into_ast(self) -> Vec<Expr> {
        vec![Expr::Value(format!("{self}"))]
    }
}

impl IntoAst for BigUint {
    fn into_ast(self) -> Vec<Expr> {
        vec![Expr::Value(format!("{self}"))]
    }
}

impl IntoAst for &BigUint {
    fn into_ast(self) -> Vec<Expr> {
        vec![Expr::Value(format!("{self}"))]
    }
}

impl IntoAst for StarkProof {
    fn into_ast(self) -> Vec<Expr> {
        [
            self.config.into_ast(),
            self.public_input.into_ast(),
            self.unsent_commitment.into_ast(),
            self.witness.into_ast(),
        ]
        .concat()
    }
}

impl IntoAst for StarkConfig {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.traces.into_ast());
        exprs.append(&mut self.composition.into_ast());
        exprs.append(&mut self.fri.into_ast());
        exprs.append(&mut self.proof_of_work.into_ast());
        exprs.append(&mut self.log_trace_domain_size.into_ast());
        exprs.append(&mut self.n_queries.into_ast());
        exprs.append(&mut self.log_n_cosets.into_ast());
        exprs.append(&mut self.n_verifier_friendly_commitment_layers.into_ast());
        exprs
    }
}

impl IntoAst for TracesConfig {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.original.into_ast());
        exprs.append(&mut self.interaction.into_ast());
        exprs
    }
}

impl IntoAst for TableCommitmentConfig {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.n_columns.into_ast());
        exprs.append(&mut self.vector.into_ast());
        exprs
    }
}

impl IntoAst for VectorCommitmentConfig {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.height.into_ast());
        exprs.append(&mut self.n_verifier_friendly_commitment_layers.into_ast());
        exprs
    }
}

impl IntoAst for FriConfig {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.log_input_size.into_ast());
        exprs.append(&mut self.n_layers.into_ast());
        exprs.append(&mut self.inner_layers.into_ast());
        exprs.append(&mut self.fri_step_sizes.into_ast());
        exprs.append(&mut self.log_last_layer_degree_bound.into_ast());
        exprs
    }
}

impl IntoAst for StarkUnsentCommitment {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.traces.into_ast());
        exprs.append(&mut self.composition.into_ast());
        exprs.append(&mut self.oods_values.into_ast());
        exprs.append(&mut self.fri.into_ast());
        exprs.append(&mut self.proof_of_work.into_ast());
        exprs
    }
}

impl IntoAst for TracesUnsentCommitment {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.original.into_ast());
        exprs.append(&mut self.interaction.into_ast());
        exprs
    }
}

impl IntoAst for FriUnsentCommitment {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.inner_layers.into_ast());
        exprs.append(&mut self.last_layer_coefficients.into_ast());
        exprs
    }
}

impl IntoAst for ProofOfWorkUnsentCommitment {
    fn into_ast(self) -> Vec<Expr> {
        vec![Expr::Value(format!("{}", self.nonce))]
    }
}

impl IntoAst for ProofOfWorkConfig {
    fn into_ast(self) -> Vec<Expr> {
        vec![Expr::Value(format!("{}", self.n_bits))]
    }
}

impl IntoAst for StarkWitness {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.traces_decommitment.into_ast());
        exprs.append(&mut self.traces_witness.into_ast());
        exprs.append(&mut self.composition_decommitment.into_ast());
        exprs.append(&mut self.composition_witness.into_ast());
        exprs.append(&mut self.fri_witness.into_ast());
        exprs
    }
}

impl IntoAst for TracesDecommitment {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.original.into_ast());
        exprs.append(&mut self.interaction.into_ast());
        exprs
    }
}

impl IntoAst for TableDecommitment {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![Expr::Value(format!("{}", self.n_values))];
        exprs.append(&mut self.values.into_ast());
        exprs
    }
}

impl IntoAst for TracesWitness {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.original.into_ast());
        exprs.append(&mut self.interaction.into_ast());
        exprs
    }
}

impl IntoAst for TableCommitmentWitness {
    fn into_ast(self) -> Vec<Expr> {
        self.vector.into_ast()
    }
}

impl IntoAst for TableCommitmentWitnessFlat {
    fn into_ast(self) -> Vec<Expr> {
        self.vector.into_ast()
    }
}

impl IntoAst for VectorCommitmentWitness {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![Expr::Value(format!("{}", self.n_authentications))];
        exprs.append(&mut self.authentications.into_ast());
        exprs
    }
}

impl IntoAst for VectorCommitmentWitnessFlat {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![Expr::Value(format!("{}", self.n_authentications))];
        exprs.append(
            &mut self.authentications.iter().flat_map(|x| x.into_ast()).collect::<Vec<_>>(),
        );
        exprs
    }
}

impl IntoAst for FriWitness {
    fn into_ast(self) -> Vec<Expr> {
        self.layers.into_ast()
    }
}

impl IntoAst for FriLayerWitness {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![Expr::Value(format!("{}", self.n_leaves))];
        exprs.append(&mut self.leaves.iter().flat_map(|x| x.into_ast()).collect::<Vec<_>>());
        exprs.append(&mut self.table_witness.into_ast());
        exprs
    }
}

impl IntoAst for CairoPublicInput {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.log_n_steps.into_ast());
        exprs.append(&mut self.range_check_min.into_ast());
        exprs.append(&mut self.range_check_max.into_ast());
        exprs.append(&mut self.layout.into_ast());
        exprs.push(Expr::Array(
            self.dynamic_params.values().map(|v| Expr::Value(format!("{v}"))).collect(),
        ));
        exprs.append(&mut self.n_segments.into_ast());
        exprs.append(&mut self.segments.into_ast());
        exprs.append(&mut self.padding_addr.into_ast());
        exprs.append(&mut self.padding_value.into_ast());
        exprs.append(&mut self.main_page_len.into_ast());
        exprs.append(&mut self.main_page.into_ast());
        exprs.append(&mut self.n_continuous_pages.into_ast());
        exprs.append(&mut self.continuous_page_headers.into_ast());
        exprs
    }
}

impl IntoAst for SegmentInfo {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.begin_addr.into_ast());
        exprs.append(&mut self.stop_ptr.into_ast());
        exprs
    }
}

impl IntoAst for PubilcMemoryCell {
    fn into_ast(self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.append(&mut self.address.into_ast());
        exprs.append(&mut self.value.into_ast());
        exprs
    }
}

impl<T> IntoAst for Vec<T>
where
    T: IntoAst,
{
    fn into_ast(self) -> Vec<Expr> {
        vec![Expr::Array(self.into_iter().flat_map(|x| x.into_ast()).collect())]
    }
}

impl From<StarkConfig> for Exprs {
    fn from(v: StarkConfig) -> Self {
        Exprs(v.into_ast())
    }
}

impl From<CairoPublicInput> for Exprs {
    fn from(v: CairoPublicInput) -> Self {
        Exprs(v.into_ast())
    }
}

impl From<StarkUnsentCommitment> for Exprs {
    fn from(v: StarkUnsentCommitment) -> Self {
        Exprs(v.into_ast())
    }
}

impl From<StarkWitness> for Exprs {
    fn from(v: StarkWitness) -> Self {
        Exprs(v.into_ast())
    }
}

impl From<StarkProof> for Exprs {
    fn from(proof: StarkProof) -> Self {
        Exprs(proof.into_ast())
    }
}
