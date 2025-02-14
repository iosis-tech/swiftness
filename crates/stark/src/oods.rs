use alloc::vec::Vec;
use funvec::{FunVec, FUNVEC_COLUMN_VALUES};
use starknet_crypto::Felt;
use swiftness_air::{
    layout::{CompositionPolyEvalError, LayoutTrait},
    public_memory::PublicInput,
    trace,
};
use swiftness_commitment::{table, CacheCommitment, CacheEvalOods};

pub struct OodsEvaluationInfo<'a> {
    pub oods_values: &'a Vec<Felt>,
    pub oods_point: &'a Felt,
    pub trace_generator: &'a Felt,
    pub constraint_coefficients: &'a Vec<Felt>,
}

// Checks that the trace and the compostion agree at oods_point, assuming the prover provided us
// with the proper evaluations.
pub fn verify_oods<Layout: LayoutTrait>(
    powers: &mut [Felt; 34],
    oods: &[Felt],
    interaction_elements: &Layout::InteractionElements,
    public_input: &PublicInput,
    constraint_coefficients: &[Felt],
    oods_point: &Felt,
    trace_domain_size: &Felt,
    trace_generator: &Felt,
) -> Result<(), OodsVerifyError> {
    let composition_from_trace = Layout::eval_composition_polynomial(
        powers,
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
    );

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

#[inline(always)]
pub fn eval_oods_boundary_poly_at_points<'a, Layout: LayoutTrait>(
    eval_oods_cache: &'a mut CacheEvalOods,
    n_original_columns: u32,
    n_interaction_columns: u32,
    public_input: &PublicInput,
    eval_info: &OodsEvaluationInfo,
    points: &[Felt],
    decommitment: &trace::Decommitment,
    composition_decommitment: &table::types::Decommitment,
) -> &'a [Felt] {
    let CacheEvalOods { powers, column_values, evaluations, .. } = eval_oods_cache;
    let evaluations = evaluations.unchecked_slice_mut(points.len());

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

    assert!(
        column_values.capacity()
            >= n_original_columns as usize
                + n_interaction_columns as usize
                + Layout::CONSTRAINT_DEGREE,
    );
    for (i, (point, evaluation)) in points.iter().zip(evaluations.iter_mut()).enumerate() {
        column_values.flush();

        column_values.extend(
            &decommitment.original.values.as_slice()
                [i * n_original_columns as usize..(i + 1) * n_original_columns as usize],
        );
        column_values.extend(
            &decommitment.interaction.values.as_slice()
                [i * n_interaction_columns as usize..(i + 1) * n_interaction_columns as usize],
        );
        column_values.extend(
            &composition_decommitment.values.as_slice()
                [i * Layout::CONSTRAINT_DEGREE..(i + 1) * Layout::CONSTRAINT_DEGREE],
        );

        *evaluation = Layout::eval_oods_polynomial(
            powers.inner(),
            public_input,
            column_values.as_slice(),
            &eval_info.oods_values,
            &eval_info.constraint_coefficients,
            &point,
            &eval_info.oods_point,
            &eval_info.trace_generator,
        )
        .unwrap();
    }

    evaluations
}
