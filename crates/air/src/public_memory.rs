use crate::{
    consts::{FELT_0, FELT_1, FELT_2},
    dynamic::DynamicParams,
    types::{AddrValue, ContinuousPageHeader, Page, SegmentInfo},
};
use alloc::vec;
use alloc::vec::Vec;
use funvec::{FunVec, FUNVEC_SEGMENTS};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::types::NonZeroFelt;
use starknet_crypto::{pedersen_hash, poseidon_hash_many, Felt};

pub const MAX_LOG_N_STEPS: Felt = Felt::from_hex_unchecked("0x50");
pub const MAX_RANGE_CHECK: Felt = Felt::from_hex_unchecked("0xffff");
pub const MAX_ADDRESS: Felt = Felt::from_hex_unchecked("0xffffffffffffffff");
pub const INITIAL_PC: Felt = Felt::from_hex_unchecked("0x1");

#[serde_as]
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct PublicInput {
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub log_n_steps: Felt,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub range_check_min: Felt,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub range_check_max: Felt,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub layout: Felt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_params: Option<DynamicParams>,
    pub segments: FunVec<SegmentInfo, FUNVEC_SEGMENTS>,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub padding_addr: Felt,
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub padding_value: Felt,
    pub main_page: Page,
    pub continuous_page_headers: FunVec<ContinuousPageHeader, 0>,
}

unsafe impl bytemuck::Zeroable for PublicInput {}
unsafe impl bytemuck::Pod for PublicInput {}

impl PublicInput {
    // Returns the ratio between the product of all public memory cells and z^|public_memory|.
    // This is the value that needs to be at the memory_multi_column_perm_perm_public_memory_prod
    // member expression.
    pub fn get_public_memory_product_ratio(
        &self,
        z: Felt,
        alpha: Felt,
        public_memory_column_size: Felt,
    ) -> Felt {
        let (pages_product, total_length) = self.get_public_memory_product(z, alpha);

        // Pad and divide
        let numerator = z.pow_felt(&public_memory_column_size);
        let padded = z - (self.padding_addr + alpha * self.padding_value);

        assert!(total_length <= public_memory_column_size);
        let denominator_pad = padded.pow_felt(&(public_memory_column_size - total_length));

        numerator
            .field_div(&NonZeroFelt::from_felt_unchecked(pages_product))
            .field_div(&NonZeroFelt::from_felt_unchecked(denominator_pad))
    }
    // Returns the product of all public memory cells.
    pub fn get_public_memory_product(&self, z: Felt, alpha: Felt) -> (Felt, Felt) {
        let main_page_prod = self.main_page.get_product(z, alpha);

        let (continuous_pages_prod, continuous_pages_total_length) =
            get_continuous_pages_product(&self.continuous_page_headers.as_slice());

        let prod = main_page_prod * continuous_pages_prod;
        let total_length = Felt::from(self.main_page.0.len()) + continuous_pages_total_length;

        (prod, total_length)
    }

    #[cfg_attr(feature = "stone5", allow(unused_variables))]
    pub fn get_hash(&self, n_verifier_friendly_commitment_layers: Felt) -> Felt {
        let mut main_page_hash = FELT_0;

        #[inline]
        fn hash_both(memory: &AddrValue, state: &Felt) -> Felt {
            let state = pedersen_hash(&state, &memory.address);
            pedersen_hash(&state, &memory.value)
        }

        for memory in self.main_page.0.iter() {
            main_page_hash = hash_both(&memory, &main_page_hash);
        }
        main_page_hash =
            pedersen_hash(&main_page_hash, &(FELT_2 * Felt::from(self.main_page.0.len())));

        let mut hash_data = {
            #[cfg(feature = "stone5")]
            {
                vec![self.log_n_steps, self.range_check_min, self.range_check_max, self.layout]
            }
            #[cfg(feature = "stone6")]
            {
                vec![
                    n_verifier_friendly_commitment_layers,
                    self.log_n_steps,
                    self.range_check_min,
                    self.range_check_max,
                    self.layout,
                ]
            }
        };

        if let Some(dynamic_params) = &self.dynamic_params {
            let dynamic_params_vec: Vec<u32> = dynamic_params.clone().into();
            hash_data.extend(dynamic_params_vec.into_iter().map(Felt::from));
        }

        // Segments.
        hash_data.extend(self.segments.iter().flat_map(|s| vec![s.begin_addr, s.stop_ptr]));

        hash_data.push(self.padding_addr);
        hash_data.push(self.padding_value);
        hash_data.push(Felt::from(self.continuous_page_headers.len() + 1));

        // Main page.
        hash_data.push(Felt::from(self.main_page.0.len()));
        hash_data.push(main_page_hash);

        // Add the rest of the pages.
        hash_data.extend(
            self.continuous_page_headers.iter().flat_map(|h| vec![h.start_address, h.size, h.hash]),
        );

        poseidon_hash_many(&hash_data)
    }
}

fn get_continuous_pages_product(page_headers: &[ContinuousPageHeader]) -> (Felt, Felt) {
    let mut res = FELT_1;
    let mut total_length = FELT_0;

    for header in page_headers {
        res *= header.prod;
        total_length += header.size
    }

    (res, total_length)
}
