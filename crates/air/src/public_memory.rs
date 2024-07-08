use crate::types::{ContinuousPageHeader, Page, SegmentInfo};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{serde::unsigned_field_element::UfeHex, types::NonZeroFelt};
use starknet_crypto::Felt;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicInput {
    #[serde_as(as = "UfeHex")]
    pub log_n_steps: Felt,
    #[serde_as(as = "UfeHex")]
    pub range_check_min: Felt,
    #[serde_as(as = "UfeHex")]
    pub range_check_max: Felt,
    #[serde_as(as = "UfeHex")]
    pub layout: Felt,
    #[serde_as(as = "Vec<UfeHex>")]
    pub dynamic_params: Vec<Felt>,
    pub segments: Vec<SegmentInfo>,
    #[serde_as(as = "UfeHex")]
    pub padding_addr: Felt,
    #[serde_as(as = "UfeHex")]
    pub padding_value: Felt,
    pub main_page: Page,
    pub continuous_page_headers: Vec<ContinuousPageHeader>,
}

impl PublicInput {
    // Returns the ratio between the product of all public memory cells and z^|public_memory|.
    // This is the value that needs to be at the memory__multi_column_perm__perm__public_memory_prod
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
            .floor_div(&NonZeroFelt::from_felt_unchecked(pages_product))
            .floor_div(&NonZeroFelt::from_felt_unchecked(denominator_pad))
    }
    // Returns the product of all public memory cells.
    pub fn get_public_memory_product(&self, z: Felt, alpha: Felt) -> (Felt, Felt) {
        let main_page_prod = self.main_page.get_product(z, alpha);

        let (continuous_pages_prod, continuous_pages_total_length) =
            get_continuous_pages_product(&self.continuous_page_headers);

        let prod = main_page_prod * continuous_pages_prod;
        let total_length = Felt::from(self.main_page.len()) + continuous_pages_total_length;

        (prod, total_length)
    }
}

fn get_continuous_pages_product(page_headers: &[ContinuousPageHeader]) -> (Felt, Felt) {
    let mut res = Felt::ONE;
    let mut total_length = Felt::ZERO;

    for header in page_headers {
        res *= header.prod;
        total_length += header.size
    }

    (res, total_length)
}
