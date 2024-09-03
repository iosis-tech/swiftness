use crate::{
    annotations::{extract::FromStrHex, Annotations},
    builtins::Builtin,
    layout::Layout,
    stark_proof::{self, *},
};
use num_bigint::BigUint;
use serde::Deserialize;
use starknet_types_core::felt::Felt;
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct StarkProof {
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
    dynamic_params: Option<BTreeMap<String, u32>>,
    layout: Layout,
    memory_segments: BTreeMap<String, MemorySegmentAddress>,
    n_steps: u32,
    public_memory: Vec<PublicMemoryElement>,
    rc_min: u32,
    rc_max: u32,
}

impl StarkProof {
    const COMPONENT_HEIGHT: u32 = 16;
    pub fn stark_config(&self) -> anyhow::Result<StarkConfig> {
        let stark = &self.proof_parameters.stark;
        let n_verifier_friendly_commitment_layers =
            self.proof_parameters.n_verifier_friendly_commitment_layers;

        let consts =
            self.public_input.layout.get_dynamics_or_consts(&self.public_input.dynamic_params);

        let log_eval_domain_size = self.log_eval_damain_size(&self.public_input.dynamic_params)?;
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

        let layer_log_sizes = self.layer_log_sizes(&self.public_input.dynamic_params)?;

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
            log_trace_domain_size: self.log_trace_domain_size(&self.public_input.dynamic_params)?,
            n_queries,
            log_n_cosets: stark.log_n_cosets,
            n_verifier_friendly_commitment_layers,
        })
    }
    fn log_trace_domain_size(
        &self,
        dynamic_params: &Option<BTreeMap<String, u32>>,
    ) -> anyhow::Result<u32> {
        let consts = self.public_input.layout.get_dynamics_or_consts(dynamic_params);
        let effective_component_height = Self::COMPONENT_HEIGHT * consts.cpu_component_step;
        log2_if_power_of_2(effective_component_height * self.public_input.n_steps)
            .ok_or(anyhow::anyhow!("Invalid cpu component step"))
    }
    fn log_eval_damain_size(
        &self,
        dynamic_params: &Option<BTreeMap<String, u32>>,
    ) -> anyhow::Result<u32> {
        Ok(self.log_trace_domain_size(dynamic_params)? + self.proof_parameters.stark.log_n_cosets)
    }
    fn layer_log_sizes(
        &self,
        dynamic_params: &Option<BTreeMap<String, u32>>,
    ) -> anyhow::Result<Vec<u32>> {
        let mut layer_log_sizes = vec![self.log_eval_damain_size(dynamic_params)?];
        for layer_step in &self.proof_parameters.stark.fri.fri_step_list {
            layer_log_sizes.push(layer_log_sizes.last().unwrap() - layer_step);
        }
        Ok(layer_log_sizes)
    }
    fn public_input(
        public_input: PublicInput,
        z: BigUint,
        alpha: BigUint,
    ) -> anyhow::Result<stark_proof::PublicInput> {
        let continuous_page_headers =
            Self::continuous_page_headers(&public_input.public_memory, z, alpha);
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
        Ok(stark_proof::PublicInput {
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
    pub fn continuous_page_headers(
        public_memory: &[PublicMemoryElement],
        z: BigUint,
        alpha: BigUint,
    ) -> Vec<BigUint> {
        let (_pages, page_prods) =
            Self::get_pages_and_products(public_memory, z.clone(), alpha.clone());

        let mut start_address: HashMap<Felt, Felt> = HashMap::new();
        let mut size: HashMap<Felt, Felt> = HashMap::new();
        let mut data: HashMap<Felt, Vec<Felt>> = HashMap::new();

        for access in public_memory {
            let page_id = Felt::from(access.page);
            let addr = Felt::from(access.address);
            let val = Felt::from_hex(&access.value).unwrap();

            start_address.entry(page_id).or_insert(addr);
            if page_id == Felt::ZERO {
                continue;
            }

            // Ensure the address is correct
            let current_size = data.entry(page_id).or_default().len();
            let expected_address = start_address.get(&page_id).unwrap() + Felt::from(current_size);
            assert_eq!(addr, expected_address);

            data.get_mut(&page_id).unwrap().push(val);
            *size.entry(page_id).or_insert(Felt::ZERO) += Felt::ONE;
        }

        let n_pages = size.len() + 1; // +1 because size does not count page 0
        assert_eq!(page_prods.len(), n_pages);

        let mut headers = Vec::new();
        let mut sorted_keys: Vec<_> = size.keys().collect();
        sorted_keys.sort(); // Ensure keys are sorted

        for (i, page_id) in sorted_keys.into_iter().enumerate() {
            let page_index = i + 1;
            assert_eq!(Felt::from(page_index), *page_id);
            let hash_value = Self::compute_hash_on_elements(data.get(page_id).unwrap());
            let header = (
                *start_address.get(page_id).unwrap(),
                *size.get(page_id).unwrap(),
                hash_value,
                *page_prods.get(page_id).unwrap(),
            );
            headers.push(header);
        }

        headers
            .into_iter()
            .flat_map(|x| {
                vec![x.0.to_biguint(), x.1.to_biguint(), x.2.to_biguint(), x.3.to_biguint()]
            })
            .collect::<Vec<BigUint>>()
    }

    // Assuming a function to compute the hash of elements
    fn compute_hash_on_elements(elements: &[Felt]) -> Felt {
        // Placeholder for the hash computation function
        // Replace this with actual hash computation logic as needed
        elements.iter().fold(Felt::from(0u64), |acc, x| acc + x)
    }
    fn get_pages_and_products(
        public_memory: &[PublicMemoryElement],
        z: BigUint,
        alpha: BigUint,
    ) -> (HashMap<Felt, Vec<Felt>>, HashMap<Felt, Felt>) {
        let mut pages = HashMap::new();
        let mut page_prods = HashMap::new();

        let z = Felt::from(z);
        let alpha = Felt::from(alpha);

        for cell in public_memory {
            let page_id = Felt::from(cell.page);
            let addr = Felt::from(cell.address);
            let val = Felt::from_hex(&cell.value).unwrap();

            // Insert or get the vector for the current page_id
            let page = pages.entry(page_id).or_insert_with(Vec::new);
            page.push(addr);
            page.push(val);

            // Calculate the product for the current page_id
            let product = z - (addr + alpha * val);
            let page_prod = page_prods.entry(page_id).or_insert(Felt::ONE);
            *page_prod *= product;
        }

        (pages, page_prods)
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

impl TryFrom<StarkProof> for stark_proof::StarkProof {
    type Error = anyhow::Error;
    fn try_from(value: StarkProof) -> anyhow::Result<Self> {
        let config = value.stark_config()?;

        let annotations = Annotations::new(
            &value.annotations.iter().map(String::as_str).collect::<Vec<_>>(),
            value.proof_parameters.stark.fri.fri_step_list.len(),
        )?;
        let public_input = StarkProof::public_input(
            value.public_input.clone(),
            annotations.z.clone(),
            annotations.alpha.clone(),
        )?;
        let unsent_commitment = value.stark_unsent_commitment(&annotations);
        let witness = value.stark_witness(&annotations);

        Ok(stark_proof::StarkProof { config, public_input, unsent_commitment, witness })
    }
}

pub fn log2_if_power_of_2(x: u32) -> Option<u32> {
    if x != 0 && (x & (x - 1)) == 0 {
        Some(f64::from(x).log2() as u32)
    } else {
        None
    }
}
