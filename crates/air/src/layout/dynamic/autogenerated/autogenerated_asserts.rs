use starknet_crypto::Felt;
use swiftness_transcript::{ensure, felt};
use crate::{
    consts::*, domains::StarkDomains, dynamic::DynamicParams, felt_nonzero,
    layout::CheckAssertsError,
};
use starknet_types_core::felt::NonZeroFelt;

pub const FELT_USIZE_MAX: Felt = Felt::from_hex_unchecked("0xFFFFFFFFFFFFFFFF");

pub fn is_power_of_2(x: Felt) -> bool {
    x != FELT_0 && (x.to_bigint() & (x - FELT_1).to_bigint()) == FELT_0.to_bigint()
}

pub fn check_asserts(
    dynamic_params: &DynamicParams,
    stark_domains: &StarkDomains,
) -> Result<(), CheckAssertsError> {
    let trace_length = stark_domains.trace_domain_size;

    // Coset step (dynamicparam(diluted_units_row_ratio)) must be a power of two.
	let x = felt!((dynamic_params.diluted_units_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.diluted_units_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.diluted_units_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.diluted_units_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (dynamicparam(range_check_units_row_ratio)) must be a power of two.
	let x = felt!((dynamic_params.range_check_units_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check_units_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check_units_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check_units_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step ((8) * (dynamicparam(memory_units_row_ratio))) must be a power of two.
	let x = (FELT_8 * felt!(dynamic_params.memory_units_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!((FELT_8 * felt!(dynamic_params.memory_units_row_ratio)))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step (dynamicparam(memory_units_row_ratio)) must be a power of two.
	let x = felt!((dynamic_params.memory_units_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.memory_units_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.memory_units_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.memory_units_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step ((16) * (dynamicparam(cpu_component_step))) must be a power of two.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!((FELT_16 * felt!(dynamic_params.cpu_component_step)))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Step must not exceed dimension.
	let x = (trace_length.floor_div(&felt_nonzero!((FELT_16 * felt!(dynamic_params.cpu_component_step))))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (dynamicparam(cpu_component_step)) must be a power of two.
	let x = felt!((dynamic_params.cpu_component_step));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = (trace_length.floor_div(&felt_nonzero!((FELT_16 * felt!(dynamic_params.cpu_component_step)))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Cpu_component_step is out of range.
	let x = FELT_256 - felt!(dynamic_params.cpu_component_step);
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Memory_units_row_ratio is out of range.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step)) - (FELT_4 * felt!(dynamic_params.memory_units_row_ratio));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/mem_inst must be nonnegative.
	let x = felt!((dynamic_params.cpu_decode_mem_inst_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/mem_inst is too big.
	let x = trace_length - felt!(dynamic_params.cpu_decode_mem_inst_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/mem_inst is too big.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step)) - (felt!(dynamic_params.cpu_decode_mem_inst_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off0 must be nonnegative.
	let x = felt!((dynamic_params.cpu_decode_off0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off0 is too big.
	let x = trace_length - felt!(dynamic_params.cpu_decode_off0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off0 is too big.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step)) - (felt!(dynamic_params.cpu_decode_off0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off1 must be nonnegative.
	let x = felt!((dynamic_params.cpu_decode_off1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off1 is too big.
	let x = trace_length - felt!(dynamic_params.cpu_decode_off1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off1 is too big.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step)) - (felt!(dynamic_params.cpu_decode_off1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off2 must be nonnegative.
	let x = felt!((dynamic_params.cpu_decode_off2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off2 is too big.
	let x = trace_length - felt!(dynamic_params.cpu_decode_off2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/decode/off2 is too big.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step)) - (felt!(dynamic_params.cpu_decode_off2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_dst must be nonnegative.
	let x = felt!((dynamic_params.cpu_operands_mem_dst_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_dst is too big.
	let x = trace_length - felt!(dynamic_params.cpu_operands_mem_dst_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_dst is too big.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step)) - (felt!(dynamic_params.cpu_operands_mem_dst_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_op0 must be nonnegative.
	let x = felt!((dynamic_params.cpu_operands_mem_op0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_op0 is too big.
	let x = trace_length - felt!(dynamic_params.cpu_operands_mem_op0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_op0 is too big.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step)) - (felt!(dynamic_params.cpu_operands_mem_op0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_op1 must be nonnegative.
	let x = felt!((dynamic_params.cpu_operands_mem_op1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_op1 is too big.
	let x = trace_length - felt!(dynamic_params.cpu_operands_mem_op1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of cpu/operands/mem_op1 is too big.
	let x = (FELT_16 * felt!(dynamic_params.cpu_component_step)) - (felt!(dynamic_params.cpu_operands_mem_op1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of orig/public_memory must be nonnegative.
	let x = felt!((dynamic_params.orig_public_memory_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of orig/public_memory is too big.
	let x = trace_length - felt!(dynamic_params.orig_public_memory_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of orig/public_memory is too big.
	let x = (FELT_8 * felt!(dynamic_params.memory_units_row_ratio)) - (felt!(dynamic_params.orig_public_memory_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Uses_pedersen_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_pedersen_builtin) * felt!(dynamic_params.uses_pedersen_builtin)) - felt!(dynamic_params.uses_pedersen_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_range_check_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_range_check_builtin) * felt!(dynamic_params.uses_range_check_builtin)) - felt!(dynamic_params.uses_range_check_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_ecdsa_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_ecdsa_builtin) * felt!(dynamic_params.uses_ecdsa_builtin)) - felt!(dynamic_params.uses_ecdsa_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_bitwise_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_bitwise_builtin) * felt!(dynamic_params.uses_bitwise_builtin)) - felt!(dynamic_params.uses_bitwise_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_ec_op_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_ec_op_builtin) * felt!(dynamic_params.uses_ec_op_builtin)) - felt!(dynamic_params.uses_ec_op_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_keccak_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_keccak_builtin) * felt!(dynamic_params.uses_keccak_builtin)) - felt!(dynamic_params.uses_keccak_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_poseidon_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_poseidon_builtin) * felt!(dynamic_params.uses_poseidon_builtin)) - felt!(dynamic_params.uses_poseidon_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_range_check96_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_range_check96_builtin) * felt!(dynamic_params.uses_range_check96_builtin)) - felt!(dynamic_params.uses_range_check96_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_add_mod_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_add_mod_builtin) * felt!(dynamic_params.uses_add_mod_builtin)) - felt!(dynamic_params.uses_add_mod_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Uses_mul_mod_builtin should be a boolean.
	ensure!((felt!(dynamic_params.uses_mul_mod_builtin) * felt!(dynamic_params.uses_mul_mod_builtin)) - felt!(dynamic_params.uses_mul_mod_builtin) == FELT_0, CheckAssertsError::NotBoolean);
	// Num_columns_first is out of range.
	let x = FELT_65536 - felt!(dynamic_params.num_columns_first) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Num_columns_second is out of range.
	let x = FELT_65536 - felt!(dynamic_params.num_columns_second) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.mem_pool_addr_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.mem_pool_addr_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.mem_pool_addr_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.mem_pool_value_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.mem_pool_value_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.mem_pool_value_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.range_check16_pool_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.range_check16_pool_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.range_check16_pool_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.cpu_decode_opcode_range_check_column_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.cpu_decode_opcode_range_check_column_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.cpu_decode_opcode_range_check_column_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.cpu_registers_ap_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.cpu_registers_ap_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.cpu_registers_ap_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.cpu_registers_fp_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.cpu_registers_fp_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.cpu_registers_fp_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.cpu_operands_ops_mul_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.cpu_operands_ops_mul_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.cpu_operands_ops_mul_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.cpu_operands_res_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.cpu_operands_res_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.cpu_operands_res_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.cpu_update_registers_update_pc_tmp0_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.cpu_update_registers_update_pc_tmp0_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.cpu_update_registers_update_pc_tmp0_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.cpu_update_registers_update_pc_tmp1_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.cpu_update_registers_update_pc_tmp1_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.cpu_update_registers_update_pc_tmp1_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.memory_sorted_addr_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.memory_sorted_addr_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.memory_sorted_addr_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.memory_sorted_value_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.memory_sorted_value_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.memory_sorted_value_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.range_check16_sorted_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.range_check16_sorted_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.range_check16_sorted_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.diluted_pool_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.diluted_pool_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.diluted_pool_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.diluted_check_permuted_values_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.diluted_check_permuted_values_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.diluted_check_permuted_values_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_x_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_x_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_x_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_y_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_y_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_y_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_slope_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.pedersen_hash0_ec_subset_sum_slope_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_slope_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_selector_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.pedersen_hash0_ec_subset_sum_selector_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_selector_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_key_points_x_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_key_points_x_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_key_points_x_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_key_points_y_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_key_points_y_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_key_points_y_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_doubling_slope_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_doubling_slope_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_doubling_slope_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_x_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_x_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_x_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_y_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_y_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_y_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_slope_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_generator_slope_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_slope_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_selector_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_generator_selector_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_selector_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_x_diff_inv_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_generator_x_diff_inv_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_generator_x_diff_inv_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_x_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_x_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_x_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_y_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_y_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_y_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_slope_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_key_slope_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_slope_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_selector_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_key_selector_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_selector_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_x_diff_inv_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_exponentiate_key_x_diff_inv_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_exponentiate_key_x_diff_inv_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_add_results_slope_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_add_results_slope_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_add_results_slope_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_add_results_inv_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_add_results_inv_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_add_results_inv_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_extract_r_slope_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_extract_r_slope_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_extract_r_slope_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_extract_r_inv_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_extract_r_inv_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_extract_r_inv_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_z_inv_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_z_inv_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_z_inv_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_r_w_inv_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_r_w_inv_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_r_w_inv_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ecdsa_signature0_q_x_squared_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_signature0_q_x_squared_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ecdsa_signature0_q_x_squared_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_doubled_points_x_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_doubled_points_x_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_doubled_points_x_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_doubled_points_y_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_doubled_points_y_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_doubled_points_y_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_doubling_slope_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_doubling_slope_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_doubling_slope_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_ec_subset_sum_partial_sum_x_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_ec_subset_sum_partial_sum_x_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_ec_subset_sum_partial_sum_x_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_ec_subset_sum_partial_sum_y_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_ec_subset_sum_partial_sum_y_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_ec_subset_sum_partial_sum_y_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_ec_subset_sum_slope_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_ec_subset_sum_slope_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_ec_subset_sum_slope_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_ec_subset_sum_selector_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_ec_subset_sum_selector_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_ec_subset_sum_selector_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_ec_subset_sum_x_diff_inv_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_ec_subset_sum_x_diff_inv_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_ec_subset_sum_x_diff_inv_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones196_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones196_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones196_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones192_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones192_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones192_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.keccak_keccak_parse_to_diluted_reshaped_intermediate_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_parse_to_diluted_reshaped_intermediate_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_parse_to_diluted_reshaped_intermediate_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.keccak_keccak_parse_to_diluted_final_reshaped_input_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_parse_to_diluted_final_reshaped_input_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_parse_to_diluted_final_reshaped_input_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.keccak_keccak_parse_to_diluted_cumulative_sum_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_parse_to_diluted_cumulative_sum_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_parse_to_diluted_cumulative_sum_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.keccak_keccak_rotated_parity0_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_rotated_parity0_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_rotated_parity0_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.keccak_keccak_rotated_parity1_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_rotated_parity1_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_rotated_parity1_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.keccak_keccak_rotated_parity2_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_rotated_parity2_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_rotated_parity2_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.keccak_keccak_rotated_parity3_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_rotated_parity3_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_rotated_parity3_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.keccak_keccak_rotated_parity4_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_rotated_parity4_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_rotated_parity4_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_full_rounds_state0_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_full_rounds_state0_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_full_rounds_state0_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_full_rounds_state1_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_full_rounds_state1_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_full_rounds_state1_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_full_rounds_state2_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_full_rounds_state2_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_full_rounds_state2_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_full_rounds_state0_squared_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_full_rounds_state0_squared_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_full_rounds_state0_squared_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_full_rounds_state1_squared_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_full_rounds_state1_squared_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_full_rounds_state1_squared_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_full_rounds_state2_squared_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_full_rounds_state2_squared_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_full_rounds_state2_squared_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_partial_rounds_state0_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_partial_rounds_state0_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_partial_rounds_state0_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_partial_rounds_state1_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_partial_rounds_state1_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_partial_rounds_state1_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_partial_rounds_state0_squared_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_partial_rounds_state0_squared_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_partial_rounds_state0_squared_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.poseidon_poseidon_partial_rounds_state1_squared_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.poseidon_poseidon_partial_rounds_state1_squared_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.poseidon_poseidon_partial_rounds_state1_squared_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.add_mod_sub_p_bit_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_sub_p_bit_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.add_mod_sub_p_bit_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.add_mod_carry1_bit_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_carry1_bit_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.add_mod_carry1_bit_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.add_mod_carry2_bit_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_carry2_bit_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.add_mod_carry2_bit_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.add_mod_carry3_bit_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_carry3_bit_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.add_mod_carry3_bit_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.add_mod_carry1_sign_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_carry1_sign_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.add_mod_carry1_sign_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.add_mod_carry2_sign_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_carry2_sign_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.add_mod_carry2_sign_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) - felt!(dynamic_params.add_mod_carry3_sign_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_carry3_sign_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.add_mod_carry3_sign_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.memory_multi_column_perm_perm_cum_prod0_column) - felt!(dynamic_params.num_columns_first);
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) + felt!(dynamic_params.num_columns_second) - felt!(dynamic_params.memory_multi_column_perm_perm_cum_prod0_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.memory_multi_column_perm_perm_cum_prod0_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.memory_multi_column_perm_perm_cum_prod0_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.range_check16_perm_cum_prod0_column) - felt!(dynamic_params.num_columns_first);
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) + felt!(dynamic_params.num_columns_second) - felt!(dynamic_params.range_check16_perm_cum_prod0_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.range_check16_perm_cum_prod0_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.range_check16_perm_cum_prod0_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.diluted_check_cumulative_value_column) - felt!(dynamic_params.num_columns_first);
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) + felt!(dynamic_params.num_columns_second) - felt!(dynamic_params.diluted_check_cumulative_value_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.diluted_check_cumulative_value_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.diluted_check_cumulative_value_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.diluted_check_permutation_cum_prod0_column) - felt!(dynamic_params.num_columns_first);
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Column index out of range.
	let x = felt!(dynamic_params.num_columns_first) + felt!(dynamic_params.num_columns_second) - felt!(dynamic_params.diluted_check_permutation_cum_prod0_column) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be nonnegative.
	let x = felt!((dynamic_params.diluted_check_permutation_cum_prod0_offset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset must be smaller than trace length.
	let x = trace_length - felt!(dynamic_params.diluted_check_permutation_cum_prod0_offset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	if felt!(dynamic_params.uses_pedersen_builtin) != FELT_0 {// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.pedersen_builtin_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.pedersen_builtin_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(pedersen_builtin_row_ratio)) / (512)) must be a power of two.
	let x = (felt!(dynamic_params.pedersen_builtin_row_ratio).floor_div(&felt_nonzero!(FELT_512)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(pedersen_builtin_row_ratio)) / (2)) must be a power of two.
	let x = (felt!(dynamic_params.pedersen_builtin_row_ratio).floor_div(&felt_nonzero!(FELT_2)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Step must not exceed dimension.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.pedersen_builtin_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.pedersen_builtin_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of pedersen/input0 must be nonnegative.
	let x = felt!((dynamic_params.pedersen_input0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of pedersen/input0 is too big.
	let x = trace_length - felt!(dynamic_params.pedersen_input0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of pedersen/input0 is too big.
	let x = felt!(dynamic_params.pedersen_builtin_row_ratio) - (felt!(dynamic_params.pedersen_input0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of pedersen/input1 must be nonnegative.
	let x = felt!((dynamic_params.pedersen_input1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of pedersen/input1 is too big.
	let x = trace_length - felt!(dynamic_params.pedersen_input1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of pedersen/input1 is too big.
	let x = felt!(dynamic_params.pedersen_builtin_row_ratio) - (felt!(dynamic_params.pedersen_input1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of pedersen/output must be nonnegative.
	let x = felt!((dynamic_params.pedersen_output_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of pedersen/output is too big.
	let x = trace_length - felt!(dynamic_params.pedersen_output_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of pedersen/output is too big.
	let x = felt!(dynamic_params.pedersen_builtin_row_ratio) - (felt!(dynamic_params.pedersen_output_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_range_check_builtin) != FELT_0 {// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.range_check_builtin_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check_builtin_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Step must not exceed dimension.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check_builtin_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check_builtin_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step ((dynamicparam(range_check_builtin_row_ratio)) / (8)) must be a power of two.
	let x = (felt!(dynamic_params.range_check_builtin_row_ratio).floor_div(&felt_nonzero!(FELT_8)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of range_check_builtin/mem must be nonnegative.
	let x = felt!((dynamic_params.range_check_builtin_mem_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check_builtin/mem is too big.
	let x = trace_length - felt!(dynamic_params.range_check_builtin_mem_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check_builtin/mem is too big.
	let x = felt!(dynamic_params.range_check_builtin_row_ratio) - (felt!(dynamic_params.range_check_builtin_mem_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check_builtin/inner_range_check must be nonnegative.
	let x = felt!((dynamic_params.range_check_builtin_inner_range_check_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check_builtin/inner_range_check is too big.
	let x = trace_length - felt!(dynamic_params.range_check_builtin_inner_range_check_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check_builtin/inner_range_check is too big.
	let x = (felt!(dynamic_params.range_check_builtin_row_ratio).floor_div(&felt_nonzero!(FELT_8))) - (felt!(dynamic_params.range_check_builtin_inner_range_check_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_ecdsa_builtin) != FELT_0 {// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.ecdsa_builtin_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.ecdsa_builtin_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(ecdsa_builtin_row_ratio)) / (512)) must be a power of two.
	let x = (felt!(dynamic_params.ecdsa_builtin_row_ratio).floor_div(&felt_nonzero!(FELT_512)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Step must not exceed dimension.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.ecdsa_builtin_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.ecdsa_builtin_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step ((dynamicparam(ecdsa_builtin_row_ratio)) / (256)) must be a power of two.
	let x = (felt!(dynamic_params.ecdsa_builtin_row_ratio).floor_div(&felt_nonzero!(FELT_256)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(ecdsa_builtin_row_ratio)) / (2)) must be a power of two.
	let x = (felt!(dynamic_params.ecdsa_builtin_row_ratio).floor_div(&felt_nonzero!(FELT_2)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of ecdsa/pubkey must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_pubkey_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ecdsa/pubkey is too big.
	let x = trace_length - felt!(dynamic_params.ecdsa_pubkey_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ecdsa/pubkey is too big.
	let x = felt!(dynamic_params.ecdsa_builtin_row_ratio) - (felt!(dynamic_params.ecdsa_pubkey_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ecdsa/message must be nonnegative.
	let x = felt!((dynamic_params.ecdsa_message_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ecdsa/message is too big.
	let x = trace_length - felt!(dynamic_params.ecdsa_message_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ecdsa/message is too big.
	let x = felt!(dynamic_params.ecdsa_builtin_row_ratio) - (felt!(dynamic_params.ecdsa_message_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_bitwise_builtin) != FELT_0 {// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.bitwise_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.bitwise_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(bitwise__row_ratio)) / (64)) must be a power of two.
	let x = (felt!(dynamic_params.bitwise_row_ratio).floor_div(&felt_nonzero!(FELT_64)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(bitwise__row_ratio)) / (4)) must be a power of two.
	let x = (felt!(dynamic_params.bitwise_row_ratio).floor_div(&felt_nonzero!(FELT_4)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.bitwise_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.bitwise_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of bitwise/var_pool must be nonnegative.
	let x = felt!((dynamic_params.bitwise_var_pool_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/var_pool is too big.
	let x = trace_length - felt!(dynamic_params.bitwise_var_pool_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/var_pool is too big.
	let x = (felt!(dynamic_params.bitwise_row_ratio).floor_div(&felt_nonzero!(FELT_4))) - (felt!(dynamic_params.bitwise_var_pool_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/x_or_y must be nonnegative.
	let x = felt!((dynamic_params.bitwise_x_or_y_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/x_or_y is too big.
	let x = trace_length - felt!(dynamic_params.bitwise_x_or_y_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/x_or_y is too big.
	let x = felt!(dynamic_params.bitwise_row_ratio) - (felt!(dynamic_params.bitwise_x_or_y_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/diluted_var_pool must be nonnegative.
	let x = felt!((dynamic_params.bitwise_diluted_var_pool_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/diluted_var_pool is too big.
	let x = trace_length - felt!(dynamic_params.bitwise_diluted_var_pool_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/diluted_var_pool is too big.
	let x = (felt!(dynamic_params.bitwise_row_ratio).floor_div(&felt_nonzero!(FELT_64))) - (felt!(dynamic_params.bitwise_diluted_var_pool_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking192 must be nonnegative.
	let x = felt!((dynamic_params.bitwise_trim_unpacking192_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking192 is too big.
	let x = trace_length - felt!(dynamic_params.bitwise_trim_unpacking192_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking192 is too big.
	let x = felt!(dynamic_params.bitwise_row_ratio) - (felt!(dynamic_params.bitwise_trim_unpacking192_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking193 must be nonnegative.
	let x = felt!((dynamic_params.bitwise_trim_unpacking193_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking193 is too big.
	let x = trace_length - felt!(dynamic_params.bitwise_trim_unpacking193_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking193 is too big.
	let x = felt!(dynamic_params.bitwise_row_ratio) - (felt!(dynamic_params.bitwise_trim_unpacking193_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking194 must be nonnegative.
	let x = felt!((dynamic_params.bitwise_trim_unpacking194_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking194 is too big.
	let x = trace_length - felt!(dynamic_params.bitwise_trim_unpacking194_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking194 is too big.
	let x = felt!(dynamic_params.bitwise_row_ratio) - (felt!(dynamic_params.bitwise_trim_unpacking194_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking195 must be nonnegative.
	let x = felt!((dynamic_params.bitwise_trim_unpacking195_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking195 is too big.
	let x = trace_length - felt!(dynamic_params.bitwise_trim_unpacking195_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of bitwise/trim_unpacking195 is too big.
	let x = felt!(dynamic_params.bitwise_row_ratio) - (felt!(dynamic_params.bitwise_trim_unpacking195_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_ec_op_builtin) != FELT_0 {// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.ec_op_builtin_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.ec_op_builtin_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(ec_op_builtin_row_ratio)) / (256)) must be a power of two.
	let x = (felt!(dynamic_params.ec_op_builtin_row_ratio).floor_div(&felt_nonzero!(FELT_256)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.ec_op_builtin_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.ec_op_builtin_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of ec_op/p_x must be nonnegative.
	let x = felt!((dynamic_params.ec_op_p_x_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/p_x is too big.
	let x = trace_length - felt!(dynamic_params.ec_op_p_x_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/p_x is too big.
	let x = felt!(dynamic_params.ec_op_builtin_row_ratio) - (felt!(dynamic_params.ec_op_p_x_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/p_y must be nonnegative.
	let x = felt!((dynamic_params.ec_op_p_y_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/p_y is too big.
	let x = trace_length - felt!(dynamic_params.ec_op_p_y_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/p_y is too big.
	let x = felt!(dynamic_params.ec_op_builtin_row_ratio) - (felt!(dynamic_params.ec_op_p_y_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/q_x must be nonnegative.
	let x = felt!((dynamic_params.ec_op_q_x_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/q_x is too big.
	let x = trace_length - felt!(dynamic_params.ec_op_q_x_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/q_x is too big.
	let x = felt!(dynamic_params.ec_op_builtin_row_ratio) - (felt!(dynamic_params.ec_op_q_x_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/q_y must be nonnegative.
	let x = felt!((dynamic_params.ec_op_q_y_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/q_y is too big.
	let x = trace_length - felt!(dynamic_params.ec_op_q_y_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/q_y is too big.
	let x = felt!(dynamic_params.ec_op_builtin_row_ratio) - (felt!(dynamic_params.ec_op_q_y_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/m must be nonnegative.
	let x = felt!((dynamic_params.ec_op_m_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/m is too big.
	let x = trace_length - felt!(dynamic_params.ec_op_m_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/m is too big.
	let x = felt!(dynamic_params.ec_op_builtin_row_ratio) - (felt!(dynamic_params.ec_op_m_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/r_x must be nonnegative.
	let x = felt!((dynamic_params.ec_op_r_x_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/r_x is too big.
	let x = trace_length - felt!(dynamic_params.ec_op_r_x_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/r_x is too big.
	let x = felt!(dynamic_params.ec_op_builtin_row_ratio) - (felt!(dynamic_params.ec_op_r_x_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/r_y must be nonnegative.
	let x = felt!((dynamic_params.ec_op_r_y_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/r_y is too big.
	let x = trace_length - felt!(dynamic_params.ec_op_r_y_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of ec_op/r_y is too big.
	let x = felt!(dynamic_params.ec_op_builtin_row_ratio) - (felt!(dynamic_params.ec_op_r_y_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_keccak_builtin) != FELT_0 {// Coset step ((dynamicparam(keccak__row_ratio)) / (4096)) must be a power of two.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_4096)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!((FELT_16 * felt!(dynamic_params.keccak_row_ratio)))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(keccak__row_ratio)) / (128)) must be a power of two.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_128)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(keccak__row_ratio)) / (32768)) must be a power of two.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_32768)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.keccak_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(keccak__row_ratio)) / (16)) must be a power of two.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_16)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = ((FELT_16 * trace_length).floor_div(&felt_nonzero!(felt!(dynamic_params.keccak_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = ((FELT_16 * trace_length).floor_div(&felt_nonzero!(felt!(dynamic_params.keccak_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = ((FELT_16 * trace_length).floor_div(&felt_nonzero!(felt!(dynamic_params.keccak_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of keccak/input_output must be nonnegative.
	let x = felt!((dynamic_params.keccak_input_output_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/input_output is too big.
	let x = trace_length - felt!(dynamic_params.keccak_input_output_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/input_output is too big.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_16))) - (felt!(dynamic_params.keccak_input_output_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column0 must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_diluted_column0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column0 is too big.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_diluted_column0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column0 is too big.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_4096))) - (felt!(dynamic_params.keccak_keccak_diluted_column0_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column1 must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_diluted_column1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column1 is too big.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_diluted_column1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column1 is too big.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_4096))) - (felt!(dynamic_params.keccak_keccak_diluted_column1_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column2 must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_diluted_column2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column2 is too big.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_diluted_column2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column2 is too big.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_4096))) - (felt!(dynamic_params.keccak_keccak_diluted_column2_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column3 must be nonnegative.
	let x = felt!((dynamic_params.keccak_keccak_diluted_column3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column3 is too big.
	let x = trace_length - felt!(dynamic_params.keccak_keccak_diluted_column3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of keccak/keccak/diluted_column3 is too big.
	let x = (felt!(dynamic_params.keccak_row_ratio).floor_div(&felt_nonzero!(FELT_4096))) - (felt!(dynamic_params.keccak_keccak_diluted_column3_suboffset) * felt!(dynamic_params.diluted_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_poseidon_builtin) != FELT_0 {// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.poseidon_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.poseidon_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(poseidon__row_ratio)) / (32)) must be a power of two.
	let x = (felt!(dynamic_params.poseidon_row_ratio).floor_div(&felt_nonzero!(FELT_32)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(poseidon__row_ratio)) / (8)) must be a power of two.
	let x = (felt!(dynamic_params.poseidon_row_ratio).floor_div(&felt_nonzero!(FELT_8)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(poseidon__row_ratio)) / (64)) must be a power of two.
	let x = (felt!(dynamic_params.poseidon_row_ratio).floor_div(&felt_nonzero!(FELT_64)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Coset step ((dynamicparam(poseidon__row_ratio)) / (2)) must be a power of two.
	let x = (felt!(dynamic_params.poseidon_row_ratio).floor_div(&felt_nonzero!(FELT_2)));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = ((FELT_2 * trace_length).floor_div(&felt_nonzero!(felt!(dynamic_params.poseidon_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = ((FELT_2 * trace_length).floor_div(&felt_nonzero!(felt!(dynamic_params.poseidon_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = ((FELT_2 * trace_length).floor_div(&felt_nonzero!(felt!(dynamic_params.poseidon_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of poseidon/param_0/input_output must be nonnegative.
	let x = felt!((dynamic_params.poseidon_param_0_input_output_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of poseidon/param_0/input_output is too big.
	let x = trace_length - felt!(dynamic_params.poseidon_param_0_input_output_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of poseidon/param_0/input_output is too big.
	let x = (felt!(dynamic_params.poseidon_row_ratio).floor_div(&felt_nonzero!(FELT_2))) - (felt!(dynamic_params.poseidon_param_0_input_output_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of poseidon/param_1/input_output must be nonnegative.
	let x = felt!((dynamic_params.poseidon_param_1_input_output_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of poseidon/param_1/input_output is too big.
	let x = trace_length - felt!(dynamic_params.poseidon_param_1_input_output_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of poseidon/param_1/input_output is too big.
	let x = (felt!(dynamic_params.poseidon_row_ratio).floor_div(&felt_nonzero!(FELT_2))) - (felt!(dynamic_params.poseidon_param_1_input_output_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of poseidon/param_2/input_output must be nonnegative.
	let x = felt!((dynamic_params.poseidon_param_2_input_output_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of poseidon/param_2/input_output is too big.
	let x = trace_length - felt!(dynamic_params.poseidon_param_2_input_output_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of poseidon/param_2/input_output is too big.
	let x = (felt!(dynamic_params.poseidon_row_ratio).floor_div(&felt_nonzero!(FELT_2))) - (felt!(dynamic_params.poseidon_param_2_input_output_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_range_check96_builtin) != FELT_0 {// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.range_check96_builtin_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check96_builtin_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Step must not exceed dimension.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check96_builtin_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.range_check96_builtin_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/mem must be nonnegative.
	let x = felt!((dynamic_params.range_check96_builtin_mem_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/mem is too big.
	let x = trace_length - felt!(dynamic_params.range_check96_builtin_mem_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/mem is too big.
	let x = felt!(dynamic_params.range_check96_builtin_row_ratio) - (felt!(dynamic_params.range_check96_builtin_mem_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check0 must be nonnegative.
	let x = felt!((dynamic_params.range_check96_builtin_inner_range_check0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check0 is too big.
	let x = trace_length - felt!(dynamic_params.range_check96_builtin_inner_range_check0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check0 is too big.
	let x = felt!(dynamic_params.range_check96_builtin_row_ratio) - (felt!(dynamic_params.range_check96_builtin_inner_range_check0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check1 must be nonnegative.
	let x = felt!((dynamic_params.range_check96_builtin_inner_range_check1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check1 is too big.
	let x = trace_length - felt!(dynamic_params.range_check96_builtin_inner_range_check1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check1 is too big.
	let x = felt!(dynamic_params.range_check96_builtin_row_ratio) - (felt!(dynamic_params.range_check96_builtin_inner_range_check1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check2 must be nonnegative.
	let x = felt!((dynamic_params.range_check96_builtin_inner_range_check2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check2 is too big.
	let x = trace_length - felt!(dynamic_params.range_check96_builtin_inner_range_check2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check2 is too big.
	let x = felt!(dynamic_params.range_check96_builtin_row_ratio) - (felt!(dynamic_params.range_check96_builtin_inner_range_check2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check3 must be nonnegative.
	let x = felt!((dynamic_params.range_check96_builtin_inner_range_check3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check3 is too big.
	let x = trace_length - felt!(dynamic_params.range_check96_builtin_inner_range_check3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check3 is too big.
	let x = felt!(dynamic_params.range_check96_builtin_row_ratio) - (felt!(dynamic_params.range_check96_builtin_inner_range_check3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check4 must be nonnegative.
	let x = felt!((dynamic_params.range_check96_builtin_inner_range_check4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check4 is too big.
	let x = trace_length - felt!(dynamic_params.range_check96_builtin_inner_range_check4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check4 is too big.
	let x = felt!(dynamic_params.range_check96_builtin_row_ratio) - (felt!(dynamic_params.range_check96_builtin_inner_range_check4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check5 must be nonnegative.
	let x = felt!((dynamic_params.range_check96_builtin_inner_range_check5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check5 is too big.
	let x = trace_length - felt!(dynamic_params.range_check96_builtin_inner_range_check5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of range_check96_builtin/inner_range_check5 is too big.
	let x = felt!(dynamic_params.range_check96_builtin_row_ratio) - (felt!(dynamic_params.range_check96_builtin_inner_range_check5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_add_mod_builtin) != FELT_0 {// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.add_mod_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.add_mod_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.add_mod_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.add_mod_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of add_mod/p0 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_p0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p0 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_p0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p0 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_p0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p1 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_p1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p1 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_p1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p1 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_p1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p2 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_p2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p2 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_p2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p2 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_p2_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p3 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_p3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p3 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_p3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/p3 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_p3_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/values_ptr must be nonnegative.
	let x = felt!((dynamic_params.add_mod_values_ptr_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/values_ptr is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_values_ptr_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/values_ptr is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_values_ptr_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/offsets_ptr must be nonnegative.
	let x = felt!((dynamic_params.add_mod_offsets_ptr_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/offsets_ptr is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_offsets_ptr_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/offsets_ptr is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_offsets_ptr_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/n must be nonnegative.
	let x = felt!((dynamic_params.add_mod_n_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/n is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_n_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/n is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_n_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a_offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_a_offset_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a_offset is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_a_offset_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a_offset is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_a_offset_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b_offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_b_offset_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b_offset is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_b_offset_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b_offset is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_b_offset_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c_offset must be nonnegative.
	let x = felt!((dynamic_params.add_mod_c_offset_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c_offset is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_c_offset_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c_offset is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_c_offset_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a0 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_a0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a0 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_a0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a0 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_a0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a1 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_a1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a1 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_a1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a1 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_a1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a2 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_a2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a2 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_a2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a2 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_a2_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a3 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_a3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a3 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_a3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/a3 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_a3_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b0 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_b0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b0 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_b0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b0 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_b0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b1 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_b1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b1 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_b1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b1 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_b1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b2 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_b2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b2 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_b2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b2 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_b2_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b3 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_b3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b3 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_b3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/b3 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_b3_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c0 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_c0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c0 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_c0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c0 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_c0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c1 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_c1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c1 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_c1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c1 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_c1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c2 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_c2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c2 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_c2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c2 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_c2_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c3 must be nonnegative.
	let x = felt!((dynamic_params.add_mod_c3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c3 is too big.
	let x = trace_length - felt!(dynamic_params.add_mod_c3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of add_mod/c3 is too big.
	let x = felt!(dynamic_params.add_mod_row_ratio) - (felt!(dynamic_params.add_mod_c3_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	
	}
	
	if felt!(dynamic_params.uses_mul_mod_builtin) != FELT_0 {// Row ratio should be a power of 2, smaller than trace length.
	let x = felt!((dynamic_params.mul_mod_row_ratio));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Dimension should be a power of 2.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.mul_mod_row_ratio))));
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Index out of range.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.mul_mod_row_ratio)))) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Index should be non negative.
	let x = (trace_length.floor_div(&felt_nonzero!(felt!(dynamic_params.mul_mod_row_ratio))));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Coset step (memberexpression(trace_length)) must be a power of two.
	let x = trace_length;
	ensure!(is_power_of_2(x), CheckAssertsError::NotPowerOfTwo);// Offset of mul_mod/p0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p2_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p3_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/values_ptr must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_values_ptr_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/values_ptr is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_values_ptr_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/values_ptr is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_values_ptr_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/offsets_ptr must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_offsets_ptr_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/offsets_ptr is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_offsets_ptr_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/offsets_ptr is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_offsets_ptr_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/n must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_n_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/n is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_n_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/n is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_n_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a_offset must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_a_offset_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a_offset is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_a_offset_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a_offset is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_a_offset_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b_offset must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_b_offset_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b_offset is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_b_offset_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b_offset is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_b_offset_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c_offset must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_c_offset_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c_offset is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_c_offset_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c_offset is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_c_offset_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_a0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_a0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_a0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_a1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_a1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_a1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_a2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_a2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_a2_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_a3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_a3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/a3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_a3_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_b0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_b0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_b0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_b1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_b1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_b1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_b2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_b2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_b2_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_b3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_b3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/b3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_b3_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_c0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_c0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_c0_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_c1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_c1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_c1_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_c2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_c2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_c2_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_c3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_c3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/c3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_c3_suboffset) * felt!(dynamic_params.memory_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier0_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier0_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier0_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier0_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier0_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier0_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier0_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier0_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier0_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier0_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier0_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier0_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier0_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier0_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier0_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier0_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier0_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier0/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier0_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier1_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier1_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier1_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier1_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier1_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier1_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier1_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier1_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier1_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier1_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier1_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier1_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier1_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier1_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier1_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier1_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier1_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier1/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier1_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier2_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier2_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier2_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier2_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier2_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier2_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier2_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier2_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier2_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier2_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier2_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier2_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier2_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier2_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier2_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier2_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier2_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier2/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier2_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier3_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier3_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier3_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier3_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier3_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier3_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier3_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier3_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier3_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier3_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier3_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier3_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier3_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier3_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier3_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_p_multiplier3_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_p_multiplier3_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/p_multiplier3/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_p_multiplier3_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry0_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry0_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry0_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry0_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry0_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry0_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry0_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry0_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry0_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry0_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry0_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry0_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry0_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry0_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry0_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry0_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry0_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry0_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part6 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry0_part6_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part6 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry0_part6_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry0/part6 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry0_part6_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry1_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry1_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry1_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry1_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry1_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry1_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry1_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry1_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry1_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry1_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry1_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry1_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry1_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry1_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry1_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry1_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry1_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry1_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part6 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry1_part6_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part6 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry1_part6_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry1/part6 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry1_part6_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry2_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry2_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry2_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry2_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry2_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry2_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry2_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry2_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry2_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry2_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry2_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry2_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry2_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry2_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry2_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry2_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry2_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry2_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part6 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry2_part6_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part6 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry2_part6_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry2/part6 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry2_part6_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry3_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry3_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry3_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry3_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry3_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry3_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry3_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry3_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry3_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry3_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry3_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry3_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry3_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry3_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry3_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry3_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry3_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry3_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part6 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry3_part6_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part6 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry3_part6_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry3/part6 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry3_part6_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry4_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry4_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry4_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry4_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry4_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry4_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry4_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry4_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry4_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry4_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry4_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry4_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry4_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry4_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry4_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry4_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry4_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry4_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part6 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry4_part6_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part6 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry4_part6_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry4/part6 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry4_part6_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part0 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry5_part0_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part0 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry5_part0_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part0 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry5_part0_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part1 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry5_part1_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part1 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry5_part1_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part1 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry5_part1_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part2 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry5_part2_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part2 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry5_part2_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part2 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry5_part2_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part3 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry5_part3_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part3 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry5_part3_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part3 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry5_part3_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part4 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry5_part4_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part4 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry5_part4_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part4 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry5_part4_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part5 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry5_part5_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part5 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry5_part5_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part5 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry5_part5_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part6 must be nonnegative.
	let x = felt!((dynamic_params.mul_mod_carry5_part6_suboffset));
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part6 is too big.
	let x = trace_length - felt!(dynamic_params.mul_mod_carry5_part6_suboffset) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	// Offset of mul_mod/carry5/part6 is too big.
	let x = felt!(dynamic_params.mul_mod_row_ratio) - (felt!(dynamic_params.mul_mod_carry5_part6_suboffset) * felt!(dynamic_params.range_check_units_row_ratio)) - FELT_1;
	ensure!(x < FELT_USIZE_MAX, CheckAssertsError::OutOfRange);
	}

    Ok(())
}
