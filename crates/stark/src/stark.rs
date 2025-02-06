use crate::{
    commit::stark_commit,
    queries::generate_queries,
    types::{Cache, StarkProof, StarkWitness},
    verify::stark_verify,
};
use alloc::boxed::Box;
use alloc::vec::Vec;
use starknet_crypto::Felt;
use swiftness_air::{
    domains::StarkDomains,
    layout::{GenericLayoutTrait, LayoutTrait, PublicInputError},
    public_memory::PublicInput,
};
pub use swiftness_commitment::CacheCommitment;
use swiftness_transcript::transcript::Transcript;

impl StarkProof {
    pub fn verify<Layout: GenericLayoutTrait + LayoutTrait>(
        &self,
        cache: &mut Cache,
        security_bits: Felt,
    ) -> Result<(Felt, Vec<Felt>), Error> {
        let n_original_columns =
            Layout::get_num_columns_first(&self.public_input).ok_or(Error::ColumnMissing)?;
        let n_interaction_columns =
            Layout::get_num_columns_second(&self.public_input).ok_or(Error::ColumnMissing)?;

        self.config.validate(
            security_bits,
            n_original_columns.into(),
            n_interaction_columns.into(),
        )?;

        // Validate the public input.
        let stark_domains = Box::new(StarkDomains::new(
            self.config.log_trace_domain_size,
            self.config.log_n_cosets,
        ));

        Layout::validate_public_input(&self.public_input, &stark_domains)?;

        // Compute the initial hash seed for the Fiat-Shamir transcript.
        let digest = self.public_input.get_hash(self.config.n_verifier_friendly_commitment_layers);
        // Construct the transcript.
        let mut transcript = Box::new(Transcript::new(digest));

        // // STARK commitment phase.
        let stark_commitment = Box::new(stark_commit::<Layout>(
            &mut transcript,
            &self.public_input,
            &self.unsent_commitment,
            &self.config,
            &stark_domains,
        )?);

        // Generate queries.
        let queries = generate_queries(
            &mut transcript,
            self.config.n_queries,
            stark_domains.eval_domain_size,
        );

        // STARK verify phase.
        stark_verify::<Layout>(
            &mut cache.commitment,
            n_original_columns,
            n_interaction_columns,
            &self.public_input,
            &queries,
            &*stark_commitment,
            &self.witness,
            &stark_domains,
        )?;

        Ok(Layout::verify_public_input(&self.public_input)?)
    }
}

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

    #[error("Column missing")]
    ColumnMissing,
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

    #[error("Column missing")]
    ColumnMissing,
}
