pub mod autogenerated_composition;
pub mod autogenerated_oods;

pub use autogenerated_composition::eval_composition_polynomial_inner;
pub use autogenerated_oods::eval_oods_polynomial_inner;

use crate::{domains::StarkDomains, dynamic::DynamicParams};

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