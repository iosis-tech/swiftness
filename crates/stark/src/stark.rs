use crate::{
    commit::stark_commit, queries::generate_queries, types::StarkProof, verify::stark_verify,
};

impl StarkProof {
    pub fn verify(&self, security_bits: Felt) -> Result<Felt, Error> {
        self.config.validate(security_bits)?;

        // Validate the public input.
        let stark_domains =
            StarkDomains::new(self.config.log_trace_domain_size, self.config.log_n_cosets);
        self.public_input.validate(&stark_domains);

        // Compute the initial hash seed for the Fiat-Shamir channel.
        let digest = self.public_input.get_public_input_hash();
        // Construct the channel.
        let mut transcript = Transcript::new(digest);

        // STARK commitment phase.
        let stark_commitment = stark_commit(
            &mut transcript,
            &self.public_input,
            &self.unsent_commitment,
            &self.config,
            &stark_domains,
        )?;

        // Generate queries.
        let queries = generate_queries(
            &mut transcript,
            self.config.n_queries,
            stark_domains.eval_domain_size,
        );

        // STARK verify phase.
        stark_verify(
            NUM_COLUMNS_FIRST as usize,
            NUM_COLUMNS_SECOND as usize,
            &queries,
            stark_commitment,
            &self.witness,
            &stark_domains,
        )?;

        Ok(digest)
    }
}

use cairovm_verifier_air::{
    domains::StarkDomains,
    layout::recursive::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND},
};
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Validation(#[from] crate::config::Error),

    #[error("Commit Error")]
    Commit(#[from] crate::commit::Error),

    #[error("Verify Error")]
    Verify(#[from] crate::verify::Error),
}
