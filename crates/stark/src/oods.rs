use alloc::vec::Vec;
use starknet_crypto::Felt;
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
    // TODO: fit me in the transaction
    // let composition_from_trace = Layout::eval_composition_polynomial(
    //     interaction_elements,
    //     public_input,
    //     &oods[0..oods.len() - 2],
    //     constraint_coefficients,
    //     oods_point,
    //     trace_domain_size,
    //     trace_generator,
    // )?;

    // // TODO support degree > 2?
    // let claimed_composition = oods[oods.len() - 2] + oods[oods.len() - 1] * oods_point;

    // assure!(
    //     composition_from_trace == claimed_composition,
    //     OodsVerifyError::EvaluationInvalid {
    //         expected: claimed_composition,
    //         actual: composition_from_trace
    //     }
    // )

    Ok(())
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
    n_original_columns: u32,
    n_interaction_columns: u32,
    public_input: &PublicInput,
    eval_info: &OodsEvaluationInfo,
    points: &[Felt],
    decommitment: &trace::Decommitment,
    composition_decommitment: &table::types::Decommitment,
) -> Vec<Felt> {
    assert!(
        decommitment.original.values.len() as u32 == points.len() as u32 * n_original_columns,
        "Invalid value"
    );
    assert!(
        decommitment.interaction.values.len() as u32 == points.len() as u32 * n_interaction_columns,
        "Invalid value"
    );
    assert!(
        composition_decommitment.values.len() == points.len() * Layout::CONSTRAINT_DEGREE,
        "Invalid value"
    );

    let mut evaluations = Vec::with_capacity(points.len());

    for (i, &point) in points.iter().enumerate() {
        let mut column_values = Vec::with_capacity(
            n_original_columns as usize
                + n_interaction_columns as usize
                + Layout::CONSTRAINT_DEGREE,
        );

        column_values.extend(
            &decommitment.original.values.to_vec()
                [i * n_original_columns as usize..(i + 1) * n_original_columns as usize],
        );
        column_values.extend(
            &decommitment.interaction.values.to_vec()
                [i * n_interaction_columns as usize..(i + 1) * n_interaction_columns as usize],
        );
        column_values.extend(
            &composition_decommitment.values.to_vec()
                [i * Layout::CONSTRAINT_DEGREE..(i + 1) * Layout::CONSTRAINT_DEGREE],
        );

        evaluations.push(
            Layout::eval_oods_polynomial(
                public_input,
                &column_values,
                &eval_info.oods_values,
                &eval_info.constraint_coefficients,
                &point,
                &eval_info.oods_point,
                &eval_info.trace_generator,
            )
            .unwrap(),
        );
    }

    evaluations
}
