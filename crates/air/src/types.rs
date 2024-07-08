use std::ops::Deref;

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
pub struct AddrValue {
    #[serde_as(as = "UfeHex")]
    address: Felt,
    #[serde_as(as = "UfeHex")]
    value: Felt,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Page(Vec<AddrValue>);

impl Deref for Page {
    type Target = Vec<AddrValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Page {
    // Returns the product of (z - (addr + alpha * val)) over a single page.
    pub fn get_product(&self, z: Felt, alpha: Felt) -> Felt {
        let mut res = Felt::ONE;
        let mut i = 0;
        loop {
            if i == self.len() {
                break res;
            }
            let current = &self[i];

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
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinuousPageHeader {
    // Start address.
    #[serde_as(as = "UfeHex")]
    pub start_address: Felt,
    // Size of the page.
    #[serde_as(as = "UfeHex")]
    pub size: Felt,
    // Hash of the page.
    #[serde_as(as = "UfeHex")]
    pub hash: Felt,
    // Cumulative product of the page.
    #[serde_as(as = "UfeHex")]
    pub prod: Felt,
}
