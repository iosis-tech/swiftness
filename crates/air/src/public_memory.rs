use crate::{
    domains::StarkDomains,
    layout::recursive::{
        segments, BITWISE_ROW_RATIO, CPU_COMPONENT_HEIGHT, CPU_COMPONENT_STEP, LAYOUT_CODE,
        PEDERSEN_BUILTIN_ROW_RATIO, RANGE_CHECK_BUILTIN_ROW_RATIO,
    },
    types::{ContinuousPageHeader, Page, SegmentInfo},
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{serde::unsigned_field_element::UfeHex, types::NonZeroFelt};
use starknet_crypto::{pedersen_hash, poseidon_hash_many, Felt};

const MAX_ADDRESS: Felt = Felt::from_hex_unchecked("0xffffffffffffffff");
const INITIAL_PC: Felt = Felt::from_hex_unchecked("0x1");

const MAX_LOG_N_STEPS: Felt = Felt::from_hex_unchecked("50");
const MAX_RANGE_CHECK: Felt = Felt::from_hex_unchecked("0xffff"); // 2 ** 16 - 1

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

    pub fn get_public_input_hash(&self) -> Felt {
        let mut main_page_hash = Felt::ZERO;
        for memory in self.main_page.iter() {
            main_page_hash = pedersen_hash(&main_page_hash, &memory.address);
            main_page_hash = pedersen_hash(&main_page_hash, &memory.value);
        }
        main_page_hash =
            pedersen_hash(&main_page_hash, &(Felt::TWO * Felt::from(self.main_page.len())));

        let mut hash_data =
            vec![self.log_n_steps, self.range_check_min, self.range_check_max, self.layout];
        hash_data.extend(self.dynamic_params.iter());

        // Segments.
        hash_data.extend(self.segments.iter().flat_map(|s| vec![s.begin_addr, s.stop_ptr]));

        hash_data.push(self.padding_addr);
        hash_data.push(self.padding_value);
        hash_data.push(Felt::from(self.continuous_page_headers.len() + 1));

        // Main page.
        hash_data.push(Felt::from(self.main_page.len()));
        hash_data.push(main_page_hash);

        // Add the rest of the pages.
        hash_data.extend(
            self.continuous_page_headers.iter().flat_map(|h| vec![h.start_address, h.size, h.hash]),
        );

        poseidon_hash_many(&hash_data)
    }

    pub fn validate(&self, stark_domains: StarkDomains) {
        assert!(self.log_n_steps < MAX_LOG_N_STEPS);
        let n_steps = Felt::TWO.pow_felt(&self.log_n_steps);
        let trace_length = stark_domains.trace_domain_size;
        assert!(
            n_steps * Felt::from(CPU_COMPONENT_HEIGHT) * Felt::from(CPU_COMPONENT_STEP)
                == trace_length
        );

        assert!(Felt::ZERO <= self.range_check_min);
        assert!(self.range_check_min < self.range_check_max);
        assert!(self.range_check_max <= MAX_RANGE_CHECK);

        assert!(self.layout == LAYOUT_CODE.into());

        let output_uses = (self.segments.get(segments::OUTPUT).unwrap().stop_ptr
            - self.segments.get(segments::OUTPUT).unwrap().begin_addr);
        assert!(output_uses < u128::MAX.into());

        let pedersen_copies = trace_length
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::from(PEDERSEN_BUILTIN_ROW_RATIO)));
        let pedersen_uses = (self.segments.get(segments::PEDERSEN).unwrap().stop_ptr
            - self.segments.get(segments::PEDERSEN).unwrap().begin_addr)
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::THREE));
        assert!(pedersen_uses < pedersen_copies);

        let range_check_copies = trace_length.field_div(&NonZeroFelt::from_felt_unchecked(
            Felt::from(RANGE_CHECK_BUILTIN_ROW_RATIO),
        ));
        let range_check_uses = self.segments.get(segments::RANGE_CHECK).unwrap().stop_ptr
            - self.segments.get(segments::RANGE_CHECK).unwrap().begin_addr;
        assert!(range_check_uses < range_check_copies);

        let bitwise_copies = trace_length
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::from(BITWISE_ROW_RATIO)));
        let bitwise_uses = (self.segments.get(segments::BITWISE).unwrap().stop_ptr
            - self.segments.get(segments::BITWISE).unwrap().begin_addr)
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::from(0x5)));
        assert!(bitwise_uses < bitwise_copies);
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
