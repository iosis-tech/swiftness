use num_bigint::BigUint;

use super::extract::{extract_annotations, extract_z_and_alpha};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ZAlpha {
    pub z: BigUint,
    pub alpha: BigUint,
}

impl ZAlpha {
    pub fn extract(annotations: &[&str]) -> anyhow::Result<Self> {
        extract_z_and_alpha(annotations)
    }
}

pub enum Annotation {
    OriginalCommitmentHash,
    InteractionCommitmentHash,
    CompositionCommitmentHash,
    OodsValues,
    FriLayersCommitments,
    FriLastLayerCoefficients,
    ProofOfWorkNonce,
    OriginalWitnessLeaves,
    OriginalWitnessAuthentications,
    InteractionWitnessLeaves,
    InteractionWitnessAuthentications,
    CompositionWitnessLeaves,
    CompositionWitnessAuthentications,
    FriWitnessesLeaves(usize),
    FriWitnessesAuthentications(usize),
}

impl Annotation {
    pub fn extract(&self, annotations: &[&str]) -> anyhow::Result<Vec<BigUint>> {
        let PrefixAndKind { prefix, kinds } = self.prefix_and_kinds();
        Ok(kinds
            .to_strs()
            .iter()
            .map(|k| extract_annotations(annotations, &prefix, k))
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    pub fn prefix_and_kinds(&self) -> PrefixAndKind {
        match self {
            Annotation::OriginalCommitmentHash => PrefixAndKind {
                prefix: "STARK/Original/Commit on Trace".to_string(),
                kinds: AnnotationKinds::Hash,
            },
            Annotation::InteractionCommitmentHash => PrefixAndKind {
                prefix: "STARK/Interaction/Commit on Trace".to_string(),
                kinds: AnnotationKinds::Hash,
            },
            Annotation::CompositionCommitmentHash => PrefixAndKind {
                prefix: "STARK/Out Of Domain Sampling/Commit on Trace".to_string(),
                kinds: AnnotationKinds::Hash,
            },
            Annotation::OodsValues => PrefixAndKind {
                prefix: "STARK/Out Of Domain Sampling/OODS values".to_string(),
                kinds: AnnotationKinds::FieldElements,
            },
            Annotation::FriLayersCommitments => PrefixAndKind {
                prefix: "STARK/FRI/Commitment/Layer [0-9]+".to_string(),
                kinds: AnnotationKinds::Hash,
            },
            Annotation::FriLastLayerCoefficients => PrefixAndKind {
                prefix: "STARK/FRI/Commitment/Last Layer".to_string(),
                kinds: AnnotationKinds::FieldElements,
            },
            Annotation::ProofOfWorkNonce => PrefixAndKind {
                prefix: "STARK/FRI/Proof of Work".to_string(),
                kinds: AnnotationKinds::Data,
            },
            Annotation::OriginalWitnessLeaves => PrefixAndKind {
                prefix: "STARK/FRI/Decommitment/Layer 0/Virtual Oracle/Trace 0".to_string(),
                kinds: AnnotationKinds::FieldElement,
            },
            Annotation::OriginalWitnessAuthentications => PrefixAndKind {
                prefix: "STARK/FRI/Decommitment/Layer 0/Virtual Oracle/Trace 0".to_string(),
                kinds: AnnotationKinds::DataAndHash,
            },
            Annotation::InteractionWitnessLeaves => PrefixAndKind {
                prefix: "STARK/FRI/Decommitment/Layer 0/Virtual Oracle/Trace 1".to_string(),
                kinds: AnnotationKinds::FieldElement,
            },
            Annotation::InteractionWitnessAuthentications => PrefixAndKind {
                prefix: "STARK/FRI/Decommitment/Layer 0/Virtual Oracle/Trace 1".to_string(),
                kinds: AnnotationKinds::DataAndHash,
            },
            Annotation::CompositionWitnessLeaves => PrefixAndKind {
                prefix: "STARK/FRI/Decommitment/Layer 0/Virtual Oracle/Trace 2".to_string(),
                kinds: AnnotationKinds::FieldElement,
            },
            Annotation::CompositionWitnessAuthentications => PrefixAndKind {
                prefix: "STARK/FRI/Decommitment/Layer 0/Virtual Oracle/Trace 2".to_string(),
                kinds: AnnotationKinds::DataAndHash,
            },
            Annotation::FriWitnessesLeaves(layer) => PrefixAndKind {
                prefix: format!("STARK/FRI/Decommitment/Layer {layer}"),
                kinds: AnnotationKinds::FieldElement,
            },
            Annotation::FriWitnessesAuthentications(layer) => PrefixAndKind {
                prefix: format!("STARK/FRI/Decommitment/Layer {layer}"),
                kinds: AnnotationKinds::Hash,
            },
        }
    }
}

pub enum AnnotationKinds {
    Data,
    Hash,
    FieldElement,
    FieldElements,
    DataAndHash,
}

impl AnnotationKinds {
    pub fn to_strs(&self) -> Vec<&'static str> {
        match self {
            AnnotationKinds::Data => vec!["Data"],
            AnnotationKinds::Hash => vec!["Hash"],
            AnnotationKinds::FieldElement => vec!["Field Element"],
            AnnotationKinds::FieldElements => vec!["Field Elements"],
            AnnotationKinds::DataAndHash => vec!["Data", "Hash"],
        }
    }
}

pub struct PrefixAndKind {
    pub prefix: String,
    pub kinds: AnnotationKinds,
}
