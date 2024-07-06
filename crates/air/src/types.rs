pub mod trace;

use cairo_felt::Felt252;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SegmentInfo {
    // Start address of the memory segment.
    begin_addr: Felt252,
    // Stop pointer of the segment - not necessarily the end of the segment.
    stop_ptr: Felt252,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicInput {
    log_n_steps: Felt252,
    range_check_min: Felt252,
    range_check_max: Felt252,
    layout: Felt252,
    dynamic_params: Vec<Felt252>,
    segments: Vec<SegmentInfo>,
    padding_addr: Felt252,
    padding_value: Felt252,
    main_page: Page,
    continuous_page_headers: Vec<ContinuousPageHeader>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddrValue {
    address: Felt252,
    value: Felt252,
}

pub type Page = Vec<AddrValue>;

// Information about a continuous page (a consecutive section of the public memory)..
// Each such page must be verified externally to the verifier:
//   hash = Hash(
//     memory[start_address], memory[start_address + 1], ..., memory[start_address + size - 1]).
//   prod = prod_i (z - ((start_address + i) + alpha * (memory[start_address + i]))).
// z, alpha are taken from the interaction values, and can be obtained directly from the
// StarkProof object.
//   z     = interaction_elements.memory_multi_column_perm_perm__interaction_elm
//   alpha = interaction_elements.memory_multi_column_perm_hash_interaction_elm0
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinuousPageHeader {
    // Start address.
    start_address: Felt252,
    // Size of the page.
    size: Felt252,
    // Hash of the page.
    hash: Felt252,
    // Cumulative product of the page.
    prod: Felt252,
}
