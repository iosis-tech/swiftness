use crate::{domains::StarkDomains, public_memory::PublicInput};
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

pub mod recursive;

pub trait LayoutTrait {
    type InteractionElements;
    fn eval_composition_polynomial(
        interaction_elements: &Self::InteractionElements,
        public_input: &PublicInput,
        mask_values: &[Felt],
        constraint_coefficients: &[Felt],
        point: &Felt,
        trace_domain_size: &Felt,
        trace_generator: &Felt,
    ) -> Result<Felt, CompositionPolyEvalError>;

    fn eval_oods_polynomial(
        column_values: &[Felt],
        oods_values: &[Felt],
        constraint_coefficients: &[Felt],
        point: &Felt,
        oods_point: &Felt,
        trace_generator: &Felt,
    ) -> Felt;

    fn validate(
        public_input: &PublicInput,
        stark_domains: &StarkDomains,
    ) -> Result<(), PublicInputValidateError>;

    fn traces_commit(
        transcript: &mut Transcript,
        unsent_commitment: &crate::trace::UnsentCommitment,
        config: crate::trace::config::Config,
    ) -> crate::trace::Commitment<Self::InteractionElements>;

    fn traces_decommit(
        queries: &[Felt],
        commitment: crate::trace::Commitment<Self::InteractionElements>,
        decommitment: crate::trace::Decommitment,
        witness: crate::trace::Witness,
    ) -> Result<(), crate::trace::decommit::Error>;
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompositionPolyEvalError {
    #[error("segment not present {segment}")]
    SegmentMissing { segment: usize },
}

#[derive(Error, Debug)]
pub enum PublicInputValidateError {
    #[error("max steps exceeded")]
    MaxSteps,

    #[error("trace length invalid")]
    TraceLengthInvalid,

    #[error("segment not present {segment}")]
    SegmentMissing { segment: usize },

    #[error("layout code invalid")]
    LayoutCodeInvalid,

    #[error("range_check invalid")]
    RangeCheckInvalid,

    #[error("invalid number of builtin uses")]
    UsesInvalid,
}
