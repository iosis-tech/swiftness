use serde::{Deserialize, Serialize};
use starknet_crypto::Felt;
use swiftness_commitment::vector;

const MAX_N_COLUMNS: Felt = Felt::from_hex_unchecked("0x80");

// Configuration for the Traces component.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub original: swiftness_commitment::table::config::Config,
    pub interaction: swiftness_commitment::table::config::Config,
}

impl Config {
    pub fn validate(
        &self,
        log_eval_domain_size: Felt,
        n_verifier_friendly_commitment_layers: Felt,
        n_columns_original: Felt,
        n_columns_interaction: Felt,
    ) -> Result<(), Error> {
        if self.original.n_columns < Felt::ONE || self.original.n_columns > MAX_N_COLUMNS {
            return Err(Error::OutOfBounds { min: Felt::ONE, max: MAX_N_COLUMNS });
        }
        if self.interaction.n_columns < Felt::ONE || self.interaction.n_columns > MAX_N_COLUMNS {
            return Err(Error::OutOfBounds { min: Felt::ONE, max: MAX_N_COLUMNS });
        }

        if self.original.n_columns != n_columns_original {
            return Err(Error::ColumnsNumInvalid);
        }

        if self.interaction.n_columns != n_columns_interaction {
            return Err(Error::ColumnsNumInvalid);
        }

        Ok(self
            .original
            .vector
            .validate(log_eval_domain_size, n_verifier_friendly_commitment_layers)
            .and(
                self.interaction
                    .vector
                    .validate(log_eval_domain_size, n_verifier_friendly_commitment_layers),
            )?)
    }
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("value out of bounds {min} - {max}")]
    OutOfBounds { min: Felt, max: Felt },

    #[error("wrong numbers of columns")]
    ColumnsNumInvalid,

    #[error("Vector Error")]
    Vector(#[from] vector::config::Error),
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("value out of bounds {min} - {max}")]
    OutOfBounds { min: Felt, max: Felt },

    #[error("wrong numbers of columns")]
    ColumnsNumInvalid,

    #[error("Vector Error")]
    Vector(#[from] vector::config::Error),
}
