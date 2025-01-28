use crate::config::Config;
use alloc::vec::Vec;
use funvec::{FunVec, FUNVEC_LAST_LAYER, FUNVEC_LAYERS, FUNVEC_LEAVES};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_crypto::Felt;

// Commitment values for FRI. Used to generate a commitment by "reading" these values
// from the transcript.
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UnsentCommitment {
    // Array of size n_layers - 1 containing unsent table commitments for each inner layer.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub inner_layers: FunVec<Felt, FUNVEC_LAYERS>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub last_layer_coefficients: FunVec<Felt, FUNVEC_LAST_LAYER>,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    pub config: Config,
    // Array of size n_layers - 1 containing table commitments for each inner layer.
    pub inner_layers: Vec<swiftness_commitment::table::types::Commitment>,
    // Array of size n_layers, of one evaluation point for each layer.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub eval_points: Vec<Felt>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub last_layer_coefficients: Vec<Felt>,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Decommitment {
    // Array of size n_values, containing the values of the input layer at query indices.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub values: Vec<Felt>,
    // Array of size n_values, containing the field elements that correspond to the query indices
    // (See queries_to_points).
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub points: Vec<Felt>,
}

// A witness for the decommitment of the FRI layers over queries.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    // An array of size n_layers - 1, containing a witness for each inner layer.
    pub layers: FunVec<LayerWitness, FUNVEC_LAYERS>,
}

// A witness for a single FRI layer. This witness is required to verify the transition from an
// inner layer to the following layer.
#[serde_as]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LayerWitness {
    // Values for the sibling leaves required for decommitment.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "Vec<starknet_core::serde::unsigned_field_element::UfeHex>")
    )]
    pub leaves: FunVec<Felt, FUNVEC_LEAVES>,
    // Table commitment witnesses for decommiting all the leaves.
    pub table_witness: swiftness_commitment::table::types::Witness,
}
