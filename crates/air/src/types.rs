use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SegmentInfo {
    // Start address of the memory segment.
    #[serde_as(as = "UfeHex")]
    begin_addr: Felt,
    // Stop pointer of the segment - not necessarily the end of the segment.
    #[serde_as(as = "UfeHex")]
    stop_ptr: Felt,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicInput {
    #[serde_as(as = "UfeHex")]
    log_n_steps: Felt,
    #[serde_as(as = "UfeHex")]
    range_check_min: Felt,
    #[serde_as(as = "UfeHex")]
    range_check_max: Felt,
    #[serde_as(as = "UfeHex")]
    layout: Felt,
    #[serde_as(as = "Vec<UfeHex>")]
    dynamic_params: Vec<Felt>,
    segments: Vec<SegmentInfo>,
    #[serde_as(as = "UfeHex")]
    padding_addr: Felt,
    #[serde_as(as = "UfeHex")]
    padding_value: Felt,
    main_page: Page,
    continuous_page_headers: Vec<ContinuousPageHeader>,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddrValue {
    #[serde_as(as = "UfeHex")]
    address: Felt,
    #[serde_as(as = "UfeHex")]
    value: Felt,
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
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinuousPageHeader {
    // Start address.
    #[serde_as(as = "UfeHex")]
    start_address: Felt,
    // Size of the page.
    #[serde_as(as = "UfeHex")]
    size: Felt,
    // Hash of the page.
    #[serde_as(as = "UfeHex")]
    hash: Felt,
    // Cumulative product of the page.
    #[serde_as(as = "UfeHex")]
    prod: Felt,
}
