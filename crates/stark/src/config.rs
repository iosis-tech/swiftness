use crate::alloc::borrow::ToOwned;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_crypto::Felt;
use swiftness_air::{
    layout::{DynamicLayoutTrait, StaticLayoutTrait},
    public_memory::PublicInput,
};
use swiftness_commitment::vector;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkConfig {
    pub traces: swiftness_air::trace::config::Config,
    pub composition: swiftness_commitment::table::config::Config,
    pub fri: swiftness_fri::config::Config,
    pub proof_of_work: swiftness_pow::config::Config,
    // Log2 of the trace domain size.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub log_trace_domain_size: Felt,
    // Number of queries to the last component, FRI.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub n_queries: Felt,
    // Log2 of the number of cosets composing the evaluation domain, where the coset size is the
    // trace length.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub log_n_cosets: Felt,
    // Number of layers that use a verifier friendly hash in each commitment.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub n_verifier_friendly_commitment_layers: Felt,
}

impl StarkConfig {
    pub fn security_bits(&self) -> Felt {
        self.n_queries * self.log_n_cosets + Felt::from(self.proof_of_work.n_bits)
    }

    pub fn validate_static_layout<Layout: StaticLayoutTrait>(
        &self,
        security_bits: Felt,
    ) -> Result<(), Error> {
        self.proof_of_work.validate()?;

        ensure!(security_bits <= self.security_bits(), Error::InsufficientSecurity);

        // Validate traces config.
        let log_eval_domain_size = self.log_trace_domain_size + self.log_n_cosets;
        self.traces.validate(
            log_eval_domain_size,
            self.n_verifier_friendly_commitment_layers,
            Layout::NUM_COLUMNS_FIRST.into(),
            Layout::NUM_COLUMNS_SECOND.into(),
        )?;

        // Validate composition config.
        self.composition
            .vector
            .validate(log_eval_domain_size, self.n_verifier_friendly_commitment_layers)?;

        // Validate Fri config.
        self.fri.validate(self.log_n_cosets, self.n_verifier_friendly_commitment_layers)?;
        Ok(())
    }

    pub fn validate_dynamic_layout<Layout: DynamicLayoutTrait>(
        &self,
        security_bits: Felt,
        public_input: &PublicInput,
    ) -> Result<(), Error> {
        self.proof_of_work.validate()?;

        ensure!(security_bits <= self.security_bits(), Error::InsufficientSecurity);

        let dynamc_params =
            public_input.dynamic_params.to_owned().ok_or(Error::DynamicParamsMissing)?;

        // Validate traces config.
        let log_eval_domain_size = self.log_trace_domain_size + self.log_n_cosets;
        self.traces.validate(
            log_eval_domain_size,
            self.n_verifier_friendly_commitment_layers,
            dynamc_params.num_columns_first.into(),
            dynamc_params.num_columns_second.into(),
        )?;

        // Validate composition config.
        self.composition
            .vector
            .validate(log_eval_domain_size, self.n_verifier_friendly_commitment_layers)?;

        // Validate Fri config.
        self.fri.validate(self.log_n_cosets, self.n_verifier_friendly_commitment_layers)?;
        Ok(())
    }
}

use swiftness_transcript::ensure;
#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Vector(#[from] vector::config::Error),
    #[error("Fri Error")]
    Fri(#[from] swiftness_fri::config::Error),
    #[error("Pow Error")]
    Pow(#[from] swiftness_pow::config::Error),
    #[error("Trace Error")]
    Trace(#[from] swiftness_air::trace::config::Error),
    #[error("dynamic params missing")]
    DynamicParamsMissing,
    #[error("insufficient number ofsecurity bits")]
    InsufficientSecurity,
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Vector(#[from] vector::config::Error),
    #[error("Fri Error")]
    Fri(#[from] swiftness_fri::config::Error),
    #[error("Pow Error")]
    Pow(#[from] swiftness_pow::config::Error),
    #[error("Trace Error")]
    Trace(#[from] swiftness_air::trace::config::Error),
    #[error("dynamic params missing")]
    DynamicParamsMissing,
    #[error("insufficient number ofsecurity bits")]
    InsufficientSecurity,
}
