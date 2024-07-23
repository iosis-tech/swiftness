use alloc::vec::Vec;
use starknet_crypto::Felt;
#[cfg(feature = "dex")]
use swiftness_air::layout::dex::CONSTRAINT_DEGREE;
#[cfg(feature = "recursive")]
use swiftness_air::layout::recursive::CONSTRAINT_DEGREE;
#[cfg(feature = "recursive_with_poseidon")]
use swiftness_air::layout::recursive_with_poseidon::CONSTRAINT_DEGREE;
#[cfg(feature = "small")]
use swiftness_air::layout::small::CONSTRAINT_DEGREE;
#[cfg(feature = "starknet")]
use swiftness_air::layout::starknet::CONSTRAINT_DEGREE;
#[cfg(feature = "starknet_with_keccak")]
use swiftness_air::layout::starknet_with_keccak::CONSTRAINT_DEGREE;
use swiftness_air::{
    layout::{CompositionPolyEvalError, LayoutTrait},
    public_memory::PublicInput,
    trace,
};
use swiftness_commitment::table;

pub struct OodsEvaluationInfo {
    pub oods_values: Vec<Felt>,
    pub oods_point: Felt,
    pub trace_generator: Felt,
    pub constraint_coefficients: Vec<Felt>,
}

// Checks that the trace and the compostion agree at oods_point, assuming the prover provided us
// with the proper evaluations.
pub fn verify_oods<Layout: LayoutTrait>(
    oods: &[Felt],
    interaction_elements: &Layout::InteractionElements,
    public_input: &PublicInput,
    constraint_coefficients: &[Felt],
    oods_point: &Felt,
    trace_domain_size: &Felt,
    trace_generator: &Felt,
) -> Result<(), OodsVerifyError> {
    let composition_from_trace = Layout::eval_composition_polynomial(
        interaction_elements,
        public_input,
        &oods[0..oods.len() - 2],
        constraint_coefficients,
        oods_point,
        trace_domain_size,
        trace_generator,
    )?;

    // TODO support degree > 2?
    let claimed_composition = oods[oods.len() - 2] + oods[oods.len() - 1] * oods_point;

    assure!(
        composition_from_trace == claimed_composition,
        OodsVerifyError::EvaluationInvalid {
            expected: claimed_composition,
            actual: composition_from_trace
        }
    )
}

use swiftness_transcript::assure;
#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum OodsVerifyError {
    #[error("oods invalid {expected} - {actual}")]
    EvaluationInvalid { expected: Felt, actual: Felt },
    #[error("CompositionPolyEval Error")]
    CompositionPolyEvalError(#[from] CompositionPolyEvalError),
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum OodsVerifyError {
    #[error("oods invalid {expected} - {actual}")]
    EvaluationInvalid { expected: Felt, actual: Felt },
    #[error("CompositionPolyEval Error")]
    CompositionPolyEvalError(#[from] CompositionPolyEvalError),
}

pub fn eval_oods_boundary_poly_at_points<Layout: LayoutTrait>(
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

        evaluations.push(Layout::eval_oods_polynomial(
            &column_values,
            &eval_info.oods_values,
            &eval_info.constraint_coefficients,
            &point,
            &eval_info.oods_point,
            &eval_info.trace_generator,
        ));
    }

    evaluations
}
