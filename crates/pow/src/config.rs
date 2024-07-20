use serde::{Deserialize, Serialize};

const MAX_PROOF_OF_WORK_BITS: u8 = 50;
const MIN_PROOF_OF_WORK_BITS: u8 = 20;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // Proof of work difficulty (number of bits required to be 0).
    pub n_bits: u8,
}

impl Config {
    pub fn validate(&self) -> Result<(), Error> {
        if self.n_bits < MIN_PROOF_OF_WORK_BITS || self.n_bits > MAX_PROOF_OF_WORK_BITS {
            Err(Error::OutOfBounds { min: MIN_PROOF_OF_WORK_BITS, max: MAX_PROOF_OF_WORK_BITS })
        } else {
            Ok(())
        }
    }
}

use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("value out of bounds {min} - {max}")]
    OutOfBounds { min: u8, max: u8 },
}
