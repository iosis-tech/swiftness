use crate::{domains::StarkDomains, public_memory::PublicInput};
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

pub mod dex;
pub mod recursive;
pub mod recursive_with_poseidon;

// StarkCurve
pub mod stark_curve {
    use starknet_crypto::Felt;

    pub const ALPHA: Felt = Felt::from_hex_unchecked("1");
    pub const BETA: Felt = Felt::from_hex_unchecked(
        "0x6f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89",
    );
    pub const ORDER: Felt = Felt::from_hex_unchecked(
        "0x800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f",
    );
    pub const GEN_X: Felt = Felt::from_hex_unchecked(
        "0x1ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca",
    );
    pub const GEN_Y: Felt = Felt::from_hex_unchecked(
        "0x5668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f",
    );
}

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

    fn validate_public_input(
        public_input: &PublicInput,
        stark_domains: &StarkDomains,
    ) -> Result<(), PublicInputError>;

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

    fn verify_public_input(public_input: &PublicInput) -> Result<(Felt, Felt), PublicInputError>;
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompositionPolyEvalError {
    #[error("segment not present {segment}")]
    SegmentMissing { segment: usize },
}

#[derive(Error, Debug)]
pub enum PublicInputError {
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

pub mod segments {
    pub const EXECUTION: usize = 1;
    pub const OUTPUT: usize = 2;
    pub const PROGRAM: usize = 0;
}
