pub mod global_values;
pub mod trace;
use starknet_crypto::Felt;

// Recursive layout
pub const BITWISE_RATIO: u32 = 8;
pub const BITWISE_ROW_RATIO: u32 = 128;
pub const BITWISE_TOTAL_N_BITS: u32 = 251;
pub const CONSTRAINT_DEGREE: u32 = 2;
pub const CPU_COMPONENT_HEIGHT: u32 = 16;
pub const CPU_COMPONENT_STEP: u32 = 1;
pub const DILUTED_N_BITS: u32 = 16;
pub const DILUTED_SPACING: u32 = 4;
pub const HAS_BITWISE_BUILTIN: u32 = 1;
pub const HAS_DILUTED_POOL: u32 = 1;
pub const HAS_EC_OP_BUILTIN: u32 = 0;
pub const HAS_ECDSA_BUILTIN: u32 = 0;
pub const HAS_KECCAK_BUILTIN: u32 = 0;
pub const HAS_OUTPUT_BUILTIN: u32 = 1;
pub const HAS_PEDERSEN_BUILTIN: u32 = 1;
pub const HAS_POSEIDON_BUILTIN: u32 = 0;
pub const HAS_RANGE_CHECK_BUILTIN: u32 = 1;
pub const HAS_RANGE_CHECK96_BUILTIN: u32 = 0;
pub const IS_DYNAMIC_AIR: u32 = 0;
pub const LAYOUT_CODE: u128 = 0x726563757273697665;
pub const LOG_CPU_COMPONENT_HEIGHT: u32 = 4;
pub const MASK_SIZE: u32 = 133;
pub const N_CONSTRAINTS: u32 = 93;
pub const N_DYNAMIC_PARAMS: u32 = 0;
pub const NUM_COLUMNS_FIRST: u32 = 7;
pub const NUM_COLUMNS_SECOND: u32 = 3;
pub const PEDERSEN_BUILTIN_RATIO: u32 = 128;
pub const PEDERSEN_BUILTIN_REPETITIONS: u32 = 1;
pub const PEDERSEN_BUILTIN_ROW_RATIO: u32 = 2048;
pub const PUBLIC_MEMORY_STEP: u32 = 16;
pub const RANGE_CHECK_BUILTIN_RATIO: u32 = 8;
pub const RANGE_CHECK_BUILTIN_ROW_RATIO: u32 = 128;
pub const RANGE_CHECK_N_PARTS: u32 = 8;

pub mod segments {
    pub const BITWISE: usize = 5;
    pub const EXECUTION: usize = 1;
    pub const N_SEGMENTS: usize = 6;
    pub const OUTPUT: usize = 2;
    pub const PEDERSEN: usize = 3;
    pub const PROGRAM: usize = 0;
    pub const RANGE_CHECK: usize = 4;
}

pub mod builtins {
    use starknet_crypto::Felt;

    pub const OUTPUT: Felt = Felt::from_hex_unchecked("0x6F7574707574");
    pub const PEDERSEN: Felt = Felt::from_hex_unchecked("0x706564657273656E");
    pub const RANGE_CHECK: Felt = Felt::from_hex_unchecked("0x72616E67655F636865636B");
    pub const BITWISE: Felt = Felt::from_hex_unchecked("0x62697477697365");
}

pub fn get_builtins() -> Vec<Felt> {
    vec![builtins::OUTPUT, builtins::PEDERSEN, builtins::RANGE_CHECK, builtins::BITWISE]
}
