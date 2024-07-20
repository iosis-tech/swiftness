use crate::{
    commit::stark_commit, queries::generate_queries, types::StarkProof, verify::stark_verify,
};
#[cfg(feature = "dex")]
use cairovm_verifier_air::layout::dex::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "recursive")]
use cairovm_verifier_air::layout::recursive::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "recursive_with_poseidon")]
use cairovm_verifier_air::layout::recursive_with_poseidon::{
    NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND,
};
#[cfg(feature = "small")]
use cairovm_verifier_air::layout::small::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "starknet")]
use cairovm_verifier_air::layout::starknet::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "starknet_with_keccak")]
use cairovm_verifier_air::layout::starknet_with_keccak::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};

impl StarkProof {
    pub fn verify<Layout: LayoutTrait>(&self, security_bits: Felt) -> Result<(Felt, Felt), Error> {
        self.config.validate(security_bits)?;

        // Validate the public input.
        let stark_domains =
            StarkDomains::new(self.config.log_trace_domain_size, self.config.log_n_cosets);

        Layout::validate_public_input(&self.public_input, &stark_domains)?;

        // Compute the initial hash seed for the Fiat-Shamir transcript.
        let digest = self.public_input.get_hash();
        // Construct the transcript.
        let mut transcript = Transcript::new(digest);

        // STARK commitment phase.
        let stark_commitment = stark_commit::<Layout>(
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
        stark_verify::<Layout>(
            NUM_COLUMNS_FIRST as usize,
            NUM_COLUMNS_SECOND as usize,
            &queries,
            stark_commitment,
            &self.witness,
            &stark_domains,
        )?;

        Ok(Layout::verify_public_input(&self.public_input)?)
    }
}

use cairovm_verifier_air::{
    domains::StarkDomains,
    layout::{LayoutTrait, PublicInputError},
};
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Validation(#[from] crate::config::Error),

    #[error("PublicInputError Error")]
    PublicInputError(#[from] PublicInputError),

    #[error("Commit Error")]
    Commit(#[from] crate::commit::Error),

    #[error("Verify Error")]
    Verify(#[from] crate::verify::Error),
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Validation(#[from] crate::config::Error),

    #[error("PublicInputError Error")]
    PublicInputError(#[from] PublicInputError),

    #[error("Commit Error")]
    Commit(#[from] crate::commit::Error),

    #[error("Verify Error")]
    Verify(#[from] crate::verify::Error),
}
