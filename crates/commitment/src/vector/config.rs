use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde_as(as = "UfeHex")]
    pub height: Felt,
    #[serde_as(as = "UfeHex")]
    pub n_verifier_friendly_commitment_layers: Felt,
}

impl Config {
    pub fn validate(
        &self,
        expected_height: Felt,
        expected_n_verifier_friendly_commitment_layers: Felt,
    ) -> Result<(), Error> {
        if self.height != expected_height {
            return Err(Error::MisMatch { value: self.height, expected: expected_height });
        }
        if self.n_verifier_friendly_commitment_layers
            != expected_n_verifier_friendly_commitment_layers
        {
            return Err(Error::MisMatch {
                value: self.n_verifier_friendly_commitment_layers,
                expected: expected_n_verifier_friendly_commitment_layers,
            });
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("mismatch value {value} expected {expected}")]
    MisMatch { value: Felt, expected: Felt },
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("mismatch value {value} expected {expected}")]
    MisMatch { value: Felt, expected: Felt },
}
