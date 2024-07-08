use super::types::{Commitment, Query, QueryWithDepth, Witness};
use sha3::Digest;
use sha3::Keccak256;
use starknet_core::types::NonZeroFelt;
use starknet_crypto::{poseidon_hash, Felt};
use thiserror::Error;

pub fn vector_commitment_decommit(
    commitment: Commitment,
    queries: &[Query],
    witness: Witness,
) -> Result<(), Error> {
    let shift = Felt::TWO.pow_felt(&commitment.config.height);
    // Shifts the query indices by shift=2**height, to convert index representation to heap-like.
    let shifted_queries: Vec<QueryWithDepth> = queries
        .iter()
        .map(|q| QueryWithDepth {
            index: q.index + shift,
            value: q.value,
            depth: commitment.config.height,
        })
        .collect();

    let expected_commitment = compute_root_from_queries(
        shifted_queries,
        0,
        commitment.config.n_verifier_friendly_commitment_layers,
        witness.authentications,
        0,
    )?;

    if commitment.commitment_hash != expected_commitment {
        return Err(Error::MisMatch {
            value: commitment.commitment_hash,
            expected: expected_commitment,
        });
    }
    Ok(())
}

pub fn compute_root_from_queries(
    mut queue: Vec<QueryWithDepth>,
    start: usize,
    n_verifier_friendly_layers: Felt,
    authentications: Vec<Felt>,
    auth_start: usize,
) -> Result<Felt, Error> {
    let current = queue.get(start).ok_or(Error::IndexInvalid)?;

    if current.index == Felt::ONE {
        // root
        Ok(current.value)
    } else {
        let (parent, bit) = current.index.div_rem(&NonZeroFelt::TWO);
        let is_verifier_friendly = n_verifier_friendly_layers >= current.depth;

        let hash = if bit == Felt::ZERO {
            if start + 1 != queue.len() {
                let next = queue.get(start + 1).ok_or(Error::IndexInvalid)?;
                if current.index + 1 == next.index {
                    // next is a sibling of current
                    let hash =
                        hash_friendly_unfriendly(current.value, next.value, is_verifier_friendly);
                    queue.push(QueryWithDepth {
                        index: parent,
                        value: hash,
                        depth: current.depth - 1,
                    });
                    return compute_root_from_queries(
                        queue,
                        start + 2,
                        n_verifier_friendly_layers,
                        authentications,
                        auth_start,
                    );
                }
            }
            hash_friendly_unfriendly(
                current.value,
                *authentications.get(auth_start).ok_or(Error::IndexInvalid)?,
                is_verifier_friendly,
            )
        } else {
            hash_friendly_unfriendly(
                *authentications.get(auth_start).ok_or(Error::IndexInvalid)?,
                current.value,
                is_verifier_friendly,
            )
        };

        queue.push(QueryWithDepth { index: parent, value: hash, depth: current.depth - 1 });

        compute_root_from_queries(
            queue,
            start + 1,
            n_verifier_friendly_layers,
            authentications,
            auth_start + 1,
        )
    }
}

fn hash_friendly_unfriendly(x: Felt, y: Felt, is_verifier_friendly: bool) -> Felt {
    if is_verifier_friendly {
        // poseidon hash
        poseidon_hash(x, y)
    } else {
        // keccak hash
        let mut hasher = Keccak256::new();
        let mut hash_data = Vec::with_capacity(40);
        hash_data.extend_from_slice(&x.to_bytes_be());
        hash_data.extend_from_slice(&y.to_bytes_be());
        hasher.update(&hash_data);
        Felt::from_bytes_be_slice(hasher.finalize().to_vec().as_slice()).floor_div(
            &NonZeroFelt::from_felt_unchecked(Felt::from_hex_unchecked(
                "0x10000000000000000000000000000000000000000",
            )),
        )
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("mismatch value {value} expected {expected}")]
    MisMatch { value: Felt, expected: Felt },
    #[error("authentications length is invalid")]
    AuthenticationInvalid,
    #[error("root tree-node error")]
    RootInvalid,
    #[error("root tree-node error")]
    IndexInvalid,
}
