use core::panic;

use super::types::{Commitment, Decommitment, Witness};
use crate::{
    vector::{decommit::vector_commitment_decommit, types::Query},
    CacheCommitment,
};
use alloc::boxed::Box;
use alloc::vec::Vec;
#[cfg(any(feature = "blake2s_160_lsb", feature = "blake2s_248_lsb"))]
use blake2::{Blake2s256, Digest};
use funvec::{print_address, print_frame};
use num_bigint::{BigInt, TryFromBigIntError};
#[cfg(any(feature = "keccak_160_lsb", feature = "keccak_248_lsb"))]
use sha3::{Digest, Keccak256};
use starknet_crypto::{poseidon_hash_many, Felt};

pub const MONTGOMERY_R: Felt =
    Felt::from_hex_unchecked("0x7FFFFFFFFFFFDF0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE1");

#[inline(always)]
pub fn table_decommit(
    cache: &mut CacheCommitment,
    commitment: &Commitment,
    queries: &[Felt],
    decommitment: &Decommitment,
    witness: &Witness,
) -> Result<(), Error> {
    // TODO: uncomment
    // An extra layer is added to the height since the table is considered as a layer, which is not
    // included in vector_commitment.config.
    let bottom_layer_depth = Box::new(commitment.vector_commitment.config.height + 1);

    // Determine if the table commitment should use a verifier friendly hash function for the bottom
    // layer. The other layers' hash function will be determined in the vector_commitment logic.
    let is_bottom_layer_verifier_friendly =
        &commitment.vector_commitment.config.n_verifier_friendly_commitment_layers
            >= &bottom_layer_depth;

    let n_columns: u32 = commitment.config.n_columns.to_bigint().try_into()?;
    if n_columns as usize * queries.len() != decommitment.values.len() {
        return Err(Error::DecommitmentLength);
    }

    // // Convert decommitment values to Montgomery form, since the commitment is in that form.
    let montgomery_values = decommitment.montgomery_values.as_slice();
    for (m, v) in montgomery_values.iter().zip(decommitment.values.iter()) {
        assert!(m == &(v * MONTGOMERY_R));
    }

    let CacheCommitment { vector_queries, shifted_queries, .. } = cache;
    shifted_queries.flush();
    assert_eq!(shifted_queries.len(), 0);

    let vector_queries = vector_queries.unchecked_slice_mut(queries.len());

    let vector_queries = generate_vector_queries(
        vector_queries,
        queries,
        &decommitment.montgomery_values.as_slice(),
        n_columns,
        is_bottom_layer_verifier_friendly,
    );

    assert_eq!(vector_queries.len(), queries.len());

    vector_commitment_decommit(
        shifted_queries,
        &commitment.vector_commitment,
        &vector_queries,
        &witness.vector,
    )
    .unwrap();

    Ok(())
}

#[inline(always)]
fn generate_vector_queries<'a>(
    vector_queries: &'a mut [Query],
    queries: &[Felt],
    values: &[Felt],
    n_columns: u32,
    is_verifier_friendly: bool,
) -> &'a [Query] {
    for i in 0..queries.len() {
        let hash = if n_columns == 1 {
            values[i]
        } else if is_verifier_friendly {
            let slice = &values[(i * n_columns as usize)..((i + 1) * n_columns as usize)];
            poseidon_hash_many(slice)
        } else {
            unimplemented!("big allocations would not fit on Solana");
            // let slice = &values[(i * n_columns as usize)..((i + 1) * n_columns as usize)];
            // let mut data = Vec::new();
            // data.extend(slice.iter().flat_map(|x| x.to_bytes_be().to_vec()));

            // let mut hasher = {
            //     #[cfg(any(feature = "keccak_160_lsb", feature = "keccak_248_lsb"))]
            //     {
            //         Keccak256::new()
            //     }
            //     #[cfg(any(feature = "blake2s_160_lsb", feature = "blake2s_248_lsb"))]
            //     {
            //         Blake2s256::new()
            //     }
            // };

            // hasher.update(&data);

            // {
            //     #[cfg(any(feature = "keccak_160_lsb", feature = "blake2s_160_lsb"))]
            //     {
            //         Felt::from_bytes_be_slice(&hasher.finalize().as_slice()[12..32])
            //     }

            //     #[cfg(any(feature = "keccak_248_lsb", feature = "blake2s_248_lsb"))]
            //     {
            //         Felt::from_bytes_be_slice(&hasher.finalize().as_slice()[1..32])
            //     }
            // }
        };

        vector_queries[i] = Query { index: queries[i], value: hash };
    }

    // vector_queries
    &vector_queries[..queries.len()]
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid decommitment length")]
    DecommitmentLength,

    #[error("Vector Error")]
    Vector(#[from] crate::vector::decommit::Error),

    #[error("BigInt conversion Error")]
    TryFromBigInt(#[from] TryFromBigIntError<BigInt>),
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid decommitment length")]
    DecommitmentLength,

    #[error("Vector Error")]
    Vector(#[from] crate::vector::decommit::Error),

    #[error("BigInt conversion Error")]
    TryFromBigInt(#[from] TryFromBigIntError<BigInt>),
}
