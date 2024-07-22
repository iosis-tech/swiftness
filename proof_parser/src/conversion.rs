use swiftness_air::{
    public_memory::PublicInput as PublicInputVerifier,
    trace::{
        config::Config as TraceConfigVerifier, Decommitment as TraceDecommitmentVerifier,
        UnsentCommitment as TraceUnsentCommitmentVerifier, Witness as TraceWitnessVerifier,
    },
    types::{AddrValue, Page, SegmentInfo as SegmentInfoVerifier},
};
use swiftness_commitment::{
    table::{
        config::Config as TableConfigVerifier,
        types::{
            Decommitment as TableDecommitmentVerifier, Witness as TableCommitmentWitnessVerifier,
        },
    },
    vector::{
        config::Config as VectorConfigVerifier, types::Witness as VectorCommitmentWitnessVerifier,
    },
};
use swiftness_fri::{
    config::Config as FriConfigVerifier,
    types::{
        LayerWitness, UnsentCommitment as FriUnsentCommitmentVerifier,
        Witness as FriWitnessVerifier,
    },
};
use swiftness_pow::{
    config::Config as PowConfigVerifier, pow::UnsentCommitment as PowUnsentCommitmentVerifier,
};
use swiftness_stark::{
    config::StarkConfig as StarkConfigVerifier,
    types::{
        StarkProof as StarkProofVerifier, StarkUnsentCommitment as StarkUnsentCommitmentVerifier,
        StarkWitness as StarkWitnessVerifier,
    },
};

use crate::stark_proof::*;

impl From<StarkProof> for StarkProofVerifier {
    fn from(proof: StarkProof) -> Self {
        StarkProofVerifier {
            config: proof.config.into(),
            public_input: proof.public_input.into(),
            unsent_commitment: proof.unsent_commitment.into(),
            witness: proof.witness.into(),
        }
    }
}

impl From<StarkConfig> for StarkConfigVerifier {
    fn from(config: StarkConfig) -> Self {
        StarkConfigVerifier {
            traces: config.traces.into(),
            composition: config.composition.into(),
            fri: config.fri.into(),
            proof_of_work: config.proof_of_work.into(),
            log_trace_domain_size: config.log_trace_domain_size.into(),
            n_queries: config.n_queries.into(),
            log_n_cosets: config.log_n_cosets.into(),
            n_verifier_friendly_commitment_layers: config
                .n_verifier_friendly_commitment_layers
                .into(),
        }
    }
}

impl From<ProofOfWorkConfig> for PowConfigVerifier {
    fn from(pow: ProofOfWorkConfig) -> Self {
        PowConfigVerifier { n_bits: pow.n_bits as u8 }
    }
}

impl From<FriConfig> for FriConfigVerifier {
    fn from(fri: FriConfig) -> Self {
        FriConfigVerifier {
            log_input_size: fri.log_input_size.into(),
            n_layers: fri.n_layers.into(),
            inner_layers: fri.inner_layers.into_iter().map(|x| x.into()).collect(),
            fri_step_sizes: fri.fri_step_sizes.into_iter().map(|x| x.into()).collect(),
            log_last_layer_degree_bound: fri.log_last_layer_degree_bound.into(),
        }
    }
}

impl From<TracesConfig> for TraceConfigVerifier {
    fn from(traces: TracesConfig) -> Self {
        TraceConfigVerifier {
            original: traces.original.into(),
            interaction: traces.interaction.into(),
        }
    }
}

impl From<TableCommitmentConfig> for TableConfigVerifier {
    fn from(config: TableCommitmentConfig) -> Self {
        TableConfigVerifier { n_columns: config.n_columns.into(), vector: config.vector.into() }
    }
}

impl From<VectorCommitmentConfig> for VectorConfigVerifier {
    fn from(vector: VectorCommitmentConfig) -> Self {
        VectorConfigVerifier {
            height: vector.height.into(),
            n_verifier_friendly_commitment_layers: vector
                .n_verifier_friendly_commitment_layers
                .into(),
        }
    }
}

// ==================================================================================================

impl From<CairoPublicInput> for PublicInputVerifier {
    fn from(public_input: CairoPublicInput) -> Self {
        PublicInputVerifier {
            log_n_steps: public_input.log_n_steps.into(),
            range_check_min: public_input.range_check_min.into(),
            range_check_max: public_input.range_check_max.into(),
            layout: public_input.layout.into(),
            dynamic_params: public_input.dynamic_params.values().map(|x| x.into()).collect(),
            segments: public_input.segments.into_iter().map(|x| x.into()).collect(),
            padding_addr: public_input.padding_addr.into(),
            padding_value: public_input.padding_value.into(),
            main_page: Page(public_input.main_page.into_iter().map(|x| x.into()).collect()),
            continuous_page_headers: vec![],
        }
    }
}

