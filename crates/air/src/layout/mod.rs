use crate::{
    consts::{FELT_0, FELT_3},
    domains::StarkDomains,
    public_memory::PublicInput,
    types::AddrValue,
};
use alloc::vec::Vec;
use num_bigint::{BigInt, TryFromBigIntError};
use starknet_core::types::NonZeroFelt;
use starknet_crypto::{pedersen_hash, Felt};
use starknet_types_core::felt::FeltIsZeroError;
use swiftness_commitment::CacheCommitment;
use swiftness_transcript::transcript::Transcript;

mod errors;
pub use errors::*;

// #[cfg(feature = "dex")]
// pub mod dex;
// #[cfg(feature = "dynamic")]
// pub mod dynamic;
#[cfg(feature = "recursive")]
pub mod recursive;
// #[cfg(feature = "recursive_with_poseidon")]
// pub mod recursive_with_poseidon;
// #[cfg(feature = "small")]
// pub mod small;
// #[cfg(feature = "starknet")]
// pub mod starknet;
// #[cfg(feature = "starknet_with_keccak")]
// pub mod starknet_with_keccak;

// StarkCurve
pub mod stark_curve {
    use starknet_crypto::Felt;

    pub const ALPHA: Felt = Felt::from_hex_unchecked("0x1");
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

    const CONSTRAINT_DEGREE: usize;
    const N_CONSTRAINTS: usize;
    const MASK_SIZE: usize;

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
        public_input: &PublicInput,
        column_values: &[Felt],
        oods_values: &[Felt],
        constraint_coefficients: &[Felt],
        point: &Felt,
        oods_point: &Felt,
        trace_generator: &Felt,
    ) -> Result<Felt, OodsPolyEvalError>;

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
        cache: &mut CacheCommitment,
        queries: &[Felt],
        commitment: &crate::trace::Commitment<Self::InteractionElements>,
        decommitment: &crate::trace::Decommitment,
        witness: &crate::trace::Witness,
    ) -> Result<(), crate::trace::decommit::Error>;

    fn verify_public_input(
        public_input: &PublicInput,
    ) -> Result<(Felt, Vec<Felt>), PublicInputError>;
}

pub trait StaticLayoutTrait {
    const NUM_COLUMNS_FIRST: u32;
    const NUM_COLUMNS_SECOND: u32;
}

pub trait GenericLayoutTrait {
    fn get_num_columns_first(public_input: &PublicInput) -> Option<u32>;
    fn get_num_columns_second(public_input: &PublicInput) -> Option<u32>;
}

pub fn safe_div(value: Felt, divisor: Felt) -> Result<Felt, FeltIsZeroError> {
    Ok(value.floor_div(&NonZeroFelt::try_from(divisor)?))
}

pub fn safe_mult(value: Felt, multiplier: Felt) -> Result<Felt, SafeMultError> {
    let mul = value.to_bigint() * multiplier.to_bigint();
    let felt_mul = value * multiplier;
    match felt_mul.to_bigint().cmp(&mul) {
        core::cmp::Ordering::Equal => Ok(felt_mul),
        _ => Err(SafeMultError::Overflow { actual: felt_mul.to_bigint(), expected: mul }),
    }
}

pub fn compute_program_hash(
    memory: &[AddrValue],
    initial_pc: Felt,
    initial_fp: Felt,
) -> Result<Felt, TryFromBigIntError<BigInt>> {
    let initial_pc: usize = initial_pc.to_bigint().try_into()?;
    let initial_fp: usize = (initial_fp - FELT_3).to_bigint().try_into()?;

    let mut program_hash = FELT_0;
    for m in (initial_pc / 2)..initial_fp {
        program_hash = pedersen_hash(&program_hash, &memory[m].value);
    }

    let program_hash = pedersen_hash(&program_hash, &Felt::from(initial_fp));

    Ok(program_hash)
}
