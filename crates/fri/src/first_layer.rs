use starknet_core::types::NonZeroFelt;
use starknet_crypto::Felt;

use crate::layer::FriLayerQuery;

pub fn gather_first_layer_queries(
    queries: Vec<Felt>,
    evaluations: Vec<Felt>,
    x_values: Vec<Felt>,
) -> Vec<FriLayerQuery> {
    let field_generator_inverse = Felt::from_dec_str(
        "1206167596222043737899107594365023368541035738443865566657697352045290673494",
    )
    .unwrap();
    let mut fri_queries = Vec::new();

    for (index, query) in queries.iter().enumerate() {
        // Translate the coset to the homogenous group to have simple FRI equations.
        let shifted_x_value = x_values.get(index).unwrap() * field_generator_inverse;

        fri_queries.push(FriLayerQuery {
            index: *query,
            y_value: *evaluations.get(index).unwrap(),
            x_inv_value: Felt::from(1)
                .field_div(&NonZeroFelt::from_felt_unchecked(shifted_x_value)),
        });
    }

    fri_queries
}
