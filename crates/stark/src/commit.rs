use alloc::vec::Vec;
use funvec::FunBox;
use starknet_crypto::Felt;
use swiftness_air::{domains::StarkDomains, layout::LayoutTrait, public_memory::PublicInput};
use swiftness_commitment::{table::commit::table_commit, CacheCommitment};
use swiftness_fri::fri::fri_commit;
use swiftness_pow::pow;
use swiftness_transcript::transcript::Transcript;

// STARK commitment phase.
pub fn stark_commit<Layout: LayoutTrait>(
    cache: &mut Cache,
    transcript: &mut Transcript,
    public_input: &PublicInput,
    unsent_commitment: &StarkUnsentCommitment,
    config: &StarkConfig,
    stark_domains: &StarkDomains,
) -> Result<StarkCommitment<Layout::InteractionElements>, Error> {
    // Read the commitment of the 'traces' component.

    let traces_commitment = FunBox::new(Layout::traces_commit(
        transcript,
        &unsent_commitment.traces,
        config.traces.clone(),
    ));

    // Generate interaction values after traces commitment.
    let composition_alpha = transcript.random_felt_to_prover();
    let traces_coefficients =
        powers_array(Felt::ONE, composition_alpha, Layout::N_CONSTRAINTS as u32);

    // Read composition commitment.
    let composition_commitment = FunBox::new(table_commit(
        transcript,
        unsent_commitment.composition,
        config.composition.clone(),
    ));

    // Generate interaction values after composition.
    let interaction_after_composition = transcript.random_felt_to_prover();

    // Read OODS values.
    transcript.read_felt_vector_from_prover(&unsent_commitment.oods_values.to_vec());

    // Check that the trace and the composition agree at oods_point.
    // TODO: uncomment
    verify_oods::<Layout>(
        cache.commitment.verify_oods.inner(),
        unsent_commitment.oods_values.as_slice(),
        &traces_commitment.interaction_elements,
        public_input,
        &traces_coefficients,
        &interaction_after_composition,
        &stark_domains.trace_domain_size,
        &stark_domains.trace_generator,
    )?;

    // Generate interaction values after OODS.
    let oods_alpha = transcript.random_felt_to_prover();

    let oods_coefficients =
        powers_array(Felt::ONE, oods_alpha, (Layout::MASK_SIZE + Layout::CONSTRAINT_DEGREE) as u32);

    // Read fri commitment.
    let fri_commitment =
        FunBox::new(fri_commit(transcript, unsent_commitment.fri.clone(), config.fri.clone()));

    // Proof of work commitment phase.
    unsent_commitment.proof_of_work.commit(transcript, &config.proof_of_work)?;

    // Return commitment.
    Ok(StarkCommitment {
        traces: traces_commitment,
        composition: composition_commitment,
        interaction_after_composition,
        oods_values: unsent_commitment.oods_values.to_vec(),
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

use crate::{
    config::StarkConfig,
    oods::{self, verify_oods},
    types::{Cache, StarkCommitment, StarkUnsentCommitment},
};

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("POW Error")]
    POW(#[from] pow::Error),

    #[error("OodsVerifyError Error")]
    Oods(#[from] oods::OodsVerifyError),
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("POW Error")]
    POW(#[from] pow::Error),

    #[error("OodsVerifyError Error")]
    Oods(#[from] oods::OodsVerifyError),
}
