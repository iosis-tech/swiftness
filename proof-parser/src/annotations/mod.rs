pub mod annotation_kind;
pub mod extract;

use self::annotation_kind::{Annotation, ZAlpha};
use alloc::vec::Vec;
use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq)]
pub struct Annotations {
    pub z: BigUint,
    pub alpha: BigUint,
    pub original_commitment_hash: BigUint,
    pub interaction_commitment_hash: BigUint,
    pub composition_commitment_hash: BigUint,
    pub oods_values: Vec<BigUint>,
    pub fri_layers_commitments: Vec<BigUint>,
    pub fri_last_layer_coefficients: Vec<BigUint>,
    pub proof_of_work_nonce: BigUint,
    pub original_witness_leaves: Vec<BigUint>,
    pub original_witness_authentications: Vec<BigUint>,
    pub interaction_witness_leaves: Vec<BigUint>,
    pub interaction_witness_authentications: Vec<BigUint>,
    pub composition_witness_leaves: Vec<BigUint>,
    pub composition_witness_authentications: Vec<BigUint>,
    pub fri_witnesses: Vec<FriWitness>,
}

impl Annotations {
    pub fn new(annotations: &[&str], n_fri_layers: usize) -> anyhow::Result<Annotations> {
        let ZAlpha { z, alpha } = ZAlpha::extract(annotations)?;
        Ok(Annotations {
            z,
            alpha,
            original_commitment_hash: Annotation::OriginalCommitmentHash
                .extract(annotations)?
                .first()
                .ok_or(anyhow::anyhow!("No OriginalCommitmentHash in annotations!"))?
                .clone(),
            interaction_commitment_hash: Annotation::InteractionCommitmentHash
                .extract(annotations)?
                .first()
                .ok_or(anyhow::anyhow!("No InteractionCommitmentHash in annotations!"))?
                .clone(),
            composition_commitment_hash: Annotation::CompositionCommitmentHash
                .extract(annotations)?
                .first()
                .ok_or(anyhow::anyhow!("No CompositionCommitmentHash in annotations!"))?
                .clone(),
            oods_values: Annotation::OodsValues.extract(annotations)?,
            fri_layers_commitments: Annotation::FriLayersCommitments.extract(annotations)?,
            fri_last_layer_coefficients: Annotation::FriLastLayerCoefficients
                .extract(annotations)?,
            proof_of_work_nonce: Annotation::ProofOfWorkNonce
                .extract(annotations)?
                .first()
                .ok_or(anyhow::anyhow!("No ProofOfWorkNonce in annotations!"))?
                .clone(),
            original_witness_leaves: Annotation::OriginalWitnessLeaves.extract(annotations)?,
            original_witness_authentications: Annotation::OriginalWitnessAuthentications
                .extract(annotations)?,
            interaction_witness_leaves: Annotation::InteractionWitnessLeaves
                .extract(annotations)?,
            interaction_witness_authentications: Annotation::InteractionWitnessAuthentications
                .extract(annotations)?,
            composition_witness_leaves: Annotation::CompositionWitnessLeaves
                .extract(annotations)?,
            composition_witness_authentications: Annotation::CompositionWitnessAuthentications
                .extract(annotations)?,
            fri_witnesses: (1..n_fri_layers)
                .map(|i| {
                    Ok(FriWitness {
                        layer: i,
                        leaves: Annotation::FriWitnessesLeaves(i).extract(annotations)?,
                        authentications: Annotation::FriWitnessesAuthentications(i)
                            .extract(annotations)?,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FriWitness {
    pub layer: usize,
    pub leaves: Vec<BigUint>,
    pub authentications: Vec<BigUint>,
}
