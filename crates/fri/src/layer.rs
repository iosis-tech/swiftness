use alloc::vec::Vec;
use starknet_crypto::Felt;

pub struct FriLayerComputationParams {
    pub coset_size: Felt,
    pub fri_group: Vec<Felt>,
    pub eval_point: Felt,
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
    queries: &mut Vec<FriLayerQuery>,
    sibling_witness: &mut Vec<Felt>,
    coset_size: Felt,
    coset_start_index: Felt,
    fri_group: &[Felt],
) -> (Vec<Felt>, Felt) {
    let mut coset_elements = Vec::new();
    let mut coset_x_inv = Felt::ZERO;
    let coset_size: usize = coset_size.to_biguint().try_into().unwrap();
    for index in 0..coset_size {
        let q = queries.first();
        if q.is_some() && q.unwrap().index == coset_start_index + Felt::from(index) {
            let query: Vec<FriLayerQuery> = queries.drain(0..1).collect();
            coset_elements.push(query[0].y_value);
            coset_x_inv = query[0].x_inv_value * fri_group.get(index).unwrap();
        } else {
            let withness: Vec<Felt> = sibling_witness.drain(0..1).collect();
            coset_elements.push(withness[0]);
        }
    }

    (coset_elements, coset_x_inv)
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
pub fn compute_next_layer(
    queries: &mut Vec<FriLayerQuery>,
    sibling_witness: &mut Vec<Felt>,
    params: FriLayerComputationParams,
) -> Result<(Vec<FriLayerQuery>, Vec<Felt>, Vec<Felt>), FriError> {
    let mut next_queries = Vec::new();
    let mut verify_indices = Vec::new();
    let mut verify_y_values = Vec::new();

    let coset_size = params.coset_size;
    while !queries.is_empty() {
        let query_uint = queries.first().unwrap().index.to_biguint();
        let coset_size_uint = coset_size.to_biguint();
        let coset_index =
            Felt::from_bytes_be_slice((query_uint / coset_size_uint).to_bytes_be().as_slice());

        verify_indices.push(coset_index);

        let (coset_elements, coset_x_inv) = compute_coset_elements(
            queries,
            sibling_witness,
            coset_size,
            coset_index * coset_size,
            &params.fri_group,
        );
        verify_y_values.extend(coset_elements.iter());

        let fri_formula_res =
            fri_formula(coset_elements, params.eval_point, coset_x_inv, coset_size)?;

        let next_x_inv = coset_x_inv.pow_felt(&params.coset_size);
        next_queries.push(FriLayerQuery {
            index: coset_index,
            y_value: fri_formula_res,
            x_inv_value: next_x_inv,
        });
    }

    Ok((next_queries, verify_indices, verify_y_values))
}

use thiserror_no_std::Error;

use crate::formula::fri_formula;

#[derive(Error, Debug)]
pub enum FriError {
    #[error("FRI formula error: {0}")]
    FriFormulaError(#[from] crate::formula::Error),
}
