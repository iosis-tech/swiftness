use crate::vector;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

// Commitment for a table (n_rows x n_columns) of field elements in montgomery form.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    config: Config,
    vector_commitment: vector::types::Commitment,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde_as(as = "UfeHex")]
    n_columns: Felt,
    vector: vector::config::Config,
}

// Responses for queries to the table commitment.
// Each query corresponds to a full row of the table.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Decommitment {
    // n_columns * n_queries values to decommit.
    #[serde_as(as = "Vec<UfeHex>")]
    values: Vec<Felt>,
}

// Witness for a decommitment over queries.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    vector: vector::types::Witness,
}