impl From<SegmentInfo> for SegmentInfoVerifier {
    fn from(segment_info: SegmentInfo) -> Self {
        SegmentInfoVerifier {
            begin_addr: segment_info.begin_addr.into(),
            stop_ptr: segment_info.stop_ptr.into(),
        }
    }
}

impl From<PubilcMemoryCell> for AddrValue {
    fn from(cell: PubilcMemoryCell) -> Self {
        AddrValue { address: cell.address.into(), value: cell.value.into() }
    }
}

// =================================================================================================

impl From<StarkUnsentCommitment> for StarkUnsentCommitmentVerifier {
    fn from(unsent_commitment: StarkUnsentCommitment) -> Self {
        StarkUnsentCommitmentVerifier {
            traces: unsent_commitment.traces.into(),
            composition: unsent_commitment.composition.into(),
            oods_values: unsent_commitment.oods_values.into_iter().map(|x| x.into()).collect(),
            fri: unsent_commitment.fri.into(),
            proof_of_work: unsent_commitment.proof_of_work.into(),
        }
    }
}

impl From<TracesUnsentCommitment> for TraceUnsentCommitmentVerifier {
    fn from(traces: TracesUnsentCommitment) -> Self {
        TraceUnsentCommitmentVerifier {
            original: traces.original.into(),
            interaction: traces.interaction.into(),
        }
    }
}

impl From<FriUnsentCommitment> for FriUnsentCommitmentVerifier {
    fn from(fri: FriUnsentCommitment) -> Self {
        FriUnsentCommitmentVerifier {
            last_layer_coefficients: fri
                .last_layer_coefficients
                .into_iter()
                .map(|x| x.into())
                .collect(),
            inner_layers: fri.inner_layers.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<ProofOfWorkUnsentCommitment> for PowUnsentCommitmentVerifier {
    fn from(pow: ProofOfWorkUnsentCommitment) -> Self {
        PowUnsentCommitmentVerifier { nonce: pow.nonce.to_u64_digits()[0] }
    }
}

// =================================================================================================

impl From<StarkWitness> for StarkWitnessVerifier {
    fn from(witness: StarkWitness) -> Self {
        StarkWitnessVerifier {
            traces_decommitment: witness.traces_decommitment.into(),
            traces_witness: witness.traces_witness.into(),
            composition_decommitment: witness.composition_decommitment.into(),
            composition_witness: witness.composition_witness.into(),
            fri_witness: witness.fri_witness.into(),
        }
    }
}

impl From<TracesDecommitment> for TraceDecommitmentVerifier {
    fn from(traces: TracesDecommitment) -> Self {
        TraceDecommitmentVerifier {
            original: traces.original.into(),
            interaction: traces.interaction.into(),
        }
    }
}

impl From<TableDecommitment> for TableDecommitmentVerifier {
    fn from(table: TableDecommitment) -> Self {
        TableDecommitmentVerifier { values: table.values.into_iter().map(|x| x.into()).collect() }
    }
}

impl From<TracesWitness> for TraceWitnessVerifier {
    fn from(traces: TracesWitness) -> Self {
        TraceWitnessVerifier {
            original: traces.original.into(),
            interaction: traces.interaction.into(),
        }
    }
}

impl From<TableCommitmentWitness> for TableCommitmentWitnessVerifier {
    fn from(table: TableCommitmentWitness) -> Self {
        TableCommitmentWitnessVerifier { vector: table.vector.into() }
    }
}

impl From<VectorCommitmentWitness> for VectorCommitmentWitnessVerifier {
    fn from(vector: VectorCommitmentWitness) -> Self {
        VectorCommitmentWitnessVerifier {
            authentications: vector.authentications.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<FriWitness> for FriWitnessVerifier {
    fn from(fri: FriWitness) -> Self {
        FriWitnessVerifier { layers: fri.layers.into_iter().map(|x| x.into()).collect() }
    }
}

impl From<FriLayerWitness> for LayerWitness {
    fn from(layer: FriLayerWitness) -> Self {
        LayerWitness {
            leaves: layer.leaves.into_iter().map(|x| x.into()).collect(),
            table_witness: layer.table_witness.into(),
        }
    }
}

impl From<TableCommitmentWitnessFlat> for TableCommitmentWitnessVerifier {
    fn from(table: TableCommitmentWitnessFlat) -> Self {
        TableCommitmentWitnessVerifier { vector: table.vector.into() }
    }
}

impl From<VectorCommitmentWitnessFlat> for VectorCommitmentWitnessVerifier {
    fn from(vector: VectorCommitmentWitnessFlat) -> Self {
        VectorCommitmentWitnessVerifier {
            authentications: vector.authentications.into_iter().map(|x| x.into()).collect(),
        }
    }
}
