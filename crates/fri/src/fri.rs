use starknet_crypto::Felt;

use crate::config::Config as FriConfig;

// Commitment values for FRI. Used to generate a commitment by "reading" these values
// from the channel.
pub struct FriUnsentCommitment {
    // Array of size n_layers - 1 containing unsent table commitments for each inner layer.
    pub inner_layers: Vec<Felt>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    pub last_layer_coefficients: Vec<Felt>,
}

pub struct FriCommitment {
    // config: FriConfig,
    // Array of size n_layers - 1 containing table commitments for each inner layer.
    // pub inner_layers: Vec<>,
}
