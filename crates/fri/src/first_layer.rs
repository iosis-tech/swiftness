use alloc::vec::Vec;
use funvec::{FunVec, FUNVEC_QUERIES};
use starknet_core::types::NonZeroFelt;
use starknet_crypto::Felt;

use crate::{layer::FriLayerQuery, FriQueries};

const FIELD_GENERATOR_INVERSE: Felt =
    Felt::from_hex_unchecked("0x2AAAAAAAAAAAAB0555555555555555555555555555555555555555555555556");

pub fn gather_first_layer_queries<'a>(
    fri_queries: &'a mut FriQueries,
    queries: &[Felt],
    evaluations: &[Felt],
    x_values: &[Felt],
) -> &'a mut [FriLayerQuery] {
    fri_queries.flush();

    for (index, query) in queries.iter().enumerate() {
        // Translate the coset to the homogenous group to have simple FRI equations.
        let shifted_x_value = x_values.get(index).unwrap() * FIELD_GENERATOR_INVERSE;

        fri_queries.push(FriLayerQuery {
            index: *query,
            y_value: *evaluations.get(index).unwrap(),
            x_inv_value: Felt::ONE.field_div(&NonZeroFelt::from_felt_unchecked(shifted_x_value)),
        });
    }

    fri_queries.as_slice_mut()
}
