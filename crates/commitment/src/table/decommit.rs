use crate::vector::{decommit::vector_commitment_decommit, types::Query};
use starknet_crypto::{poseidon_hash_many, Felt};
use thiserror::Error;

#[cfg(feature = "blake2s")]
use blake2::Blake2s256;
#[cfg(feature = "blake2s")]
use blake2::Digest;
#[cfg(feature = "keccak")]
use sha3::Digest;
#[cfg(feature = "keccak")]
use sha3::Keccak256;

use super::types::{Commitment, Decommitment, Witness};

const MONTGOMERY_R: Felt =
    Felt::from_hex_unchecked("0x7FFFFFFFFFFFDF0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE1");

pub fn table_decommit(
    commitment: Commitment,
    queries: &[Felt],
    decommitment: Decommitment,
    witness: Witness,
) -> Result<(), Error> {
    // An extra layer is added to the height since the table is considered as a layer, which is not
    // included in vector_commitment.config.
    let bottom_layer_depth = commitment.vector_commitment.config.height + 1;

    // Determine if the table commitment should use a verifier friendly hash function for the bottom
    // layer. The other layers' hash function will be determined in the vector_commitment logic.
    let is_bottom_layer_verifier_friendly =
        commitment.vector_commitment.config.n_verifier_friendly_commitment_layers
            >= bottom_layer_depth;

    let n_columns: u32 = commitment.config.n_columns.to_bigint().try_into().unwrap();
    if n_columns as usize * queries.len() != decommitment.values.len() {
        return Err(Error::DecommitmentLength);
    }

    // Convert decommitment values to Montgomery form, since the commitment is in that form.
    let montgomery_values: Vec<Felt> =
        decommitment.values.into_iter().map(|v| v * MONTGOMERY_R).collect();

    // Generate queries to the underlying vector commitment.
    let vector_queries = generate_vector_queries(
        queries,
        &montgomery_values,
        n_columns,
        is_bottom_layer_verifier_friendly,
    );

    Ok(vector_commitment_decommit(commitment.vector_commitment, &vector_queries, witness.vector)?)
}

fn generate_vector_queries(
    queries: &[Felt],
    values: &[Felt],
    n_columns: u32,
    is_verifier_friendly: bool,
) -> Vec<Query> {
    let mut vector_queries = Vec::new();
    for i in 0..queries.len() {
        let hash = if n_columns == 1 {
            values[i]
        } else if is_verifier_friendly {
            let slice = &values[(i * n_columns as usize)..((i + 1) * n_columns as usize)];
            poseidon_hash_many(slice)
        } else {
            let slice = &values[(i * n_columns as usize)..((i + 1) * n_columns as usize)];
            let mut data = Vec::new();
            data.extend(slice.iter().flat_map(|x| x.to_bytes_be().to_vec()));

            #[cfg(feature = "keccak")]
            let mut hasher = Keccak256::new();
            #[cfg(feature = "blake2s")]
            let mut hasher = Blake2s256::new();

            hasher.update(&data);
            Felt::from_bytes_be_slice(&hasher.finalize().to_vec().as_slice()[12..32])
        };

        vector_queries.push(Query { index: queries[i], value: hash })
    }

    vector_queries
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid decommitment length")]
    DecommitmentLength,

    #[error("Vector Error")]
    Vector(#[from] crate::vector::decommit::Error),
}
