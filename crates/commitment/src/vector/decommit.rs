use starknet_crypto::Felt;
use thiserror::Error;

pub fn vector_commitment_decommit() -> Result<(), Error> {
    todo!()
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("mismatch value {value} expected {expected}")]
    MisMatch { value: Felt, expected: Felt },
}
