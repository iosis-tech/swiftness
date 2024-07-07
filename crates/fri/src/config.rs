use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

const MAX_LAST_LAYER_LOG_DEGREE_BOUND: u64 = 14;
const MAX_FRI_LAYERS: u64 = 15;
const MIN_FRI_LAYERS: u64 = 15;
const MAX_FRI_STEP: u64 = 4;
const MIN_FRI_STEP: u64 = 1;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // Log2 of the size of the input layer to FRI.
    #[serde_as(as = "UfeHex")]
    log_input_size: Felt,
    // Number of layers in the FRI. Inner + last layer.
    #[serde_as(as = "UfeHex")]
    n_layers: Felt,
    // Array of size n_layers - 1, each entry is a configuration of a table commitment for the
    // corresponding inner layer.
    inner_layers: Vec<cairovm_verifier_commitment::table::types::Config>,
    // Array of size n_layers, each entry represents the FRI step size,
    // i.e. the number of FRI-foldings between layer i and i+1.
    #[serde_as(as = "Vec<UfeHex>")]
    fri_step_sizes: Vec<Felt>,
    #[serde_as(as = "UfeHex")]
    log_last_layer_degree_bound: Felt,
}

impl Config {
    pub fn validate(&self) -> Result<(), Error> {
        if self.n_layers < MIN_FRI_LAYERS.into() || self.n_layers > MAX_FRI_LAYERS.into() {
            return Err(Error::OutOfBounds { min: MIN_FRI_LAYERS, max: MAX_FRI_LAYERS });
        }
        if self.log_last_layer_degree_bound < Felt::ZERO
            || self.log_last_layer_degree_bound > MAX_LAST_LAYER_LOG_DEGREE_BOUND.into()
        {
            return Err(Error::OutOfBounds { min: 0, max: MAX_LAST_LAYER_LOG_DEGREE_BOUND });
        }
        if *self.fri_step_sizes.get(0).ok_or(Error::FirstFriStepInvalid)? == Felt::ZERO {
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
        }

        Ok(())
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("value out of bounds {min} - {max}")]
    OutOfBounds { min: u64, max: u64 },
    #[error("invalid first fri step")]
    FirstFriStepInvalid,
}
