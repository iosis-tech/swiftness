use num_bigint::{BigInt, TryFromBigIntError};
use starknet_types_core::felt::FeltIsZeroError;

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum CompositionPolyEvalError {
    #[error("segment not present {segment}")]
    SegmentMissing { segment: usize },

    #[error("value out of range")]
    ValueOutOfRange,

    #[error("dynamic params missing")]
    DynamicParamsMissing,

    #[error("field element is zero")]
    FeltIsZero(#[from] FeltIsZeroError),
}

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum OodsPolyEvalError {
    #[error("dynamic params missing")]
    DynamicParamsMissing,

    #[error("field element is zero")]
    FeltIsZero(#[from] FeltIsZeroError),
}

#[cfg(feature = "std")]
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

    #[error("invalid number of builtin copies")]
    CopiesInvalid,

    #[error("invalid number of segments")]
    InvalidSegments,

    #[error("dynamic params missing")]
    DynamicParamsMissing,

    #[error("BigInt conversion Error")]
    TryFromBigInt(#[from] TryFromBigIntError<BigInt>),

    #[error("field element is zero")]
    FeltIsZero(#[from] FeltIsZeroError),

    #[error("dynamic params check failed")]
    CheckAsserts(#[from] CheckAssertsError),
}

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum CheckAssertsError {
    #[error("value is not power of two")]
    NotPowerOfTwo,

    #[error("value out of range")]
    OutOfRange,

    #[error("value not boolean")]
    NotBoolean,

    #[error("field element is zero")]
    FeltIsZero(#[from] FeltIsZeroError),

    #[error("field multiplication fail")]
    SafeMult(#[from] SafeMultError),

    #[error("segment not present {segment}")]
    SegmentMissing { segment: usize },
}

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum SafeMultError {
    #[error("value multiplication overflowed actual {actual}, expected {expected}")]
    Overflow { actual: BigInt, expected: BigInt },
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum CompositionPolyEvalError {
    #[error("segment not present {segment}")]
    SegmentMissing { segment: usize },

    #[error("value out of range")]
    ValueOutOfRange,

    #[error("dynamic params missing")]
    DynamicParamsMissing,

    #[error("field element is zero")]
    FeltIsZero(#[from] FeltIsZeroError),
}

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum OodsPolyEvalError {
    #[error("dynamic params missing")]
    DynamicParamsMissing,

    #[error("field element is zero")]
    FeltIsZero(#[from] FeltIsZeroError),
}

#[cfg(not(feature = "std"))]
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

    #[error("invalid number of builtin copies")]
    CopiesInvalid,

    #[error("invalid number of segments")]
    InvalidSegments,

    #[error("dynamic params missing")]
    DynamicParamsMissing,

    #[error("BigInt conversion Error")]
    TryFromBigInt(#[from] TryFromBigIntError<BigInt>),

    #[error("field element is zero")]
    FeltIsZero(#[from] FeltIsZeroError),

    #[error("dynamic params check failed")]
    CheckAsserts(#[from] CheckAssertsError),
}

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum CheckAssertsError {
    #[error("value is not power of two")]
    NotPowerOfTwo,

    #[error("value out of range")]
    OutOfRange,

    #[error("value not boolean")]
    NotBoolean,

    #[error("field element is zero")]
    FeltIsZero(#[from] FeltIsZeroError),

    #[error("field multiplication fail")]
    SafeMult(#[from] SafeMultError),

    #[error("segment not present {segment}")]
    SegmentMissing { segment: usize },
}

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum SafeMultError {
    #[error("value multiplication overflowed actual {actual}, expected {expected}")]
    Overflow { actual: BigInt, expected: BigInt },
}
