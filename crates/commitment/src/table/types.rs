use super::config::Config;
use crate::vector;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Decommitment {
    // n_columns * n_queries values to decommit.
    #[serde_as(as = "Vec<UfeHex>")]
    pub values: Vec<Felt>,
}

// Witness for a decommitment over queries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    pub vector: vector::types::Witness,
}
