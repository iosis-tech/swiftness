use cairo_felt::Felt252;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // Log2 of the size of the input layer to FRI.
    log_input_size: Felt252,
    // Number of layers in the FRI. Inner + last layer.
    n_layers: Felt252,
    // Array of size n_layers - 1, each entry is a configuration of a table commitment for the
    // corresponding inner layer.
    inner_layers: Vec<cairovm_verifier_commitment::table::types::Config>,
    // Array of size n_layers, each entry represents the FRI step size,
    // i.e. the number of FRI-foldings between layer i and i+1.
    fri_step_sizes: Vec<Felt252>,
    log_last_layer_degree_bound: Felt252,
}

// Commitment values for FRI. Used to generate a commitment by "reading" these values
// from the channel.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UnsentCommitment {
    // Array of size n_layers - 1 containing unsent table commitments for each inner layer.
    inner_layers: Vec<Felt252>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    last_layer_coefficients: Vec<Felt252>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    config: Config,
    // Array of size n_layers - 1 containing table commitments for each inner layer.
    inner_layers: Vec<cairovm_verifier_commitment::table::types::Commitment>,
    // Array of size n_layers, of one evaluation point for each layer.
    eval_points: Vec<Felt252>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    last_layer_coefficients: Vec<Felt252>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Decommitment {
    // Array of size n_values, containing the values of the input layer at query indices.
    values: Vec<Felt252>,
    // Array of size n_values, containing the field elements that correspond to the query indices
    // (See queries_to_points).
    points: Vec<Felt252>,
}

// A witness for the decommitment of the FRI layers over queries.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    // An array of size n_layers - 1, containing a witness for each inner layer.
    layers: Vec<LayerWitness>,
}

// A witness for a single FRI layer. This witness is required to verify the transition from an
// inner layer to the following layer.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LayerWitness {
    // Values for the sibling leaves required for decommitment.
    leaves: Vec<Felt252>,
    // Table commitment witnesses for decommiting all the leaves.
    table_witness: cairovm_verifier_commitment::table::types::Witness,
}
