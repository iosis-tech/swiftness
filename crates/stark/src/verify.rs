use starknet_crypto::Felt;
use swiftness_air::{domains::StarkDomains, layout::LayoutTrait};
use swiftness_commitment::table::decommit::table_decommit;
use swiftness_fri::{
    fri::{self, fri_verify},
    types,
};

use crate::{
    oods::{eval_oods_boundary_poly_at_points, OodsEvaluationInfo},
    queries::queries_to_points,
    types::{StarkCommitment, StarkWitness},
};

// STARK verify phase.
pub fn stark_verify<Layout: LayoutTrait>(
    n_original_columns: usize,
    n_interaction_columns: usize,
    queries: &[Felt],
    commitment: StarkCommitment<Layout::InteractionElements>,
    witness: &StarkWitness,
    stark_domains: &StarkDomains,
) -> Result<(), Error> {
    // First layer decommit.
    Layout::traces_decommit(
        queries,
        commitment.traces,
        witness.traces_decommitment.to_owned(),
        witness.traces_witness.to_owned(),
    )?;

    table_decommit(
        commitment.composition,
        queries,
        witness.composition_decommitment.to_owned(),
        witness.composition_witness.to_owned(),
    )?;

    // Compute query points.
    let points = queries_to_points(queries, stark_domains);

    // Evaluate the FRI input layer at query points.
    let eval_info = OodsEvaluationInfo {
        oods_values: commitment.oods_values,
        oods_point: commitment.interaction_after_composition,
        trace_generator: stark_domains.trace_generator,
        constraint_coefficients: commitment.interaction_after_oods,
    };
    let oods_poly_evals = eval_oods_boundary_poly_at_points::<Layout>(
        n_original_columns,
        n_interaction_columns,
        eval_info,
        &points,
        witness.traces_decommitment.to_owned(),
        witness.composition_decommitment.to_owned(),
    );

    // Decommit FRI.
    let fri_decommitment = types::Decommitment { values: oods_poly_evals, points };
    Ok(fri_verify(queries, commitment.fri, fri_decommitment, witness.fri_witness.to_owned())?)
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Fri Error")]
    FriError(#[from] fri::Error),

    #[error("TraceDecommit Error")]
    TraceDecommitError(#[from] swiftness_air::trace::decommit::Error),

    #[error("TableDecommit Error")]
    TableDecommitError(#[from] swiftness_commitment::table::decommit::Error),
}
