use funvec::{FunVec, FUNVEC_PAGES};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_crypto::Felt;

#[serde_as]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SegmentInfo {
    // Start address of the memory segment.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub begin_addr: Felt,
    // Stop pointer of the segment - not necessarily the end of the segment.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub stop_ptr: Felt,
}

#[serde_as]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AddrValue {
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub address: Felt,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub value: Felt,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Page(pub FunVec<AddrValue, FUNVEC_PAGES>);

impl Page {
    // Returns the product of (z - (addr + alpha * val)) over a single page.
    pub fn get_product(&self, z: Felt, alpha: Felt) -> Felt {
        let mut res = Felt::ONE;
        let mut i = 0;
        loop {
            if i == self.0.len() {
                break res;
            }
            let current = &self.0.as_slice()[i];

            res *= z - (current.address + alpha * current.value);
            i += 1;
        }
    }
}

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ContinuousPageHeader {
    // Start address.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub start_address: Felt,
    // Size of the page.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub size: Felt,
    // Hash of the page.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub hash: Felt,
    // Cumulative product of the page.
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub prod: Felt,
}
