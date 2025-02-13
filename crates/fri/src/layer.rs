use alloc::vec::Vec;
use funvec::{FunVec, FUNVEC_AUTHENTICATIONS, FUNVEC_QUERIES};
use starknet_crypto::Felt;

pub struct FriLayerComputationParams<'a> {
    pub coset_size: &'a Felt,
    pub fri_group: &'a [Felt],
    pub eval_point: &'a Felt,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FriLayerQuery {
    pub index: Felt,
    pub y_value: Felt,
    pub x_inv_value: Felt,
}

// Computes the elements of the coset starting at coset_start_index.
//
// Inputs:
//   - queries: an iterator over the input queries.
//   - sibling_witness: a list of all the query's siblings.
//   - coset_size: the number of elements in the coset.
//   - coset_start_index: the index of the first element of the coset being calculated.
//   - fri_group: holds the group <g> in bit reversed order, where g is the generator of the coset.
//
// Outputs:
//   - coset_elements: the values of the coset elements.
//   - coset_x_inv: x_inv of the first element in the coset. This value is set only if at least one
//     query was consumed by this function.
pub fn compute_coset_elements(
    coset_elements: &mut FunVec<Felt, FUNVEC_QUERIES>,
    queries: &mut FriQueries,
    sibling_witness: &mut FunVec<Felt, FUNVEC_AUTHENTICATIONS>,
    coset_size: &Felt,
    coset_start_index: Felt,
    fri_group: &[Felt],
    // ) -> (Vec<Felt>, Felt) {
) -> Felt {
    // let mut coset_elements = Vec::new();
    coset_elements.flush();

    let mut coset_x_inv = Felt::ZERO;
    let coset_size: usize = coset_size.to_biguint().try_into().unwrap();
    for index in 0..coset_size {
        let q = queries.first();
        if q.is_some() && q.unwrap().index == coset_start_index + Felt::from(index) {
            coset_elements.push(q.unwrap().y_value);
            coset_x_inv = q.unwrap().x_inv_value * fri_group.get(index).unwrap();
            queries.shift(2);
        } else {
            let witness = sibling_witness.first();
            coset_elements.push(*witness.unwrap());
            sibling_witness.shift(2);
        }
    }

    coset_x_inv
}

// Computes FRI next layer for the given queries. I.e., takes the given i-th layer queries
// and produces queries for layer i+1 (a single query for each coset in the i-th layer).
//
// Inputs:
//   - queries: input queries.
//   - sibling_witness: a list of all the query's siblings.
//   - params: the parameters to use for the layer computation.
//
// Outputs:
//   - next_queries: queries for the next layer.
//   - verify_indices: query indices of the given layer for Merkle verification.
//   - verify_y_values: query y values of the given layer for Merkle verification.
#[allow(clippy::type_complexity)]
#[inline(always)]
pub fn compute_next_layer(
    cache: &mut ComputeNextLayerCache,
    queries: &mut FunVec<FriLayerQuery, { FUNVEC_QUERIES * 3 }>,
    sibling_witness: &mut FunVec<Felt, FUNVEC_AUTHENTICATIONS>,
    params: FriLayerComputationParams,
) -> Result<
    // (&mut FunVec<FriLayerQuery, 256>, &mut FunVec<Felt, 256>, &mut FunVec<Felt, 256>),
    (),
    FriError,
> {
    // let mut next_queries = Vec::new();
    // let mut verify_indices = Vec::new();
    // let mut verify_y_values = Vec::new();
    let ComputeNextLayerCache { next_queries, verify_indices, verify_y_values, coset_elements } =
        cache;
    next_queries.flush();
    verify_indices.flush();
    verify_y_values.flush();

    let coset_size = params.coset_size;
    while !queries.is_empty() {
        let query_uint = queries.at(0).index.to_biguint();
        let coset_size_uint = coset_size.to_biguint();
        let coset_index =
            Felt::from_bytes_be_slice((query_uint / coset_size_uint).to_bytes_be().as_slice());

        verify_indices.push(coset_index);

        let coset_x_inv = compute_coset_elements(
            coset_elements,
            queries,
            sibling_witness,
            coset_size,
            coset_index * coset_size,
            &params.fri_group,
        );

        verify_y_values.extend(coset_elements.as_slice());

        let fri_formula_res =
            fri_formula(coset_elements.as_slice(), *params.eval_point, coset_x_inv, *coset_size)?;

        let next_x_inv = coset_x_inv.pow_felt(&params.coset_size);
        next_queries.push(FriLayerQuery {
            index: coset_index,
            y_value: fri_formula_res,
            x_inv_value: next_x_inv,
        });
    }

    // Ok((next_queries, verify_indices, verify_y_values))
    Ok(())
}

use thiserror_no_std::Error;

use crate::{formula::fri_formula, ComputeNextLayerCache, FriQueries};

#[derive(Error, Debug)]
pub enum FriError {
    #[error("FRI formula error: {0}")]
    FriFormulaError(#[from] crate::formula::Error),
}
