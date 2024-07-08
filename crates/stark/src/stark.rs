use crate::types::StarkProof;

impl StarkProof {
    pub fn verify(&self, security_bits: Felt) -> Result<(), Error> {
        self.config.validate(security_bits)?;

        // Validate the public input.
        let stark_domains =
            StarkDomains::new(self.config.log_trace_domain_size, self.config.log_n_cosets);
        self.public_input.validate(stark_domains);

        todo!()
    }
}

use cairovm_verifier_air::domains::StarkDomains;
use starknet_crypto::Felt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Validation(#[from] crate::config::Error),
}
