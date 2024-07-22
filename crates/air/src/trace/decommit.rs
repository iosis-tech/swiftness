use swiftness_commitment::table;

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Table Error")]
    Table(#[from] table::decommit::Error),
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[derive(Error, Debug)]
#[cfg(not(feature = "std"))]
pub enum Error {
    #[error("Table Error")]
    Table(#[from] table::decommit::Error),
}
