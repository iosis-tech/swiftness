#[cfg(feature = "dex")]
use crate::layout::dex::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "recursive")]
use crate::layout::recursive::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "recursive_with_poseidon")]
use crate::layout::recursive_with_poseidon::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "small")]
use crate::layout::small::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "starknet")]
use crate::layout::starknet::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
#[cfg(feature = "starknet_with_keccak")]
use crate::layout::starknet_with_keccak::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};

use serde::{Deserialize, Serialize};
use starknet_crypto::Felt;
use swiftness_commitment::vector;

const MAX_N_COLUMNS: Felt = Felt::from_hex_unchecked("0x80");

// Configuration for the Traces component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub original: swiftness_commitment::table::config::Config,
    pub interaction: swiftness_commitment::table::config::Config,
}

impl Config {
    pub fn validate(
        &self,
        log_eval_domain_size: Felt,
        n_verifier_friendly_commitment_layers: Felt,
    ) -> Result<(), Error> {
        if self.original.n_columns < Felt::ONE || self.original.n_columns > MAX_N_COLUMNS {
            return Err(Error::OutOfBounds { min: Felt::ONE, max: MAX_N_COLUMNS });
        }
        if self.interaction.n_columns < Felt::ONE || self.interaction.n_columns > MAX_N_COLUMNS {
            return Err(Error::OutOfBounds { min: Felt::ONE, max: MAX_N_COLUMNS });
        }

        if self.original.n_columns != NUM_COLUMNS_FIRST.into() {
            return Err(Error::ColumnsNumInvalid);
        }

        if self.interaction.n_columns != NUM_COLUMNS_SECOND.into() {
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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("value out of bounds {min} - {max}")]
    OutOfBounds { min: Felt, max: Felt },

    #[error("wrong numbers of columns")]
    ColumnsNumInvalid,

    #[error("Vector Error")]
    Vector(#[from] vector::config::Error),
}
