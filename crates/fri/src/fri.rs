use alloc::boxed::Box;
use alloc::{borrow::ToOwned, vec::Vec};
use funvec::{FunVec, FUNVEC_QUERIES};
use starknet_crypto::Felt;
use swiftness_commitment::{
    table::{
        commit::table_commit,
        config::Config as TableCommitmentConfig,
        decommit::{table_decommit, MONTGOMERY_R},
        types::{Commitment as TableCommitment, Decommitment as TableDecommitment},
    },
    CacheCommitment,
};
use swiftness_transcript::transcript::Transcript;

use crate::{
    config::Config as FriConfig,
    first_layer::gather_first_layer_queries,
    group::get_fri_group,
    last_layer::verify_last_layer,
    layer::{compute_next_layer, FriLayerComputationParams, FriLayerQuery},
    types::{
        self, Commitment as FriCommitment, Decommitment as FriDecommitment, DecommitmentRef,
        LayerWitness, Witness,
    },
    ComputeNextLayerCache, FriVerifyCache,
};

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
    transcript: &mut Transcript,
    n_layers: Felt,
    configs: Vec<TableCommitmentConfig>,
    unsent_commitments: &[Felt],
) -> (Vec<TableCommitment>, Vec<Felt>) {
    let mut commitments = Vec::<TableCommitment>::new();
    let mut eval_points = Vec::<Felt>::new();

    let len: usize = n_layers.to_biguint().try_into().unwrap();
    for i in 0..len {
        // Read commitments.
        commitments.push(table_commit(
            transcript,
            *unsent_commitments.get(i).unwrap(),
            configs.get(i).unwrap().clone(),
        ));
        // Send the next eval_points.
        eval_points.push(transcript.random_felt_to_prover());
    }

    (commitments, eval_points)
}

pub fn fri_commit(
    transcript: &mut Transcript,
    unsent_commitment: types::UnsentCommitment,
    config: FriConfig,
) -> FriCommitment {
    assert!(config.n_layers > Felt::from(0), "Invalid value");

    let (commitments, eval_points) = fri_commit_rounds(
        transcript,
        config.n_layers - 1,
        config.inner_layers.to_vec(),
        &unsent_commitment.inner_layers.to_vec(),
    );

    // Read last layer coefficients.
    transcript.read_felt_vector_from_prover(&unsent_commitment.last_layer_coefficients.as_slice());
    let coefficients = unsent_commitment.last_layer_coefficients.to_vec();

    assert!(
        Felt::TWO.pow_felt(&config.log_last_layer_degree_bound) == coefficients.len().into(),
        "Invalid value"
    );

    FriCommitment {
        config,
        inner_layers: commitments,
        eval_points,
        last_layer_coefficients: coefficients,
    }
}

#[inline(always)]
fn fri_verify_layers<'a>(
    cache: &'a mut FriVerifyCache,
    fri_group: &[Felt],
    n_layers: Felt,
    commitment: &[TableCommitment],
    layer_witness: &mut [LayerWitness],
    eval_points: &[Felt],
    step_sizes: &[Felt],
    // queries: &mut FunVec<FriLayerQuery, { FUNVEC_QUERIES * 3 }>,
) -> &'a [FriLayerQuery] {
    let FriVerifyCache { fri_queries, .. } = cache;

    let len: usize = n_layers.to_biguint().try_into().unwrap();

    for i in 0..len {
        let target_layer_witness = layer_witness.get_mut(i).unwrap();
        let target_layer_witness_leaves = &mut target_layer_witness.leaves;
        let target_layer_witness_table_withness = &target_layer_witness.table_witness;
        let target_commitment = commitment.get(i).unwrap();

        // Params.
        let coset_size = Box::new(Felt::TWO.pow_felt(step_sizes.get(i).unwrap()));
        let params = FriLayerComputationParams {
            coset_size: &coset_size,
            fri_group,
            eval_point: eval_points.get(i).unwrap(),
        };

        // Compute next layer queries.
        compute_next_layer(
            &mut cache.next_layer_cache,
            fri_queries,
            target_layer_witness_leaves,
            params,
        )
        .unwrap();
        let ComputeNextLayerCache { next_queries, verify_indices, verify_y_values, .. } =
            cache.next_layer_cache;

        // // // Table decommitment.
        // let _ = table_decommit(
        //     &mut cache.commitment,
        //     &target_commitment,
        //     &verify_indices,
        //     &TableDecommitment {
        //         montgomery_values: FunVec::from_vec(
        //             verify_y_values.iter().map(|y| y * MONTGOMERY_R).collect(),
        //         ),
        //         values: FunVec::from_vec(verify_y_values),
        //     },
        //     &target_layer_witness_table_withness,
        // );

        // queries = next_queries;
    }

    fri_queries.as_slice()
}

// FRI protocol component decommitment.
#[inline(always)]
pub fn fri_verify(
    cache: &mut FriVerifyCache,
    queries: &[Felt],
    commitment: &FriCommitment,
    decommitment: &DecommitmentRef,
    witness: &mut Witness,
) -> Result<(), Error> {
    if queries.len() != decommitment.values.len() {
        return Err(Error::InvalidLength {
            expected: queries.len(),
            actual: decommitment.values.len(),
        });
    }

    // Compute first FRI layer queries.
    let fri_queries = gather_first_layer_queries(
        &mut cache.fri_queries,
        queries,
        decommitment.values,
        decommitment.points,
    );

    // Compute fri_group.
    let fri_group = get_fri_group();

    let fri_step_sizes = commitment.config.fri_step_sizes.as_slice();

    // Verify inner layers.
    let last_queries = fri_verify_layers(
        cache,
        &fri_group,
        commitment.config.n_layers - 1,
        &commitment.inner_layers,
        witness.layers.as_slice_mut(),
        &commitment.eval_points,
        &fri_step_sizes[1..fri_step_sizes.len()],
        // fri_queries,
    );

    if Felt::from(commitment.last_layer_coefficients.len())
        != Felt::TWO.pow_felt(&commitment.config.log_last_layer_degree_bound)
    {
        return Err(Error::InvalidValue);
    };

    verify_last_layer(last_queries, &commitment.last_layer_coefficients)
        .map_err(|_| Error::LastLayerVerificationError)?;

    Ok(())
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid length: expected {expected}, actual {actual}")]
    InvalidLength { expected: usize, actual: usize },

    #[error("Invalid value")]
    InvalidValue,

    #[error("Last layer verification error")]
    LastLayerVerificationError,
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid length: expected {expected}, actual {actual}")]
    InvalidLength { expected: usize, actual: usize },

    #[error("Invalid value")]
    InvalidValue,

    #[error("Last layer verification error")]
    LastLayerVerificationError,
}
