use cairovm_verifier_commitment::table;
use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Table Error")]
    Table(#[from] table::decommit::Error),
}
