use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
};

use num_bigint::BigUint;
use serde::Deserialize;

use crate::{
    annotations::{extract::FromStrHex, Annotations},
    builtins::Builtin,
    layout::Layout,
    stark_proof::*,
    utils::log2_if_power_of_2,
};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ProofJSON {
    proof_parameters: ProofParameters,
    annotations: Vec<String>,
    public_input: PublicInput,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ProofParameters {
    pub stark: Stark,
    #[serde(default)]
    pub n_verifier_friendly_commitment_layers: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Stark {
    pub fri: Fri,
    pub log_n_cosets: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Fri {
    pub fri_step_list: Vec<u32>,
    pub last_layer_degree_bound: u32,
    pub n_queries: u32,
    pub proof_of_work_bits: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MemorySegmentAddress {
    begin_addr: u32,
    stop_ptr: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PublicMemoryElement {
    address: u32,
    page: u32,
    value: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PublicInput {
    dynamic_params: Option<BTreeMap<String, BigUint>>,
    layout: Layout,
    memory_segments: HashMap<String, MemorySegmentAddress>,
    n_steps: u32,
    public_memory: Vec<PublicMemoryElement>,
    rc_min: u32,
    rc_max: u32,
}

impl ProofJSON {
    const COMPONENT_HEIGHT: u32 = 16;
    pub fn stark_config(&self) -> anyhow::Result<StarkConfig> {
        let stark = &self.proof_parameters.stark;
        let n_verifier_friendly_commitment_layers =
            self.proof_parameters.n_verifier_friendly_commitment_layers;

        let consts = match self
            .public_input
            .layout
            .get_dynamics_or_consts(&self.public_input.dynamic_params)
        {
            Some(c) => c,
            None => anyhow::bail!(
                "There were some constant overrides in the dynamic params but couldn't be parsed!"
            ),
        };

        let log_eval_domain_size = self.log_eval_damain_size()?;
        let traces = TracesConfig {
            original: TableCommitmentConfig {
                n_columns: consts.num_columns_first,
                vector: VectorCommitmentConfig {
                    height: log_eval_domain_size,
                    n_verifier_friendly_commitment_layers,
                },
            },
            interaction: TableCommitmentConfig {
                n_columns: consts.num_columns_second,
                vector: VectorCommitmentConfig {
                    height: log_eval_domain_size,
                    n_verifier_friendly_commitment_layers,
                },
            },
        };

        let composition = TableCommitmentConfig {
            n_columns: consts.constraint_degree,
            vector: VectorCommitmentConfig {
                height: log_eval_domain_size,
                n_verifier_friendly_commitment_layers,
            },
        };

        let fri = self.proof_parameters.stark.fri.clone();

        let proof_of_work = ProofOfWorkConfig { n_bits: fri.proof_of_work_bits };
        let n_queries = fri.n_queries;

        let layer_log_sizes = self.layer_log_sizes()?;

        let fri_step_list = fri.fri_step_list;
        let log_last_layer_degree_bound = log2_if_power_of_2(fri.last_layer_degree_bound)
            .ok_or(anyhow::anyhow!("Invalid last layer degree bound"))?;
        let fri = FriConfig {
            log_input_size: layer_log_sizes[0],
            n_layers: fri_step_list.len() as u32,
            inner_layers: fri_step_list[1..]
                .iter()
                .zip(layer_log_sizes[2..].iter())
                .map(|(layer_steps, layer_log_rows)| TableCommitmentConfig {
                    n_columns: 2_u32.pow(*layer_steps),
                    vector: VectorCommitmentConfig {
                        height: *layer_log_rows,
                        n_verifier_friendly_commitment_layers,
                    },
                })
                .collect(),
            fri_step_sizes: fri_step_list,
            log_last_layer_degree_bound,
        };

        Ok(StarkConfig {
            traces,
            composition,
            fri,
            proof_of_work,
            log_trace_domain_size: self.log_trace_domain_size()?,
            n_queries,
            log_n_cosets: stark.log_n_cosets,
            n_verifier_friendly_commitment_layers,
        })
    }
    fn log_trace_domain_size(&self) -> anyhow::Result<u32> {
        let consts = self.public_input.layout.get_consts();
        let effective_component_height = Self::COMPONENT_HEIGHT * consts.cpu_component_step;
        log2_if_power_of_2(effective_component_height * self.public_input.n_steps)
            .ok_or(anyhow::anyhow!("Invalid cpu component step"))
    }
    fn log_eval_damain_size(&self) -> anyhow::Result<u32> {
        Ok(self.log_trace_domain_size()? + self.proof_parameters.stark.log_n_cosets)
    }
    fn layer_log_sizes(&self) -> anyhow::Result<Vec<u32>> {
        let mut layer_log_sizes = vec![self.log_eval_damain_size()?];
        for layer_step in &self.proof_parameters.stark.fri.fri_step_list {
            layer_log_sizes.push(layer_log_sizes.last().unwrap() - layer_step);
        }
        Ok(layer_log_sizes)
    }
    fn public_input(
        public_input: PublicInput,
        z: BigUint,
        alpha: BigUint,
    ) -> anyhow::Result<CairoPublicInput> {
        let continuous_page_headers =
            Self::continuous_page_headers(&public_input.public_memory, z, alpha)?;
        let main_page = Self::main_page(&public_input.public_memory)?;
        let dynamic_params = public_input.dynamic_params.unwrap_or_default();
        let memory_segments = Builtin::sort_segments(public_input.memory_segments)
            .into_iter()
            .map(|s| SegmentInfo { begin_addr: s.begin_addr, stop_ptr: s.stop_ptr })
            .collect::<Vec<_>>();
        let layout = BigUint::from_bytes_be(&public_input.layout.bytes_encode());
        let (padding_addr, padding_value) = match public_input.public_memory.first() {
            Some(m) => (
                m.address,
                BigUint::from_str_hex(&m.value).ok_or(anyhow::anyhow!("Invalid memory value"))?,
            ),
            None => anyhow::bail!("Invalid public memory"),
        };
        Ok(CairoPublicInput {
            log_n_steps: log2_if_power_of_2(public_input.n_steps)
                .ok_or(anyhow::anyhow!("Invalid number of steps"))?,
            range_check_min: public_input.rc_min,
            range_check_max: public_input.rc_max,
            layout,
            dynamic_params: dynamic_params.into_iter().collect(),
            n_segments: memory_segments.len(),
            segments: memory_segments,
            padding_addr,
            padding_value,
            main_page_len: main_page.len(),
            main_page,
            n_continuous_pages: continuous_page_headers.len(),
            continuous_page_headers,
        })
    }
    fn main_page(public_memory: &[PublicMemoryElement]) -> anyhow::Result<Vec<PubilcMemoryCell>> {
        public_memory
            .iter()
            .filter(|m| m.page == 0)
            .map(|m| {
                Ok(PubilcMemoryCell {
                    address: m.address,
                    value: BigUint::from_str_hex(&m.value)
                        .ok_or(anyhow::anyhow!("Invalid memory value"))?,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }
    fn continuous_page_headers(
        _public_memory: &[PublicMemoryElement],
        _z: BigUint,
        _alpha: BigUint,
    ) -> anyhow::Result<Vec<BigUint>> {
        //TODO: Do it properly
        Ok(vec![])
    }
    fn stark_unsent_commitment(&self, annotations: &Annotations) -> StarkUnsentCommitment {
        StarkUnsentCommitment {
            traces: TracesUnsentCommitment {
                original: annotations.original_commitment_hash.clone(),
                interaction: annotations.interaction_commitment_hash.clone(),
            },
            composition: annotations.composition_commitment_hash.clone(),
            oods_values: annotations.oods_values.clone(),
            fri: FriUnsentCommitment {
                inner_layers: annotations.fri_layers_commitments.clone(),
                last_layer_coefficients: annotations.fri_last_layer_coefficients.clone(),
            },
            proof_of_work: ProofOfWorkUnsentCommitment {
                nonce: annotations.proof_of_work_nonce.clone(),
            },
        }
    }
    fn stark_witness(&self, annotations: &Annotations) -> StarkWitness {
        StarkWitness {
            traces_decommitment: TracesDecommitment {
                original: TableDecommitment {
                    n_values: annotations.original_witness_leaves.len(),
                    values: annotations.original_witness_leaves.clone(),
                },
                interaction: TableDecommitment {
                    n_values: annotations.interaction_witness_leaves.len(),
                    values: annotations.interaction_witness_leaves.clone(),
                },
            },
            traces_witness: TracesWitness {
                original: TableCommitmentWitness {
                    vector: VectorCommitmentWitness {
                        n_authentications: annotations.original_witness_authentications.len(),
                        authentications: annotations.original_witness_authentications.clone(),
                    },
                },
                interaction: TableCommitmentWitness {
                    vector: VectorCommitmentWitness {
                        n_authentications: annotations.interaction_witness_authentications.len(),
                        authentications: annotations.interaction_witness_authentications.clone(),
                    },
                },
            },
            composition_decommitment: TableDecommitment {
                n_values: annotations.composition_witness_leaves.len(),
                values: annotations.composition_witness_leaves.clone(),
            },
            composition_witness: TableCommitmentWitness {
                vector: VectorCommitmentWitness {
                    n_authentications: annotations.composition_witness_authentications.len(),
                    authentications: annotations.composition_witness_authentications.clone(),
                },
            },
            fri_witness: FriWitness {
                layers: annotations
                    .fri_witnesses
                    .iter()
                    .map(|w| FriLayerWitness {
                        n_leaves: w.leaves.len(),
                        leaves: w.leaves.clone(),
                        table_witness: TableCommitmentWitnessFlat {
                            vector: VectorCommitmentWitnessFlat {
                                n_authentications: w.authentications.len(),
                                authentications: w.authentications.clone(),
                            },
                        },
                    })
                    .collect(),
            },
        }
    }
}

impl TryFrom<ProofJSON> for StarkProof {
    type Error = anyhow::Error;
    fn try_from(value: ProofJSON) -> anyhow::Result<Self> {
        let config = value.stark_config()?;

        let annotations = Annotations::new(
            &value.annotations.iter().map(String::as_str).collect::<Vec<_>>(),
            value.proof_parameters.stark.fri.fri_step_list.len(),
        )?;
        let public_input = ProofJSON::public_input(
            value.public_input.clone(),
            annotations.z.clone(),
            annotations.alpha.clone(),
        )?;
        let unsent_commitment = value.stark_unsent_commitment(&annotations);
        let witness = value.stark_witness(&annotations);

        Ok(StarkProof { config, public_input, unsent_commitment, witness })
    }
}
