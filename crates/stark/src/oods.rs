use cairovm_verifier_air::{
    layout::recursive::{
        eval_composition_polynomial, eval_oods_polynomial, global_values::InteractionElements,
        CONSTRAINT_DEGREE,
    },
    public_memory::PublicInput,
    trace,
};
use cairovm_verifier_commitment::table;
use starknet_crypto::Felt;

pub struct OodsEvaluationInfo {
    pub oods_values: Vec<Felt>,
    pub oods_point: Felt,
    pub trace_generator: Felt,
    pub constraint_coefficients: Vec<Felt>,
}

// Checks that the trace and the compostion agree at oods_point, assuming the prover provided us
// with the proper evaluations.
pub fn verify_oods(
    oods: &[Felt],
    interaction_elements: InteractionElements,
    public_input: &PublicInput,
    constraint_coefficients: &[Felt],
    oods_point: Felt,
    trace_domain_size: Felt,
    trace_generator: Felt,
) {
    let composition_from_trace = eval_composition_polynomial(
        interaction_elements,
        public_input,
        &oods[0..oods.len() - 2],
        constraint_coefficients,
        oods_point,
        trace_domain_size,
        trace_generator,
    );

    // TODO support degree > 2?
    let claimed_composition = oods[oods.len() - 2] + oods[oods.len() - 1] * oods_point;

    assert!(composition_from_trace == claimed_composition);
}

pub fn eval_oods_boundary_poly_at_points(
    n_original_columns: usize,
    n_interaction_columns: usize,
    eval_info: OodsEvaluationInfo,
    points: &[Felt],
    decommitment: trace::Decommitment,
    composition_decommitment: table::types::Decommitment,
) -> Vec<Felt> {
    assert!(
        decommitment.original.values.len() == points.len() * n_original_columns,
        "Invalid value"
    );
    assert!(
        decommitment.interaction.values.len() == points.len() * n_interaction_columns,
        "Invalid value"
    );
    assert!(
        composition_decommitment.values.len() == points.len() * CONSTRAINT_DEGREE as usize,
        "Invalid value"
    );

    let mut evaluations = Vec::with_capacity(points.len());

    for (i, &point) in points.iter().enumerate() {
        let mut column_values = Vec::with_capacity(
            n_original_columns + n_interaction_columns + CONSTRAINT_DEGREE as usize,
        );

        column_values.extend(
            &decommitment.original.values[i * n_original_columns..(i + 1) * n_original_columns],
        );
        column_values.extend(
            &decommitment.interaction.values
                [i * n_interaction_columns..(i + 1) * n_interaction_columns],
        );
        column_values.extend(
            &composition_decommitment.values
                [i * CONSTRAINT_DEGREE as usize..(i + 1) * CONSTRAINT_DEGREE as usize],
        );

        evaluations.push(eval_oods_polynomial(
            &column_values,
            &eval_info.oods_values,
            &eval_info.constraint_coefficients,
            point,
            eval_info.oods_point,
            eval_info.trace_generator,
        ));
    }

    evaluations
}

#[cfg(test)]
mod tests {
    use super::verify_oods;
    use starknet_crypto::Felt;

    #[test]
    fn test_verify_oods() {
        // let public_input = stone_proof_fibonacci::public_input::get();
        // let interaction_elements = stone_proof_fibonacci::interaction_elements::get();
        // let mask_values = stone_proof_fibonacci::stark::oods_values::get();
        // let constraint_coefficients = stone_proof_fibonacci::constraint_coefficients::get();

        // verify_oods(
        //     mask_values,
        //     interaction_elements,
        //     public_input,
        //     constraint_coefficients.span(),
        //     Felt::from_hex_unchecked("0x47148421d376a8ca07af1e4c89890bf29c90272f63b16103646397d907281a8"),
        //     Felt::from_hex_unchecked("0x40000"),
        //     Felt::from_hex_unchecked("0x4768803ef85256034f67453635f87997ff61841e411ee63ce7b0a8b9745a046")
        // );
    }
}
