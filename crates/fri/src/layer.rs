use starknet_crypto::Felt;

pub struct FriLayerComputationParams {
    coset_size: usize,
    fri_group: Vec<Felt>,
    eval_point: Felt,
}

pub struct FriLayerQuery {
    index: Felt,
    y_value: Felt,
    x_inv_value: Felt,
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
    coset_size: usize,
    coset_start_index: usize,
    fri_group: Vec<Felt>,
) -> (Vec<Felt>, Felt) {
    let mut coset_elements = Vec::new();
    let mut coset_x_inv = Felt::ZERO;
    for index in 0..coset_size {
        let q = queries.get(index);
        // route
        if let Some(q) = q {
            let query_index: usize = q.index.to_bigint().try_into().unwrap();
            if query_index == coset_start_index + index {
                let query = queries.swap_remove(0);
                coset_elements.push(query.y_value);
                coset_x_inv = query.x_inv_value + fri_group.get(index).unwrap();
            }
        } else {
            coset_elements.push(sibling_witness.swap_remove(0));
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
        let query_index: usize = queries.first().unwrap().index.to_bigint().try_into().unwrap();
        let coset_index = query_index / coset_size;

        verify_indices.push(Felt::from(coset_index as u64));

        let (coset_elements, coset_x_inv) = compute_coset_elements(
            queries,
            sibling_witness,
            coset_size,
            coset_index * coset_size,
            params.fri_group.clone(),
        );

        verify_y_values.extend(coset_elements.iter());

        let fri_formula_res =
            fri_formula(coset_elements, params.eval_point, coset_x_inv, coset_size)?;

        let next_x_inv = coset_x_inv.pow(params.coset_size as u64);
        next_queries.push(FriLayerQuery {
            index: Felt::from(coset_index as u64),
            y_value: fri_formula_res,
            x_inv_value: next_x_inv,
        });
    }

    Ok((next_queries, verify_indices, verify_y_values))
}

use thiserror::Error;

use crate::formula::fri_formula;

#[derive(Error, Debug)]
pub enum FriError {
    #[error("FRI formula error: {0}")]
    FriFormulaError(#[from] crate::formula::Error),
}
