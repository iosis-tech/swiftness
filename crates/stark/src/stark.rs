use crate::types::StarkProof;

impl StarkProof {
    pub fn verify(&self, security_bits: Felt) -> Result<(), Error> {
        self.config.validate(security_bits)?;

        todo!()
    }
}

use starknet_crypto::Felt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Validation(#[from] crate::config::Error),
}
