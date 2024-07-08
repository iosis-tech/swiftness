use cairovm_verifier_commitment::table::{
    commit::table_commit,
    config::Config as TableCommitmentConfig,
    decommit::table_decommit,
    types::{
        Commitment as TableCommitment, Decommitment as TableDecommitment,
        Witness as TableCommitmentWitness,
    },
};
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

use crate::{
    config::Config as FriConfig,
    first_layer::gather_first_layer_queries,
    group::get_fri_group,
    layer::{compute_next_layer, FriLayerComputationParams, FriLayerQuery},
};

// Commitment values for FRI. Used to generate a commitment by "reading" these values
// from the channel.
pub struct FriUnsentCommitment {
    // Array of size n_layers - 1 containing unsent table commitments for each inner layer.
    pub inner_layers: Vec<Felt>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    pub last_layer_coefficients: Vec<Felt>,
}

#[derive(Debug, PartialEq)]
pub struct FriCommitment {
    pub config: FriConfig,
    // Array of size n_layers - 1 containing table commitments for each inner layer.
    pub inner_layers: Vec<TableCommitment>,
    // Array of size n_layers, of one evaluation point for each layer.
    pub eval_points: Vec<Felt>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    pub last_layer_coefficients: Vec<Felt>,
}

struct FriDecommitment {
    // Array of size n_values, containing the values of the input layer at query indices.
    values: Vec<Felt>,
    // Array of size n_values, containing the field elements that correspond to the query indices
    // (See queries_to_points).
    points: Vec<Felt>,
}

// A witness for the decommitment of the FRI layers over queries.
struct FriWitness {
    // An array of size n_layers - 1, containing a witness for each inner layer.
    layers: Vec<FriLayerWitness>,
}

#[derive(Clone)]
struct FriLayerWitness {
    // Values for the sibling leaves required for decommitment.
    pub leaves: Vec<Felt>,
    // Table commitment witnesses for decommiting all the leaves.
    table_witness: TableCommitmentWitness,
}

// A FRI phase with N layers starts with a single input layer.
// Afterwards, there are N - 1 inner layers resulting from FRI-folding each preceding layer.
// Each such layer has a separate table commitment, for a total of N - 1 commitments.
// Lastly, there is another FRI-folding resulting in the last FRI layer, that is commited by
// sending the polynomial coefficients, instead of a table commitment.
// Each folding has a step size.
// Illustration:
// InputLayer, no commitment.
//   fold step 0
// InnerLayer 0, Table commitment
//   fold step 1
// ...
// InnerLayer N - 2, Table commitment
//   fold step N - 1
// LastLayer, Polynomial coefficients
//
// N steps.
// N - 1 inner layers.

// Performs FRI commitment phase rounds. Each round reads a commitment on a layer, and sends an
// evaluation point for the next round.
pub fn fri_commit_rounds(
    channel: &mut Transcript,
    n_layers: Felt,
    configs: Vec<TableCommitmentConfig>,
    unsent_commitments: Vec<Felt>,
) -> (Vec<TableCommitment>, Vec<Felt>) {
    let mut commitments = Vec::<TableCommitment>::new();
    let mut eval_points = Vec::<Felt>::new();

    let len: usize = n_layers.to_biguint().try_into().unwrap();
    for i in 0..len {
        let config = configs.get(i).unwrap().clone();
        // Read commitments.
        commitments.push(table_commit(channel, *unsent_commitments.get(i).unwrap(), config));
        // Send the next eval_points.
        eval_points.push(channel.random_felt_to_prover());
    }

    (commitments, eval_points)
}

pub fn fri_commit(
    channel: &mut Transcript,
    unsent_commitment: FriUnsentCommitment,
    config: FriConfig,
) -> FriCommitment {
    assert!(config.n_layers > Felt::from(0), "Invalid value");
    let inner_layers = config.inner_layers.clone();

    let (commitments, eval_points) = fri_commit_rounds(
        channel,
        config.n_layers - 1,
        inner_layers,
        unsent_commitment.inner_layers,
    );

    // Read last layer coefficients.
    channel.read_felt_vector_from_prover(&unsent_commitment.last_layer_coefficients);
    let coefficients = unsent_commitment.last_layer_coefficients;
    let log_last_layer_degree_bound: u128 =
        config.log_last_layer_degree_bound.to_biguint().try_into().unwrap();

    assert!(
        Felt::from(2).pow(log_last_layer_degree_bound) == coefficients.len().into(),
        "Invalid value"
    );

    FriCommitment {
        config,
        inner_layers: commitments,
        eval_points,
        last_layer_coefficients: coefficients,
    }
}

fn fri_verify_layers(
    fri_group: Vec<Felt>,
    n_layers: Felt,
    commitment: Vec<TableCommitment>,
    layer_witness: Vec<FriLayerWitness>,
    eval_points: Vec<Felt>,
    step_sizes: Vec<Felt>,
    mut queries: Vec<FriLayerQuery>,
) -> Vec<FriLayerQuery> {
    let len: usize = n_layers.to_biguint().try_into().unwrap();

    for i in 0..len {
        let step_size_uint: u128 = step_sizes.get(i).unwrap().to_biguint().try_into().unwrap();
        let target_layer_witness = layer_witness.get(i).unwrap().clone();
        let mut target_layer_witness_leaves = target_layer_witness.leaves;
        let target_layer_witness_table_withness = target_layer_witness.table_witness;
        let target_commitment = commitment.get(i).unwrap().clone();

        // Params.
        let coset_size = Felt::from(2).pow(step_size_uint);
        let params = FriLayerComputationParams {
            coset_size,
            fri_group: fri_group.clone(),
            eval_point: *eval_points.get(i).unwrap(),
        };

        // Compute next layer queries.
        let (next_queries, verify_indices, verify_y_values) =
            compute_next_layer(&mut queries, &mut target_layer_witness_leaves, params).unwrap();

        // Table decommitment.
        let _ = table_decommit(
            target_commitment,
            &verify_indices,
            TableDecommitment { values: verify_y_values },
            target_layer_witness_table_withness,
        );

        queries = next_queries;
    }

    queries
}

// FRI protocol component decommitment.
fn fri_verify(
    queries: Vec<Felt>,
    commitment: FriCommitment,
    decommitment: FriDecommitment,
    witness: FriWitness,
) {
    if queries.len() != decommitment.values.len() {
        assert!(false, "Invalid value");
    }

    // Compute first FRI layer queries.
    let fri_queries = gather_first_layer_queries(queries, decommitment.values, decommitment.points);

    // Compute fri_group.
    let fri_group = get_fri_group();

    // Verify inner layers.
    let last_queries = fri_verify_layers(
        fri_group,
        commitment.config.n_layers - 1,
        commitment.inner_layers,
        witness.layers,
        commitment.eval_points,
        commitment.config.fri_step_sizes[1..commitment.config.fri_step_sizes.len() - 1].to_vec(),
        fri_queries,
    );

    let log_last_layer_degree_bound: u128 =
        commitment.config.log_last_layer_degree_bound.to_biguint().try_into().unwrap();
    if Felt::from(commitment.last_layer_coefficients.len())
        == Felt::from(2).pow(log_last_layer_degree_bound)
    {
        assert!(true, "Invalid value");
    }
}
