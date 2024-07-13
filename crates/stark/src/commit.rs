use cairovm_verifier_air::{
    domains::StarkDomains,
    layout::{
        recursive::{CONSTRAINT_DEGREE, MASK_SIZE, N_CONSTRAINTS},
        LayoutTrait,
    },
    public_memory::PublicInput,
};
use cairovm_verifier_commitment::table::commit::table_commit;
use cairovm_verifier_fri::fri::fri_commit;
use cairovm_verifier_pow::pow;
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

use crate::{
    config::StarkConfig,
    oods::{self, verify_oods},
    types::{StarkCommitment, StarkUnsentCommitment},
};

// STARK commitment phase.
pub fn stark_commit<Layout: LayoutTrait>(
    transcript: &mut Transcript,
    public_input: &PublicInput,
    unsent_commitment: &StarkUnsentCommitment,
    config: &StarkConfig,
    stark_domains: &StarkDomains,
) -> Result<StarkCommitment<Layout::InteractionElements>, Error> {
    // Read the commitment of the 'traces' component.
    let traces_commitment =
        Layout::traces_commit(transcript, &unsent_commitment.traces, config.traces.to_owned());

    // Generate interaction values after traces commitment.
    let composition_alpha = transcript.random_felt_to_prover();
    let traces_coefficients = powers_array(Felt::ONE, composition_alpha, N_CONSTRAINTS);

    // Read composition commitment.
    let composition_commitment =
        table_commit(transcript, unsent_commitment.composition, config.composition.to_owned());

    // Generate interaction values after composition.
    let interaction_after_composition = transcript.random_felt_to_prover();

    // Read OODS values.
    transcript.read_felt_vector_from_prover(&unsent_commitment.oods_values);

    // Check that the trace and the composition agree at oods_point.
    verify_oods::<Layout>(
        &unsent_commitment.oods_values,
        &traces_commitment.interaction_elements,
        public_input,
        &traces_coefficients,
        &interaction_after_composition,
        &stark_domains.trace_domain_size,
        &stark_domains.trace_generator,
    )?;

    // Generate interaction values after OODS.
    let oods_alpha = transcript.random_felt_to_prover();
    let oods_coefficients = powers_array(Felt::ONE, oods_alpha, MASK_SIZE + CONSTRAINT_DEGREE);

    // Read fri commitment.
    let fri_commitment =
        fri_commit(transcript, unsent_commitment.fri.to_owned(), config.fri.to_owned());

    // Proof of work commitment phase.
    unsent_commitment.proof_of_work.commit(transcript, &config.proof_of_work)?;

    // Return commitment.
    Ok(StarkCommitment {
        traces: traces_commitment,
        composition: composition_commitment,
        interaction_after_composition,
        oods_values: unsent_commitment.oods_values.to_owned(),
        interaction_after_oods: oods_coefficients,
        fri: fri_commitment,
    })
}

fn powers_array(initial: Felt, alpha: Felt, n: u32) -> Vec<Felt> {
    let mut array = Vec::with_capacity(n as usize);
    let mut value = initial;

    for _ in 0..n {
        array.push(value);
        value *= alpha;
    }

    array
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("POW Error")]
    POW(#[from] pow::Error),

    #[error("OodsVerifyError Error")]
    Oods(#[from] oods::OodsVerifyError),
}
