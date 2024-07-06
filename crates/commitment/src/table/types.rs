use crate::vector;
use cairo_felt::Felt252;
use serde::{Deserialize, Serialize};

// Commitment for a table (n_rows x n_columns) of field elements in montgomery form.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    config: Config,
    vector_commitment: vector::types::Commitment,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    n_columns: Felt252,
    vector: vector::types::Config,
}

// Responses for queries to the table commitment.
// Each query corresponds to a full row of the table.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Decommitment {
    // n_columns * n_queries values to decommit.
    values: Vec<Felt252>,
}

// Witness for a decommitment over queries.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    vector: vector::types::Witness,
}
