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

    assert_eq!(composition_from_trace, claimed_composition);
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
