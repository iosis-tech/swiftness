use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;
use swiftness_transcript::transcript::Transcript;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EcPoint {
    #[serde_as(as = "UfeHex")]
    pub x: Felt,
    #[serde_as(as = "UfeHex")]
    pub y: Felt,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CurveConfig {
    #[serde_as(as = "UfeHex")]
    pub alpha: Felt,
    #[serde_as(as = "UfeHex")]
    pub beta: Felt,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EcdsaSigConfig {
    #[serde_as(as = "UfeHex")]
    pub alpha: Felt,
    #[serde_as(as = "UfeHex")]
    pub beta: Felt,
    pub shift_point: EcPoint,
}

// Accumulation of member expressions for auto generated composition polynomial code.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalValues {
    // Public input.
    #[serde_as(as = "UfeHex")]
    pub trace_length: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_pc: Felt,
    #[serde_as(as = "UfeHex")]
    pub final_pc: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_ap: Felt,
    #[serde_as(as = "UfeHex")]
    pub final_ap: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_pedersen_addr: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_range_check_addr: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_ecdsa_addr: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_bitwise_addr: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_ec_op_addr: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_keccak_addr: Felt,
    #[serde_as(as = "UfeHex")]
    pub initial_poseidon_addr: Felt,
    #[serde_as(as = "UfeHex")]
    pub range_check_min: Felt,
    #[serde_as(as = "UfeHex")]
    pub range_check_max: Felt,
    // Constants.
    #[serde_as(as = "UfeHex")]
    pub offset_size: Felt,
    #[serde_as(as = "UfeHex")]
    pub half_offset_size: Felt,
    pub pedersen_shift_point: EcPoint,
    pub ecdsa_sig_config: EcdsaSigConfig,
    pub ec_op_curve_config: CurveConfig,
    // Periodic columns.
    #[serde_as(as = "UfeHex")]
    pub pedersen_points_x: Felt,
    #[serde_as(as = "UfeHex")]
    pub pedersen_points_y: Felt,
    #[serde_as(as = "UfeHex")]
    pub ecdsa_generator_points_x: Felt,
    #[serde_as(as = "UfeHex")]
    pub ecdsa_generator_points_y: Felt,
    #[serde_as(as = "UfeHex")]
    pub keccak_keccak_keccak_round_key0: Felt,
    #[serde_as(as = "UfeHex")]
    pub keccak_keccak_keccak_round_key1: Felt,
    #[serde_as(as = "UfeHex")]
    pub keccak_keccak_keccak_round_key3: Felt,
    #[serde_as(as = "UfeHex")]
    pub keccak_keccak_keccak_round_key7: Felt,
    #[serde_as(as = "UfeHex")]
    pub keccak_keccak_keccak_round_key15: Felt,
    #[serde_as(as = "UfeHex")]
    pub keccak_keccak_keccak_round_key31: Felt,
    #[serde_as(as = "UfeHex")]
    pub keccak_keccak_keccak_round_key63: Felt,
    #[serde_as(as = "UfeHex")]
    pub poseidon_poseidon_full_round_key0: Felt,
    #[serde_as(as = "UfeHex")]
    pub poseidon_poseidon_full_round_key1: Felt,
    #[serde_as(as = "UfeHex")]
    pub poseidon_poseidon_full_round_key2: Felt,
    #[serde_as(as = "UfeHex")]
    pub poseidon_poseidon_partial_round_key0: Felt,
    #[serde_as(as = "UfeHex")]
    pub poseidon_poseidon_partial_round_key1: Felt,
    // Interaction elements.
    #[serde_as(as = "UfeHex")]
    pub memory_multi_column_perm_perm_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub memory_multi_column_perm_hash_interaction_elm0: Felt,
    #[serde_as(as = "UfeHex")]
    pub range_check16_perm_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_permutation_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_interaction_z: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_interaction_alpha: Felt,
    // Permutation products.
    #[serde_as(as = "UfeHex")]
    pub memory_multi_column_perm_perm_public_memory_prod: Felt,
    #[serde_as(as = "UfeHex")]
    pub range_check16_perm_public_memory_prod: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_first_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_permutation_public_memory_prod: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_final_cum_val: Felt,
}

// Elements that are sent from the prover after the commitment on the original trace.
// Used for components after the first interaction, e.g., memory and range check.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionElements {
    #[serde_as(as = "UfeHex")]
    pub memory_multi_column_perm_perm_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub memory_multi_column_perm_hash_interaction_elm0: Felt,
    #[serde_as(as = "UfeHex")]
    pub range_check16_perm_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_permutation_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_interaction_z: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_interaction_alpha: Felt,
}

impl InteractionElements {
    pub fn new(transcript: &mut Transcript) -> Self {
        Self {
            memory_multi_column_perm_perm_interaction_elm: transcript.random_felt_to_prover(),
            memory_multi_column_perm_hash_interaction_elm0: transcript.random_felt_to_prover(),
            range_check16_perm_interaction_elm: transcript.random_felt_to_prover(),
            diluted_check_permutation_interaction_elm: transcript.random_felt_to_prover(),
            diluted_check_interaction_z: transcript.random_felt_to_prover(),
            diluted_check_interaction_alpha: transcript.random_felt_to_prover(),
        }
    }
}
