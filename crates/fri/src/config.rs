use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

const MAX_LAST_LAYER_LOG_DEGREE_BOUND: u64 = 15;
const MAX_FRI_LAYERS: u64 = 15;
const MIN_FRI_LAYERS: u64 = 2;
const MAX_FRI_STEP: u64 = 4;
const MIN_FRI_STEP: u64 = 1;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // Log2 of the size of the input layer to FRI.
    #[serde_as(as = "UfeHex")]
    pub log_input_size: Felt,
    // Number of layers in the FRI. Inner + last layer.
    #[serde_as(as = "UfeHex")]
    pub n_layers: Felt,
    // Array of size n_layers - 1, each entry is a configuration of a table commitment for the
    // corresponding inner layer.
    pub inner_layers: Vec<cairovm_verifier_commitment::table::config::Config>,
    // Array of size n_layers, each entry represents the FRI step size,
    // i.e. the number of FRI-foldings between layer i and i+1.
    #[serde_as(as = "Vec<UfeHex>")]
    pub fri_step_sizes: Vec<Felt>,
    #[serde_as(as = "UfeHex")]
    pub log_last_layer_degree_bound: Felt,
}

impl Config {
    pub fn validate(
        &self,
        log_n_cosets: Felt,
        n_verifier_friendly_commitment_layers: Felt,
    ) -> Result<Felt, Error> {
        if self.n_layers < MIN_FRI_LAYERS.into() || self.n_layers > MAX_FRI_LAYERS.into() {
            return Err(Error::OutOfBounds { min: MIN_FRI_LAYERS, max: MAX_FRI_LAYERS });
        }
        if self.log_last_layer_degree_bound < Felt::ZERO
            || self.log_last_layer_degree_bound > MAX_LAST_LAYER_LOG_DEGREE_BOUND.into()
        {
            return Err(Error::OutOfBounds { min: 0, max: MAX_LAST_LAYER_LOG_DEGREE_BOUND });
        }
        if *self.fri_step_sizes.first().ok_or(Error::FirstFriStepInvalid)? != Felt::ZERO {
            return Err(Error::FirstFriStepInvalid);
        }

        let n_layers: usize = self.n_layers.to_bigint().try_into().unwrap();
        let mut sum_of_step_sizes = Felt::ZERO;
        let mut log_input_size = self.log_input_size;

        for i in 1..n_layers {
            let fri_step = self.fri_step_sizes[i];
            let table_commitment = &self.inner_layers[i - 1];
            log_input_size -= fri_step;
            sum_of_step_sizes += fri_step;

            if fri_step < MIN_FRI_STEP.into() || fri_step > MAX_FRI_STEP.into() {
                return Err(Error::OutOfBounds { min: MIN_FRI_STEP, max: MAX_FRI_STEP });
            }
            let fri_step: u64 = fri_step.to_bigint().try_into().unwrap();
            let expected_n_columns = Felt::from(2).pow(fri_step);
            if table_commitment.n_columns != expected_n_columns {
                return Err(Error::InvalidColumnCount {
                    expected: expected_n_columns,
                    actual: table_commitment.n_columns,
                });
            }
            table_commitment
                .vector
                .validate(log_input_size, n_verifier_friendly_commitment_layers)?;
        }

        let log_expected_input_degree = sum_of_step_sizes + self.log_last_layer_degree_bound;
        if log_expected_input_degree + log_n_cosets != self.log_input_size {
            return Err(Error::LogInputSizeMismatch {
                expected: log_expected_input_degree,
                actual: self.log_input_size,
            });
        }

        Ok(log_expected_input_degree)
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("value out of bounds {min} - {max}")]
    OutOfBounds { min: u64, max: u64 },
    #[error("invalid first fri step")]
    FirstFriStepInvalid,
    #[error("invalid value for column count, expected {expected}, got {actual}")]
    InvalidColumnCount { expected: Felt, actual: Felt },
    #[error("log input size mismatch, expected {expected}, got {actual}")]
    LogInputSizeMismatch { expected: Felt, actual: Felt },
    #[error("vector validation failed: {0}")]
    VectorValidationFailed(#[from] cairovm_verifier_commitment::vector::config::Error),
}
