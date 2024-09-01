use super::global_values::GlobalValues;
use crate::{domains::StarkDomains, dynamic::DynamicParams, layout::LayoutTrait};
use starknet_crypto::Felt;

pub fn eval_composition_polynomial_inner(
    mask_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    trace_generator: &Felt,
    global_values: &GlobalValues,
) -> Felt {
    Felt::ZERO
}

pub fn eval_oods_polynomial_inner<Layout: LayoutTrait>(
    column_values: &[Felt],
    oods_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    oods_point: &Felt,
    trace_generator: &Felt,
) -> Felt {
    Felt::ZERO
}

pub fn check_asserts(
    _dynamic_params: &DynamicParams,
    _stark_domains: StarkDomains,
) -> Result<(), CheckAssertsError> {
    Ok(())
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum CheckAssertsError {
    #[error("value is not power of two")]
    ValueNotPowerOfTwo,

    #[error("value out of range")]
    ValueOutOfRange,
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum CheckAssertsError {
    #[error("value is not power of two")]
    ValueNotPowerOfTwo,

    #[error("value out of range")]
    ValueOutOfRange,
}
