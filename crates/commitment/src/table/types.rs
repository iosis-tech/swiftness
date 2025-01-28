use super::config::Config;
use crate::vector;
use funvec::{FunVec, FUNVEC_DECOMMITMENT_VALUES};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_crypto::Felt;

// Commitment for a table (n_rows x n_columns) of field elements in montgomery form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    pub config: Config,
    pub vector_commitment: vector::types::Commitment,
}

// Responses for queries to the table commitment.
// Each query corresponds to a full row of the table.
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Decommitment {
    // n_columns * n_queries values to decommit.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
}

// Witness for a decommitment over queries.
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    pub vector: vector::types::Witness,
}
