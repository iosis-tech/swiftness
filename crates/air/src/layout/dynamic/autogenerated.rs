use super::global_values::GlobalValues;
use crate::{domains::StarkDomains, dynamic::DynamicParams, layout::LayoutTrait};
use starknet_crypto::Felt;

pub fn eval_composition_polynomial_inner(
    mask_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    trace_generator: &Felt,
    global_values: &GlobalValues,
    dynamic_params: &DynamicParams,
) -> Felt {
    // Fetch dynamic params.
    let add_mod_a0_suboffset = dynamic_params.add_mod_a0_suboffset;
    let add_mod_a1_suboffset = dynamic_params.add_mod_a1_suboffset;
    let add_mod_a2_suboffset = dynamic_params.add_mod_a2_suboffset;
    let add_mod_a3_suboffset = dynamic_params.add_mod_a3_suboffset;
    let add_mod_a_offset_suboffset = dynamic_params.add_mod_a_offset_suboffset;
    let add_mod_b0_suboffset = dynamic_params.add_mod_b0_suboffset;
    let add_mod_b1_suboffset = dynamic_params.add_mod_b1_suboffset;
    let add_mod_b2_suboffset = dynamic_params.add_mod_b2_suboffset;
    let add_mod_b3_suboffset = dynamic_params.add_mod_b3_suboffset;
    let add_mod_b_offset_suboffset = dynamic_params.add_mod_b_offset_suboffset;
    let add_mod_c0_suboffset = dynamic_params.add_mod_c0_suboffset;
    let add_mod_c1_suboffset = dynamic_params.add_mod_c1_suboffset;
    let add_mod_c2_suboffset = dynamic_params.add_mod_c2_suboffset;
    let add_mod_c3_suboffset = dynamic_params.add_mod_c3_suboffset;
    let add_mod_c_offset_suboffset = dynamic_params.add_mod_c_offset_suboffset;
    let add_mod_carry1_bit_column = dynamic_params.add_mod_carry1_bit_column;
    let add_mod_carry1_bit_offset = dynamic_params.add_mod_carry1_bit_offset;
    let add_mod_carry1_sign_column = dynamic_params.add_mod_carry1_sign_column;
    let add_mod_carry1_sign_offset = dynamic_params.add_mod_carry1_sign_offset;
    let add_mod_carry2_bit_column = dynamic_params.add_mod_carry2_bit_column;
    let add_mod_carry2_bit_offset = dynamic_params.add_mod_carry2_bit_offset;
    let add_mod_carry2_sign_column = dynamic_params.add_mod_carry2_sign_column;
    let add_mod_carry2_sign_offset = dynamic_params.add_mod_carry2_sign_offset;
    let add_mod_carry3_bit_column = dynamic_params.add_mod_carry3_bit_column;
    let add_mod_carry3_bit_offset = dynamic_params.add_mod_carry3_bit_offset;
    let add_mod_carry3_sign_column = dynamic_params.add_mod_carry3_sign_column;
    let add_mod_carry3_sign_offset = dynamic_params.add_mod_carry3_sign_offset;
    let add_mod_n_suboffset = dynamic_params.add_mod_n_suboffset;
    let add_mod_offsets_ptr_suboffset = dynamic_params.add_mod_offsets_ptr_suboffset;
    let add_mod_p0_suboffset = dynamic_params.add_mod_p0_suboffset;
    let add_mod_p1_suboffset = dynamic_params.add_mod_p1_suboffset;
    let add_mod_p2_suboffset = dynamic_params.add_mod_p2_suboffset;
    let add_mod_p3_suboffset = dynamic_params.add_mod_p3_suboffset;
    let add_mod_row_ratio = dynamic_params.add_mod_row_ratio;
    let add_mod_sub_p_bit_column = dynamic_params.add_mod_sub_p_bit_column;
    let add_mod_sub_p_bit_offset = dynamic_params.add_mod_sub_p_bit_offset;
    let add_mod_values_ptr_suboffset = dynamic_params.add_mod_values_ptr_suboffset;
    let bitwise_diluted_var_pool_suboffset = dynamic_params.bitwise_diluted_var_pool_suboffset;
    let bitwise_row_ratio = dynamic_params.bitwise_row_ratio;
    let bitwise_trim_unpacking192_suboffset = dynamic_params.bitwise_trim_unpacking192_suboffset;
    let bitwise_trim_unpacking193_suboffset = dynamic_params.bitwise_trim_unpacking193_suboffset;
    let bitwise_trim_unpacking194_suboffset = dynamic_params.bitwise_trim_unpacking194_suboffset;
    let bitwise_trim_unpacking195_suboffset = dynamic_params.bitwise_trim_unpacking195_suboffset;
    let bitwise_var_pool_suboffset = dynamic_params.bitwise_var_pool_suboffset;
    let bitwise_x_or_y_suboffset = dynamic_params.bitwise_x_or_y_suboffset;
    let cpu_decode_mem_inst_suboffset = dynamic_params.cpu_decode_mem_inst_suboffset;
    let cpu_decode_off0_suboffset = dynamic_params.cpu_decode_off0_suboffset;
    let cpu_decode_off1_suboffset = dynamic_params.cpu_decode_off1_suboffset;
    let cpu_decode_off2_suboffset = dynamic_params.cpu_decode_off2_suboffset;
    let cpu_decode_opcode_range_check_column_column =
        dynamic_params.cpu_decode_opcode_range_check_column_column;
    let cpu_decode_opcode_range_check_column_offset =
        dynamic_params.cpu_decode_opcode_range_check_column_offset;
    let cpu_operands_mem_dst_suboffset = dynamic_params.cpu_operands_mem_dst_suboffset;
    let cpu_operands_mem_op0_suboffset = dynamic_params.cpu_operands_mem_op0_suboffset;
    let cpu_operands_mem_op1_suboffset = dynamic_params.cpu_operands_mem_op1_suboffset;
    let cpu_operands_ops_mul_column = dynamic_params.cpu_operands_ops_mul_column;
    let cpu_operands_ops_mul_offset = dynamic_params.cpu_operands_ops_mul_offset;
    let cpu_operands_res_column = dynamic_params.cpu_operands_res_column;
    let cpu_operands_res_offset = dynamic_params.cpu_operands_res_offset;
    let cpu_registers_ap_column = dynamic_params.cpu_registers_ap_column;
    let cpu_registers_ap_offset = dynamic_params.cpu_registers_ap_offset;
    let cpu_registers_fp_column = dynamic_params.cpu_registers_fp_column;
    let cpu_registers_fp_offset = dynamic_params.cpu_registers_fp_offset;
    let cpu_update_registers_update_pc_tmp0_column =
        dynamic_params.cpu_update_registers_update_pc_tmp0_column;
    let cpu_update_registers_update_pc_tmp0_offset =
        dynamic_params.cpu_update_registers_update_pc_tmp0_offset;
    let cpu_update_registers_update_pc_tmp1_column =
        dynamic_params.cpu_update_registers_update_pc_tmp1_column;
    let cpu_update_registers_update_pc_tmp1_offset =
        dynamic_params.cpu_update_registers_update_pc_tmp1_offset;
    let cpu_component_step = dynamic_params.cpu_component_step;
    let diluted_check_cumulative_value_column =
        dynamic_params.diluted_check_cumulative_value_column;
    let diluted_check_cumulative_value_offset =
        dynamic_params.diluted_check_cumulative_value_offset;
    let diluted_check_permutation_cum_prod0_column =
        dynamic_params.diluted_check_permutation_cum_prod0_column;
    let diluted_check_permutation_cum_prod0_offset =
        dynamic_params.diluted_check_permutation_cum_prod0_offset;
    let diluted_check_permuted_values_column = dynamic_params.diluted_check_permuted_values_column;
    let diluted_check_permuted_values_offset = dynamic_params.diluted_check_permuted_values_offset;
    let diluted_pool_column = dynamic_params.diluted_pool_column;
    let diluted_pool_offset = dynamic_params.diluted_pool_offset;
    let diluted_units_row_ratio = dynamic_params.diluted_units_row_ratio;
    let ec_op_doubled_points_x_column = dynamic_params.ec_op_doubled_points_x_column;
    let ec_op_doubled_points_x_offset = dynamic_params.ec_op_doubled_points_x_offset;
    let ec_op_doubled_points_y_column = dynamic_params.ec_op_doubled_points_y_column;
    let ec_op_doubled_points_y_offset = dynamic_params.ec_op_doubled_points_y_offset;
    let ec_op_doubling_slope_column = dynamic_params.ec_op_doubling_slope_column;
    let ec_op_doubling_slope_offset = dynamic_params.ec_op_doubling_slope_offset;
    let ec_op_ec_subset_sum_bit_unpacking_prod_ones192_column =
        dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones192_column;
    let ec_op_ec_subset_sum_bit_unpacking_prod_ones192_offset =
        dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones192_offset;
    let ec_op_ec_subset_sum_bit_unpacking_prod_ones196_column =
        dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones196_column;
    let ec_op_ec_subset_sum_bit_unpacking_prod_ones196_offset =
        dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones196_offset;
    let ec_op_ec_subset_sum_partial_sum_x_column =
        dynamic_params.ec_op_ec_subset_sum_partial_sum_x_column;
    let ec_op_ec_subset_sum_partial_sum_x_offset =
        dynamic_params.ec_op_ec_subset_sum_partial_sum_x_offset;
    let ec_op_ec_subset_sum_partial_sum_y_column =
        dynamic_params.ec_op_ec_subset_sum_partial_sum_y_column;
    let ec_op_ec_subset_sum_partial_sum_y_offset =
        dynamic_params.ec_op_ec_subset_sum_partial_sum_y_offset;
    let ec_op_ec_subset_sum_selector_column = dynamic_params.ec_op_ec_subset_sum_selector_column;
    let ec_op_ec_subset_sum_selector_offset = dynamic_params.ec_op_ec_subset_sum_selector_offset;
    let ec_op_ec_subset_sum_slope_column = dynamic_params.ec_op_ec_subset_sum_slope_column;
    let ec_op_ec_subset_sum_slope_offset = dynamic_params.ec_op_ec_subset_sum_slope_offset;
    let ec_op_ec_subset_sum_x_diff_inv_column =
        dynamic_params.ec_op_ec_subset_sum_x_diff_inv_column;
    let ec_op_ec_subset_sum_x_diff_inv_offset =
        dynamic_params.ec_op_ec_subset_sum_x_diff_inv_offset;
    let ec_op_m_suboffset = dynamic_params.ec_op_m_suboffset;
    let ec_op_p_x_suboffset = dynamic_params.ec_op_p_x_suboffset;
    let ec_op_p_y_suboffset = dynamic_params.ec_op_p_y_suboffset;
    let ec_op_q_x_suboffset = dynamic_params.ec_op_q_x_suboffset;
    let ec_op_q_y_suboffset = dynamic_params.ec_op_q_y_suboffset;
    let ec_op_r_x_suboffset = dynamic_params.ec_op_r_x_suboffset;
    let ec_op_r_y_suboffset = dynamic_params.ec_op_r_y_suboffset;
    let ec_op_builtin_row_ratio = dynamic_params.ec_op_builtin_row_ratio;
    let ecdsa_message_suboffset = dynamic_params.ecdsa_message_suboffset;
    let ecdsa_pubkey_suboffset = dynamic_params.ecdsa_pubkey_suboffset;
    let ecdsa_signature0_add_results_inv_column =
        dynamic_params.ecdsa_signature0_add_results_inv_column;
    let ecdsa_signature0_add_results_inv_offset =
        dynamic_params.ecdsa_signature0_add_results_inv_offset;
    let ecdsa_signature0_add_results_slope_column =
        dynamic_params.ecdsa_signature0_add_results_slope_column;
    let ecdsa_signature0_add_results_slope_offset =
        dynamic_params.ecdsa_signature0_add_results_slope_offset;
    let ecdsa_signature0_doubling_slope_column =
        dynamic_params.ecdsa_signature0_doubling_slope_column;
    let ecdsa_signature0_doubling_slope_offset =
        dynamic_params.ecdsa_signature0_doubling_slope_offset;
    let ecdsa_signature0_exponentiate_generator_partial_sum_x_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_x_column;
    let ecdsa_signature0_exponentiate_generator_partial_sum_x_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_x_offset;
    let ecdsa_signature0_exponentiate_generator_partial_sum_y_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_y_column;
    let ecdsa_signature0_exponentiate_generator_partial_sum_y_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_y_offset;
    let ecdsa_signature0_exponentiate_generator_selector_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_selector_column;
    let ecdsa_signature0_exponentiate_generator_selector_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_selector_offset;
    let ecdsa_signature0_exponentiate_generator_slope_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_slope_column;
    let ecdsa_signature0_exponentiate_generator_slope_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_slope_offset;
    let ecdsa_signature0_exponentiate_generator_x_diff_inv_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_x_diff_inv_column;
    let ecdsa_signature0_exponentiate_generator_x_diff_inv_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_x_diff_inv_offset;
    let ecdsa_signature0_exponentiate_key_partial_sum_x_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_x_column;
    let ecdsa_signature0_exponentiate_key_partial_sum_x_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_x_offset;
    let ecdsa_signature0_exponentiate_key_partial_sum_y_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_y_column;
    let ecdsa_signature0_exponentiate_key_partial_sum_y_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_y_offset;
    let ecdsa_signature0_exponentiate_key_selector_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_selector_column;
    let ecdsa_signature0_exponentiate_key_selector_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_selector_offset;
    let ecdsa_signature0_exponentiate_key_slope_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_slope_column;
    let ecdsa_signature0_exponentiate_key_slope_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_slope_offset;
    let ecdsa_signature0_exponentiate_key_x_diff_inv_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_x_diff_inv_column;
    let ecdsa_signature0_exponentiate_key_x_diff_inv_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_x_diff_inv_offset;
    let ecdsa_signature0_extract_r_inv_column =
        dynamic_params.ecdsa_signature0_extract_r_inv_column;
    let ecdsa_signature0_extract_r_inv_offset =
        dynamic_params.ecdsa_signature0_extract_r_inv_offset;
    let ecdsa_signature0_extract_r_slope_column =
        dynamic_params.ecdsa_signature0_extract_r_slope_column;
    let ecdsa_signature0_extract_r_slope_offset =
        dynamic_params.ecdsa_signature0_extract_r_slope_offset;
    let ecdsa_signature0_key_points_x_column = dynamic_params.ecdsa_signature0_key_points_x_column;
    let ecdsa_signature0_key_points_x_offset = dynamic_params.ecdsa_signature0_key_points_x_offset;
    let ecdsa_signature0_key_points_y_column = dynamic_params.ecdsa_signature0_key_points_y_column;
    let ecdsa_signature0_key_points_y_offset = dynamic_params.ecdsa_signature0_key_points_y_offset;
    let ecdsa_signature0_q_x_squared_column = dynamic_params.ecdsa_signature0_q_x_squared_column;
    let ecdsa_signature0_q_x_squared_offset = dynamic_params.ecdsa_signature0_q_x_squared_offset;
    let ecdsa_signature0_r_w_inv_column = dynamic_params.ecdsa_signature0_r_w_inv_column;
    let ecdsa_signature0_r_w_inv_offset = dynamic_params.ecdsa_signature0_r_w_inv_offset;
    let ecdsa_signature0_z_inv_column = dynamic_params.ecdsa_signature0_z_inv_column;
    let ecdsa_signature0_z_inv_offset = dynamic_params.ecdsa_signature0_z_inv_offset;
    let ecdsa_builtin_row_ratio = dynamic_params.ecdsa_builtin_row_ratio;
    let keccak_input_output_suboffset = dynamic_params.keccak_input_output_suboffset;
    let keccak_keccak_diluted_column0_suboffset =
        dynamic_params.keccak_keccak_diluted_column0_suboffset;
    let keccak_keccak_diluted_column1_suboffset =
        dynamic_params.keccak_keccak_diluted_column1_suboffset;
    let keccak_keccak_diluted_column2_suboffset =
        dynamic_params.keccak_keccak_diluted_column2_suboffset;
    let keccak_keccak_diluted_column3_suboffset =
        dynamic_params.keccak_keccak_diluted_column3_suboffset;
    let keccak_keccak_parse_to_diluted_cumulative_sum_column =
        dynamic_params.keccak_keccak_parse_to_diluted_cumulative_sum_column;
    let keccak_keccak_parse_to_diluted_cumulative_sum_offset =
        dynamic_params.keccak_keccak_parse_to_diluted_cumulative_sum_offset;
    let keccak_keccak_parse_to_diluted_final_reshaped_input_column =
        dynamic_params.keccak_keccak_parse_to_diluted_final_reshaped_input_column;
    let keccak_keccak_parse_to_diluted_final_reshaped_input_offset =
        dynamic_params.keccak_keccak_parse_to_diluted_final_reshaped_input_offset;
    let keccak_keccak_parse_to_diluted_reshaped_intermediate_column =
        dynamic_params.keccak_keccak_parse_to_diluted_reshaped_intermediate_column;
    let keccak_keccak_parse_to_diluted_reshaped_intermediate_offset =
        dynamic_params.keccak_keccak_parse_to_diluted_reshaped_intermediate_offset;
    let keccak_keccak_rotated_parity0_column = dynamic_params.keccak_keccak_rotated_parity0_column;
    let keccak_keccak_rotated_parity0_offset = dynamic_params.keccak_keccak_rotated_parity0_offset;
    let keccak_keccak_rotated_parity1_column = dynamic_params.keccak_keccak_rotated_parity1_column;
    let keccak_keccak_rotated_parity1_offset = dynamic_params.keccak_keccak_rotated_parity1_offset;
    let keccak_keccak_rotated_parity2_column = dynamic_params.keccak_keccak_rotated_parity2_column;
    let keccak_keccak_rotated_parity2_offset = dynamic_params.keccak_keccak_rotated_parity2_offset;
    let keccak_keccak_rotated_parity3_column = dynamic_params.keccak_keccak_rotated_parity3_column;
    let keccak_keccak_rotated_parity3_offset = dynamic_params.keccak_keccak_rotated_parity3_offset;
    let keccak_keccak_rotated_parity4_column = dynamic_params.keccak_keccak_rotated_parity4_column;
    let keccak_keccak_rotated_parity4_offset = dynamic_params.keccak_keccak_rotated_parity4_offset;
    let keccak_row_ratio = dynamic_params.keccak_row_ratio;
    let mem_pool_addr_column = dynamic_params.mem_pool_addr_column;
    let mem_pool_addr_offset = dynamic_params.mem_pool_addr_offset;
    let mem_pool_value_column = dynamic_params.mem_pool_value_column;
    let mem_pool_value_offset = dynamic_params.mem_pool_value_offset;
    let memory_multi_column_perm_perm_cum_prod0_column =
        dynamic_params.memory_multi_column_perm_perm_cum_prod0_column;
    let memory_multi_column_perm_perm_cum_prod0_offset =
        dynamic_params.memory_multi_column_perm_perm_cum_prod0_offset;
    let memory_sorted_addr_column = dynamic_params.memory_sorted_addr_column;
    let memory_sorted_addr_offset = dynamic_params.memory_sorted_addr_offset;
    let memory_sorted_value_column = dynamic_params.memory_sorted_value_column;
    let memory_sorted_value_offset = dynamic_params.memory_sorted_value_offset;
    let memory_units_row_ratio = dynamic_params.memory_units_row_ratio;
    let mul_mod_a0_suboffset = dynamic_params.mul_mod_a0_suboffset;
    let mul_mod_a1_suboffset = dynamic_params.mul_mod_a1_suboffset;
    let mul_mod_a2_suboffset = dynamic_params.mul_mod_a2_suboffset;
    let mul_mod_a3_suboffset = dynamic_params.mul_mod_a3_suboffset;
    let mul_mod_a_offset_suboffset = dynamic_params.mul_mod_a_offset_suboffset;
    let mul_mod_b0_suboffset = dynamic_params.mul_mod_b0_suboffset;
    let mul_mod_b1_suboffset = dynamic_params.mul_mod_b1_suboffset;
    let mul_mod_b2_suboffset = dynamic_params.mul_mod_b2_suboffset;
    let mul_mod_b3_suboffset = dynamic_params.mul_mod_b3_suboffset;
    let mul_mod_b_offset_suboffset = dynamic_params.mul_mod_b_offset_suboffset;
    let mul_mod_c0_suboffset = dynamic_params.mul_mod_c0_suboffset;
    let mul_mod_c1_suboffset = dynamic_params.mul_mod_c1_suboffset;
    let mul_mod_c2_suboffset = dynamic_params.mul_mod_c2_suboffset;
    let mul_mod_c3_suboffset = dynamic_params.mul_mod_c3_suboffset;
    let mul_mod_c_offset_suboffset = dynamic_params.mul_mod_c_offset_suboffset;
    let mul_mod_carry0_part0_suboffset = dynamic_params.mul_mod_carry0_part0_suboffset;
    let mul_mod_carry0_part1_suboffset = dynamic_params.mul_mod_carry0_part1_suboffset;
    let mul_mod_carry0_part2_suboffset = dynamic_params.mul_mod_carry0_part2_suboffset;
    let mul_mod_carry0_part3_suboffset = dynamic_params.mul_mod_carry0_part3_suboffset;
    let mul_mod_carry0_part4_suboffset = dynamic_params.mul_mod_carry0_part4_suboffset;
    let mul_mod_carry0_part5_suboffset = dynamic_params.mul_mod_carry0_part5_suboffset;
    let mul_mod_carry0_part6_suboffset = dynamic_params.mul_mod_carry0_part6_suboffset;
    let mul_mod_carry1_part0_suboffset = dynamic_params.mul_mod_carry1_part0_suboffset;
    let mul_mod_carry1_part1_suboffset = dynamic_params.mul_mod_carry1_part1_suboffset;
    let mul_mod_carry1_part2_suboffset = dynamic_params.mul_mod_carry1_part2_suboffset;
    let mul_mod_carry1_part3_suboffset = dynamic_params.mul_mod_carry1_part3_suboffset;
    let mul_mod_carry1_part4_suboffset = dynamic_params.mul_mod_carry1_part4_suboffset;
    let mul_mod_carry1_part5_suboffset = dynamic_params.mul_mod_carry1_part5_suboffset;
    let mul_mod_carry1_part6_suboffset = dynamic_params.mul_mod_carry1_part6_suboffset;
    let mul_mod_carry2_part0_suboffset = dynamic_params.mul_mod_carry2_part0_suboffset;
    let mul_mod_carry2_part1_suboffset = dynamic_params.mul_mod_carry2_part1_suboffset;
    let mul_mod_carry2_part2_suboffset = dynamic_params.mul_mod_carry2_part2_suboffset;
    let mul_mod_carry2_part3_suboffset = dynamic_params.mul_mod_carry2_part3_suboffset;
    let mul_mod_carry2_part4_suboffset = dynamic_params.mul_mod_carry2_part4_suboffset;
    let mul_mod_carry2_part5_suboffset = dynamic_params.mul_mod_carry2_part5_suboffset;
    let mul_mod_carry2_part6_suboffset = dynamic_params.mul_mod_carry2_part6_suboffset;
    let mul_mod_carry3_part0_suboffset = dynamic_params.mul_mod_carry3_part0_suboffset;
    let mul_mod_carry3_part1_suboffset = dynamic_params.mul_mod_carry3_part1_suboffset;
    let mul_mod_carry3_part2_suboffset = dynamic_params.mul_mod_carry3_part2_suboffset;
    let mul_mod_carry3_part3_suboffset = dynamic_params.mul_mod_carry3_part3_suboffset;
    let mul_mod_carry3_part4_suboffset = dynamic_params.mul_mod_carry3_part4_suboffset;
    let mul_mod_carry3_part5_suboffset = dynamic_params.mul_mod_carry3_part5_suboffset;
    let mul_mod_carry3_part6_suboffset = dynamic_params.mul_mod_carry3_part6_suboffset;
    let mul_mod_carry4_part0_suboffset = dynamic_params.mul_mod_carry4_part0_suboffset;
    let mul_mod_carry4_part1_suboffset = dynamic_params.mul_mod_carry4_part1_suboffset;
    let mul_mod_carry4_part2_suboffset = dynamic_params.mul_mod_carry4_part2_suboffset;
    let mul_mod_carry4_part3_suboffset = dynamic_params.mul_mod_carry4_part3_suboffset;
    let mul_mod_carry4_part4_suboffset = dynamic_params.mul_mod_carry4_part4_suboffset;
    let mul_mod_carry4_part5_suboffset = dynamic_params.mul_mod_carry4_part5_suboffset;
    let mul_mod_carry4_part6_suboffset = dynamic_params.mul_mod_carry4_part6_suboffset;
    let mul_mod_carry5_part0_suboffset = dynamic_params.mul_mod_carry5_part0_suboffset;
    let mul_mod_carry5_part1_suboffset = dynamic_params.mul_mod_carry5_part1_suboffset;
    let mul_mod_carry5_part2_suboffset = dynamic_params.mul_mod_carry5_part2_suboffset;
    let mul_mod_carry5_part3_suboffset = dynamic_params.mul_mod_carry5_part3_suboffset;
    let mul_mod_carry5_part4_suboffset = dynamic_params.mul_mod_carry5_part4_suboffset;
    let mul_mod_carry5_part5_suboffset = dynamic_params.mul_mod_carry5_part5_suboffset;
    let mul_mod_carry5_part6_suboffset = dynamic_params.mul_mod_carry5_part6_suboffset;
    let mul_mod_n_suboffset = dynamic_params.mul_mod_n_suboffset;
    let mul_mod_offsets_ptr_suboffset = dynamic_params.mul_mod_offsets_ptr_suboffset;
    let mul_mod_p0_suboffset = dynamic_params.mul_mod_p0_suboffset;
    let mul_mod_p1_suboffset = dynamic_params.mul_mod_p1_suboffset;
    let mul_mod_p2_suboffset = dynamic_params.mul_mod_p2_suboffset;
    let mul_mod_p3_suboffset = dynamic_params.mul_mod_p3_suboffset;
    let mul_mod_p_multiplier0_part0_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part0_suboffset;
    let mul_mod_p_multiplier0_part1_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part1_suboffset;
    let mul_mod_p_multiplier0_part2_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part2_suboffset;
    let mul_mod_p_multiplier0_part3_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part3_suboffset;
    let mul_mod_p_multiplier0_part4_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part4_suboffset;
    let mul_mod_p_multiplier0_part5_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part5_suboffset;
    let mul_mod_p_multiplier1_part0_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part0_suboffset;
    let mul_mod_p_multiplier1_part1_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part1_suboffset;
    let mul_mod_p_multiplier1_part2_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part2_suboffset;
    let mul_mod_p_multiplier1_part3_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part3_suboffset;
    let mul_mod_p_multiplier1_part4_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part4_suboffset;
    let mul_mod_p_multiplier1_part5_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part5_suboffset;
    let mul_mod_p_multiplier2_part0_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part0_suboffset;
    let mul_mod_p_multiplier2_part1_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part1_suboffset;
    let mul_mod_p_multiplier2_part2_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part2_suboffset;
    let mul_mod_p_multiplier2_part3_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part3_suboffset;
    let mul_mod_p_multiplier2_part4_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part4_suboffset;
    let mul_mod_p_multiplier2_part5_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part5_suboffset;
    let mul_mod_p_multiplier3_part0_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part0_suboffset;
    let mul_mod_p_multiplier3_part1_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part1_suboffset;
    let mul_mod_p_multiplier3_part2_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part2_suboffset;
    let mul_mod_p_multiplier3_part3_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part3_suboffset;
    let mul_mod_p_multiplier3_part4_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part4_suboffset;
    let mul_mod_p_multiplier3_part5_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part5_suboffset;
    let mul_mod_row_ratio = dynamic_params.mul_mod_row_ratio;
    let mul_mod_values_ptr_suboffset = dynamic_params.mul_mod_values_ptr_suboffset;
    let num_columns_first = dynamic_params.num_columns_first;
    let num_columns_second = dynamic_params.num_columns_second;
    let orig_public_memory_suboffset = dynamic_params.orig_public_memory_suboffset;
    let pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_column;
    let pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_offset;
    let pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_column;
    let pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_offset;
    let pedersen_hash0_ec_subset_sum_partial_sum_x_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_x_column;
    let pedersen_hash0_ec_subset_sum_partial_sum_x_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_x_offset;
    let pedersen_hash0_ec_subset_sum_partial_sum_y_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_y_column;
    let pedersen_hash0_ec_subset_sum_partial_sum_y_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_y_offset;
    let pedersen_hash0_ec_subset_sum_selector_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_selector_column;
    let pedersen_hash0_ec_subset_sum_selector_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_selector_offset;
    let pedersen_hash0_ec_subset_sum_slope_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_slope_column;
    let pedersen_hash0_ec_subset_sum_slope_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_slope_offset;
    let pedersen_input0_suboffset = dynamic_params.pedersen_input0_suboffset;
    let pedersen_input1_suboffset = dynamic_params.pedersen_input1_suboffset;
    let pedersen_output_suboffset = dynamic_params.pedersen_output_suboffset;
    let pedersen_builtin_row_ratio = dynamic_params.pedersen_builtin_row_ratio;
    let poseidon_param_0_input_output_suboffset =
        dynamic_params.poseidon_param_0_input_output_suboffset;
    let poseidon_param_1_input_output_suboffset =
        dynamic_params.poseidon_param_1_input_output_suboffset;
    let poseidon_param_2_input_output_suboffset =
        dynamic_params.poseidon_param_2_input_output_suboffset;
    let poseidon_poseidon_full_rounds_state0_column =
        dynamic_params.poseidon_poseidon_full_rounds_state0_column;
    let poseidon_poseidon_full_rounds_state0_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state0_offset;
    let poseidon_poseidon_full_rounds_state0_squared_column =
        dynamic_params.poseidon_poseidon_full_rounds_state0_squared_column;
    let poseidon_poseidon_full_rounds_state0_squared_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state0_squared_offset;
    let poseidon_poseidon_full_rounds_state1_column =
        dynamic_params.poseidon_poseidon_full_rounds_state1_column;
    let poseidon_poseidon_full_rounds_state1_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state1_offset;
    let poseidon_poseidon_full_rounds_state1_squared_column =
        dynamic_params.poseidon_poseidon_full_rounds_state1_squared_column;
    let poseidon_poseidon_full_rounds_state1_squared_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state1_squared_offset;
    let poseidon_poseidon_full_rounds_state2_column =
        dynamic_params.poseidon_poseidon_full_rounds_state2_column;
    let poseidon_poseidon_full_rounds_state2_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state2_offset;
    let poseidon_poseidon_full_rounds_state2_squared_column =
        dynamic_params.poseidon_poseidon_full_rounds_state2_squared_column;
    let poseidon_poseidon_full_rounds_state2_squared_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state2_squared_offset;
    let poseidon_poseidon_partial_rounds_state0_column =
        dynamic_params.poseidon_poseidon_partial_rounds_state0_column;
    let poseidon_poseidon_partial_rounds_state0_offset =
        dynamic_params.poseidon_poseidon_partial_rounds_state0_offset;
    let poseidon_poseidon_partial_rounds_state0_squared_column =
        dynamic_params.poseidon_poseidon_partial_rounds_state0_squared_column;
    let poseidon_poseidon_partial_rounds_state0_squared_offset =
        dynamic_params.poseidon_poseidon_partial_rounds_state0_squared_offset;
    let poseidon_poseidon_partial_rounds_state1_column =
        dynamic_params.poseidon_poseidon_partial_rounds_state1_column;
    let poseidon_poseidon_partial_rounds_state1_offset =
        dynamic_params.poseidon_poseidon_partial_rounds_state1_offset;
    let poseidon_poseidon_partial_rounds_state1_squared_column =
        dynamic_params.poseidon_poseidon_partial_rounds_state1_squared_column;
    let poseidon_poseidon_partial_rounds_state1_squared_offset =
        dynamic_params.poseidon_poseidon_partial_rounds_state1_squared_offset;
    let poseidon_row_ratio = dynamic_params.poseidon_row_ratio;
    let range_check16_perm_cum_prod0_column = dynamic_params.range_check16_perm_cum_prod0_column;
    let range_check16_perm_cum_prod0_offset = dynamic_params.range_check16_perm_cum_prod0_offset;
    let range_check16_sorted_column = dynamic_params.range_check16_sorted_column;
    let range_check16_sorted_offset = dynamic_params.range_check16_sorted_offset;
    let range_check16_pool_column = dynamic_params.range_check16_pool_column;
    let range_check16_pool_offset = dynamic_params.range_check16_pool_offset;
    let range_check96_builtin_inner_range_check0_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check0_suboffset;
    let range_check96_builtin_inner_range_check1_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check1_suboffset;
    let range_check96_builtin_inner_range_check2_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check2_suboffset;
    let range_check96_builtin_inner_range_check3_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check3_suboffset;
    let range_check96_builtin_inner_range_check4_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check4_suboffset;
    let range_check96_builtin_inner_range_check5_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check5_suboffset;
    let range_check96_builtin_mem_suboffset = dynamic_params.range_check96_builtin_mem_suboffset;
    let range_check96_builtin_row_ratio = dynamic_params.range_check96_builtin_row_ratio;
    let range_check_builtin_inner_range_check_suboffset =
        dynamic_params.range_check_builtin_inner_range_check_suboffset;
    let range_check_builtin_mem_suboffset = dynamic_params.range_check_builtin_mem_suboffset;
    let range_check_builtin_row_ratio = dynamic_params.range_check_builtin_row_ratio;
    let range_check_units_row_ratio = dynamic_params.range_check_units_row_ratio;
    let uses_add_mod_builtin = dynamic_params.uses_add_mod_builtin;
    let uses_bitwise_builtin = dynamic_params.uses_bitwise_builtin;
    let uses_ec_op_builtin = dynamic_params.uses_ec_op_builtin;
    let uses_ecdsa_builtin = dynamic_params.uses_ecdsa_builtin;
    let uses_keccak_builtin = dynamic_params.uses_keccak_builtin;
    let uses_mul_mod_builtin = dynamic_params.uses_mul_mod_builtin;
    let uses_pedersen_builtin = dynamic_params.uses_pedersen_builtin;
    let uses_poseidon_builtin = dynamic_params.uses_poseidon_builtin;
    let uses_range_check96_builtin = dynamic_params.uses_range_check96_builtin;
    let uses_range_check_builtin = dynamic_params.uses_range_check_builtin;

    todo!()
}

pub fn eval_oods_polynomial_inner<Layout: LayoutTrait>(
    column_values: &[Felt],
    oods_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    oods_point: &Felt,
    trace_generator: &Felt,
    dynamic_params: &DynamicParams,
) -> Felt {
    // Fetch dynamic params.
    let add_mod_a0_suboffset = dynamic_params.add_mod_a0_suboffset;
    let add_mod_a1_suboffset = dynamic_params.add_mod_a1_suboffset;
    let add_mod_a2_suboffset = dynamic_params.add_mod_a2_suboffset;
    let add_mod_a3_suboffset = dynamic_params.add_mod_a3_suboffset;
    let add_mod_a_offset_suboffset = dynamic_params.add_mod_a_offset_suboffset;
    let add_mod_b0_suboffset = dynamic_params.add_mod_b0_suboffset;
    let add_mod_b1_suboffset = dynamic_params.add_mod_b1_suboffset;
    let add_mod_b2_suboffset = dynamic_params.add_mod_b2_suboffset;
    let add_mod_b3_suboffset = dynamic_params.add_mod_b3_suboffset;
    let add_mod_b_offset_suboffset = dynamic_params.add_mod_b_offset_suboffset;
    let add_mod_c0_suboffset = dynamic_params.add_mod_c0_suboffset;
    let add_mod_c1_suboffset = dynamic_params.add_mod_c1_suboffset;
    let add_mod_c2_suboffset = dynamic_params.add_mod_c2_suboffset;
    let add_mod_c3_suboffset = dynamic_params.add_mod_c3_suboffset;
    let add_mod_c_offset_suboffset = dynamic_params.add_mod_c_offset_suboffset;
    let add_mod_carry1_bit_column = dynamic_params.add_mod_carry1_bit_column;
    let add_mod_carry1_bit_offset = dynamic_params.add_mod_carry1_bit_offset;
    let add_mod_carry1_sign_column = dynamic_params.add_mod_carry1_sign_column;
    let add_mod_carry1_sign_offset = dynamic_params.add_mod_carry1_sign_offset;
    let add_mod_carry2_bit_column = dynamic_params.add_mod_carry2_bit_column;
    let add_mod_carry2_bit_offset = dynamic_params.add_mod_carry2_bit_offset;
    let add_mod_carry2_sign_column = dynamic_params.add_mod_carry2_sign_column;
    let add_mod_carry2_sign_offset = dynamic_params.add_mod_carry2_sign_offset;
    let add_mod_carry3_bit_column = dynamic_params.add_mod_carry3_bit_column;
    let add_mod_carry3_bit_offset = dynamic_params.add_mod_carry3_bit_offset;
    let add_mod_carry3_sign_column = dynamic_params.add_mod_carry3_sign_column;
    let add_mod_carry3_sign_offset = dynamic_params.add_mod_carry3_sign_offset;
    let add_mod_n_suboffset = dynamic_params.add_mod_n_suboffset;
    let add_mod_offsets_ptr_suboffset = dynamic_params.add_mod_offsets_ptr_suboffset;
    let add_mod_p0_suboffset = dynamic_params.add_mod_p0_suboffset;
    let add_mod_p1_suboffset = dynamic_params.add_mod_p1_suboffset;
    let add_mod_p2_suboffset = dynamic_params.add_mod_p2_suboffset;
    let add_mod_p3_suboffset = dynamic_params.add_mod_p3_suboffset;
    let add_mod_row_ratio = dynamic_params.add_mod_row_ratio;
    let add_mod_sub_p_bit_column = dynamic_params.add_mod_sub_p_bit_column;
    let add_mod_sub_p_bit_offset = dynamic_params.add_mod_sub_p_bit_offset;
    let add_mod_values_ptr_suboffset = dynamic_params.add_mod_values_ptr_suboffset;
    let bitwise_diluted_var_pool_suboffset = dynamic_params.bitwise_diluted_var_pool_suboffset;
    let bitwise_row_ratio = dynamic_params.bitwise_row_ratio;
    let bitwise_trim_unpacking192_suboffset = dynamic_params.bitwise_trim_unpacking192_suboffset;
    let bitwise_trim_unpacking193_suboffset = dynamic_params.bitwise_trim_unpacking193_suboffset;
    let bitwise_trim_unpacking194_suboffset = dynamic_params.bitwise_trim_unpacking194_suboffset;
    let bitwise_trim_unpacking195_suboffset = dynamic_params.bitwise_trim_unpacking195_suboffset;
    let bitwise_var_pool_suboffset = dynamic_params.bitwise_var_pool_suboffset;
    let bitwise_x_or_y_suboffset = dynamic_params.bitwise_x_or_y_suboffset;
    let cpu_decode_mem_inst_suboffset = dynamic_params.cpu_decode_mem_inst_suboffset;
    let cpu_decode_off0_suboffset = dynamic_params.cpu_decode_off0_suboffset;
    let cpu_decode_off1_suboffset = dynamic_params.cpu_decode_off1_suboffset;
    let cpu_decode_off2_suboffset = dynamic_params.cpu_decode_off2_suboffset;
    let cpu_decode_opcode_range_check_column_column =
        dynamic_params.cpu_decode_opcode_range_check_column_column;
    let cpu_decode_opcode_range_check_column_offset =
        dynamic_params.cpu_decode_opcode_range_check_column_offset;
    let cpu_operands_mem_dst_suboffset = dynamic_params.cpu_operands_mem_dst_suboffset;
    let cpu_operands_mem_op0_suboffset = dynamic_params.cpu_operands_mem_op0_suboffset;
    let cpu_operands_mem_op1_suboffset = dynamic_params.cpu_operands_mem_op1_suboffset;
    let cpu_operands_ops_mul_column = dynamic_params.cpu_operands_ops_mul_column;
    let cpu_operands_ops_mul_offset = dynamic_params.cpu_operands_ops_mul_offset;
    let cpu_operands_res_column = dynamic_params.cpu_operands_res_column;
    let cpu_operands_res_offset = dynamic_params.cpu_operands_res_offset;
    let cpu_registers_ap_column = dynamic_params.cpu_registers_ap_column;
    let cpu_registers_ap_offset = dynamic_params.cpu_registers_ap_offset;
    let cpu_registers_fp_column = dynamic_params.cpu_registers_fp_column;
    let cpu_registers_fp_offset = dynamic_params.cpu_registers_fp_offset;
    let cpu_update_registers_update_pc_tmp0_column =
        dynamic_params.cpu_update_registers_update_pc_tmp0_column;
    let cpu_update_registers_update_pc_tmp0_offset =
        dynamic_params.cpu_update_registers_update_pc_tmp0_offset;
    let cpu_update_registers_update_pc_tmp1_column =
        dynamic_params.cpu_update_registers_update_pc_tmp1_column;
    let cpu_update_registers_update_pc_tmp1_offset =
        dynamic_params.cpu_update_registers_update_pc_tmp1_offset;
    let cpu_component_step = dynamic_params.cpu_component_step;
    let diluted_check_cumulative_value_column =
        dynamic_params.diluted_check_cumulative_value_column;
    let diluted_check_cumulative_value_offset =
        dynamic_params.diluted_check_cumulative_value_offset;
    let diluted_check_permutation_cum_prod0_column =
        dynamic_params.diluted_check_permutation_cum_prod0_column;
    let diluted_check_permutation_cum_prod0_offset =
        dynamic_params.diluted_check_permutation_cum_prod0_offset;
    let diluted_check_permuted_values_column = dynamic_params.diluted_check_permuted_values_column;
    let diluted_check_permuted_values_offset = dynamic_params.diluted_check_permuted_values_offset;
    let diluted_pool_column = dynamic_params.diluted_pool_column;
    let diluted_pool_offset = dynamic_params.diluted_pool_offset;
    let diluted_units_row_ratio = dynamic_params.diluted_units_row_ratio;
    let ec_op_doubled_points_x_column = dynamic_params.ec_op_doubled_points_x_column;
    let ec_op_doubled_points_x_offset = dynamic_params.ec_op_doubled_points_x_offset;
    let ec_op_doubled_points_y_column = dynamic_params.ec_op_doubled_points_y_column;
    let ec_op_doubled_points_y_offset = dynamic_params.ec_op_doubled_points_y_offset;
    let ec_op_doubling_slope_column = dynamic_params.ec_op_doubling_slope_column;
    let ec_op_doubling_slope_offset = dynamic_params.ec_op_doubling_slope_offset;
    let ec_op_ec_subset_sum_bit_unpacking_prod_ones192_column =
        dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones192_column;
    let ec_op_ec_subset_sum_bit_unpacking_prod_ones192_offset =
        dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones192_offset;
    let ec_op_ec_subset_sum_bit_unpacking_prod_ones196_column =
        dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones196_column;
    let ec_op_ec_subset_sum_bit_unpacking_prod_ones196_offset =
        dynamic_params.ec_op_ec_subset_sum_bit_unpacking_prod_ones196_offset;
    let ec_op_ec_subset_sum_partial_sum_x_column =
        dynamic_params.ec_op_ec_subset_sum_partial_sum_x_column;
    let ec_op_ec_subset_sum_partial_sum_x_offset =
        dynamic_params.ec_op_ec_subset_sum_partial_sum_x_offset;
    let ec_op_ec_subset_sum_partial_sum_y_column =
        dynamic_params.ec_op_ec_subset_sum_partial_sum_y_column;
    let ec_op_ec_subset_sum_partial_sum_y_offset =
        dynamic_params.ec_op_ec_subset_sum_partial_sum_y_offset;
    let ec_op_ec_subset_sum_selector_column = dynamic_params.ec_op_ec_subset_sum_selector_column;
    let ec_op_ec_subset_sum_selector_offset = dynamic_params.ec_op_ec_subset_sum_selector_offset;
    let ec_op_ec_subset_sum_slope_column = dynamic_params.ec_op_ec_subset_sum_slope_column;
    let ec_op_ec_subset_sum_slope_offset = dynamic_params.ec_op_ec_subset_sum_slope_offset;
    let ec_op_ec_subset_sum_x_diff_inv_column =
        dynamic_params.ec_op_ec_subset_sum_x_diff_inv_column;
    let ec_op_ec_subset_sum_x_diff_inv_offset =
        dynamic_params.ec_op_ec_subset_sum_x_diff_inv_offset;
    let ec_op_m_suboffset = dynamic_params.ec_op_m_suboffset;
    let ec_op_p_x_suboffset = dynamic_params.ec_op_p_x_suboffset;
    let ec_op_p_y_suboffset = dynamic_params.ec_op_p_y_suboffset;
    let ec_op_q_x_suboffset = dynamic_params.ec_op_q_x_suboffset;
    let ec_op_q_y_suboffset = dynamic_params.ec_op_q_y_suboffset;
    let ec_op_r_x_suboffset = dynamic_params.ec_op_r_x_suboffset;
    let ec_op_r_y_suboffset = dynamic_params.ec_op_r_y_suboffset;
    let ec_op_builtin_row_ratio = dynamic_params.ec_op_builtin_row_ratio;
    let ecdsa_message_suboffset = dynamic_params.ecdsa_message_suboffset;
    let ecdsa_pubkey_suboffset = dynamic_params.ecdsa_pubkey_suboffset;
    let ecdsa_signature0_add_results_inv_column =
        dynamic_params.ecdsa_signature0_add_results_inv_column;
    let ecdsa_signature0_add_results_inv_offset =
        dynamic_params.ecdsa_signature0_add_results_inv_offset;
    let ecdsa_signature0_add_results_slope_column =
        dynamic_params.ecdsa_signature0_add_results_slope_column;
    let ecdsa_signature0_add_results_slope_offset =
        dynamic_params.ecdsa_signature0_add_results_slope_offset;
    let ecdsa_signature0_doubling_slope_column =
        dynamic_params.ecdsa_signature0_doubling_slope_column;
    let ecdsa_signature0_doubling_slope_offset =
        dynamic_params.ecdsa_signature0_doubling_slope_offset;
    let ecdsa_signature0_exponentiate_generator_partial_sum_x_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_x_column;
    let ecdsa_signature0_exponentiate_generator_partial_sum_x_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_x_offset;
    let ecdsa_signature0_exponentiate_generator_partial_sum_y_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_y_column;
    let ecdsa_signature0_exponentiate_generator_partial_sum_y_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_partial_sum_y_offset;
    let ecdsa_signature0_exponentiate_generator_selector_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_selector_column;
    let ecdsa_signature0_exponentiate_generator_selector_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_selector_offset;
    let ecdsa_signature0_exponentiate_generator_slope_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_slope_column;
    let ecdsa_signature0_exponentiate_generator_slope_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_slope_offset;
    let ecdsa_signature0_exponentiate_generator_x_diff_inv_column =
        dynamic_params.ecdsa_signature0_exponentiate_generator_x_diff_inv_column;
    let ecdsa_signature0_exponentiate_generator_x_diff_inv_offset =
        dynamic_params.ecdsa_signature0_exponentiate_generator_x_diff_inv_offset;
    let ecdsa_signature0_exponentiate_key_partial_sum_x_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_x_column;
    let ecdsa_signature0_exponentiate_key_partial_sum_x_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_x_offset;
    let ecdsa_signature0_exponentiate_key_partial_sum_y_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_y_column;
    let ecdsa_signature0_exponentiate_key_partial_sum_y_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_partial_sum_y_offset;
    let ecdsa_signature0_exponentiate_key_selector_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_selector_column;
    let ecdsa_signature0_exponentiate_key_selector_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_selector_offset;
    let ecdsa_signature0_exponentiate_key_slope_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_slope_column;
    let ecdsa_signature0_exponentiate_key_slope_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_slope_offset;
    let ecdsa_signature0_exponentiate_key_x_diff_inv_column =
        dynamic_params.ecdsa_signature0_exponentiate_key_x_diff_inv_column;
    let ecdsa_signature0_exponentiate_key_x_diff_inv_offset =
        dynamic_params.ecdsa_signature0_exponentiate_key_x_diff_inv_offset;
    let ecdsa_signature0_extract_r_inv_column =
        dynamic_params.ecdsa_signature0_extract_r_inv_column;
    let ecdsa_signature0_extract_r_inv_offset =
        dynamic_params.ecdsa_signature0_extract_r_inv_offset;
    let ecdsa_signature0_extract_r_slope_column =
        dynamic_params.ecdsa_signature0_extract_r_slope_column;
    let ecdsa_signature0_extract_r_slope_offset =
        dynamic_params.ecdsa_signature0_extract_r_slope_offset;
    let ecdsa_signature0_key_points_x_column = dynamic_params.ecdsa_signature0_key_points_x_column;
    let ecdsa_signature0_key_points_x_offset = dynamic_params.ecdsa_signature0_key_points_x_offset;
    let ecdsa_signature0_key_points_y_column = dynamic_params.ecdsa_signature0_key_points_y_column;
    let ecdsa_signature0_key_points_y_offset = dynamic_params.ecdsa_signature0_key_points_y_offset;
    let ecdsa_signature0_q_x_squared_column = dynamic_params.ecdsa_signature0_q_x_squared_column;
    let ecdsa_signature0_q_x_squared_offset = dynamic_params.ecdsa_signature0_q_x_squared_offset;
    let ecdsa_signature0_r_w_inv_column = dynamic_params.ecdsa_signature0_r_w_inv_column;
    let ecdsa_signature0_r_w_inv_offset = dynamic_params.ecdsa_signature0_r_w_inv_offset;
    let ecdsa_signature0_z_inv_column = dynamic_params.ecdsa_signature0_z_inv_column;
    let ecdsa_signature0_z_inv_offset = dynamic_params.ecdsa_signature0_z_inv_offset;
    let ecdsa_builtin_row_ratio = dynamic_params.ecdsa_builtin_row_ratio;
    let keccak_input_output_suboffset = dynamic_params.keccak_input_output_suboffset;
    let keccak_keccak_diluted_column0_suboffset =
        dynamic_params.keccak_keccak_diluted_column0_suboffset;
    let keccak_keccak_diluted_column1_suboffset =
        dynamic_params.keccak_keccak_diluted_column1_suboffset;
    let keccak_keccak_diluted_column2_suboffset =
        dynamic_params.keccak_keccak_diluted_column2_suboffset;
    let keccak_keccak_diluted_column3_suboffset =
        dynamic_params.keccak_keccak_diluted_column3_suboffset;
    let keccak_keccak_parse_to_diluted_cumulative_sum_column =
        dynamic_params.keccak_keccak_parse_to_diluted_cumulative_sum_column;
    let keccak_keccak_parse_to_diluted_cumulative_sum_offset =
        dynamic_params.keccak_keccak_parse_to_diluted_cumulative_sum_offset;
    let keccak_keccak_parse_to_diluted_final_reshaped_input_column =
        dynamic_params.keccak_keccak_parse_to_diluted_final_reshaped_input_column;
    let keccak_keccak_parse_to_diluted_final_reshaped_input_offset =
        dynamic_params.keccak_keccak_parse_to_diluted_final_reshaped_input_offset;
    let keccak_keccak_parse_to_diluted_reshaped_intermediate_column =
        dynamic_params.keccak_keccak_parse_to_diluted_reshaped_intermediate_column;
    let keccak_keccak_parse_to_diluted_reshaped_intermediate_offset =
        dynamic_params.keccak_keccak_parse_to_diluted_reshaped_intermediate_offset;
    let keccak_keccak_rotated_parity0_column = dynamic_params.keccak_keccak_rotated_parity0_column;
    let keccak_keccak_rotated_parity0_offset = dynamic_params.keccak_keccak_rotated_parity0_offset;
    let keccak_keccak_rotated_parity1_column = dynamic_params.keccak_keccak_rotated_parity1_column;
    let keccak_keccak_rotated_parity1_offset = dynamic_params.keccak_keccak_rotated_parity1_offset;
    let keccak_keccak_rotated_parity2_column = dynamic_params.keccak_keccak_rotated_parity2_column;
    let keccak_keccak_rotated_parity2_offset = dynamic_params.keccak_keccak_rotated_parity2_offset;
    let keccak_keccak_rotated_parity3_column = dynamic_params.keccak_keccak_rotated_parity3_column;
    let keccak_keccak_rotated_parity3_offset = dynamic_params.keccak_keccak_rotated_parity3_offset;
    let keccak_keccak_rotated_parity4_column = dynamic_params.keccak_keccak_rotated_parity4_column;
    let keccak_keccak_rotated_parity4_offset = dynamic_params.keccak_keccak_rotated_parity4_offset;
    let keccak_row_ratio = dynamic_params.keccak_row_ratio;
    let mem_pool_addr_column = dynamic_params.mem_pool_addr_column;
    let mem_pool_addr_offset = dynamic_params.mem_pool_addr_offset;
    let mem_pool_value_column = dynamic_params.mem_pool_value_column;
    let mem_pool_value_offset = dynamic_params.mem_pool_value_offset;
    let memory_multi_column_perm_perm_cum_prod0_column =
        dynamic_params.memory_multi_column_perm_perm_cum_prod0_column;
    let memory_multi_column_perm_perm_cum_prod0_offset =
        dynamic_params.memory_multi_column_perm_perm_cum_prod0_offset;
    let memory_sorted_addr_column = dynamic_params.memory_sorted_addr_column;
    let memory_sorted_addr_offset = dynamic_params.memory_sorted_addr_offset;
    let memory_sorted_value_column = dynamic_params.memory_sorted_value_column;
    let memory_sorted_value_offset = dynamic_params.memory_sorted_value_offset;
    let memory_units_row_ratio = dynamic_params.memory_units_row_ratio;
    let mul_mod_a0_suboffset = dynamic_params.mul_mod_a0_suboffset;
    let mul_mod_a1_suboffset = dynamic_params.mul_mod_a1_suboffset;
    let mul_mod_a2_suboffset = dynamic_params.mul_mod_a2_suboffset;
    let mul_mod_a3_suboffset = dynamic_params.mul_mod_a3_suboffset;
    let mul_mod_a_offset_suboffset = dynamic_params.mul_mod_a_offset_suboffset;
    let mul_mod_b0_suboffset = dynamic_params.mul_mod_b0_suboffset;
    let mul_mod_b1_suboffset = dynamic_params.mul_mod_b1_suboffset;
    let mul_mod_b2_suboffset = dynamic_params.mul_mod_b2_suboffset;
    let mul_mod_b3_suboffset = dynamic_params.mul_mod_b3_suboffset;
    let mul_mod_b_offset_suboffset = dynamic_params.mul_mod_b_offset_suboffset;
    let mul_mod_c0_suboffset = dynamic_params.mul_mod_c0_suboffset;
    let mul_mod_c1_suboffset = dynamic_params.mul_mod_c1_suboffset;
    let mul_mod_c2_suboffset = dynamic_params.mul_mod_c2_suboffset;
    let mul_mod_c3_suboffset = dynamic_params.mul_mod_c3_suboffset;
    let mul_mod_c_offset_suboffset = dynamic_params.mul_mod_c_offset_suboffset;
    let mul_mod_carry0_part0_suboffset = dynamic_params.mul_mod_carry0_part0_suboffset;
    let mul_mod_carry0_part1_suboffset = dynamic_params.mul_mod_carry0_part1_suboffset;
    let mul_mod_carry0_part2_suboffset = dynamic_params.mul_mod_carry0_part2_suboffset;
    let mul_mod_carry0_part3_suboffset = dynamic_params.mul_mod_carry0_part3_suboffset;
    let mul_mod_carry0_part4_suboffset = dynamic_params.mul_mod_carry0_part4_suboffset;
    let mul_mod_carry0_part5_suboffset = dynamic_params.mul_mod_carry0_part5_suboffset;
    let mul_mod_carry0_part6_suboffset = dynamic_params.mul_mod_carry0_part6_suboffset;
    let mul_mod_carry1_part0_suboffset = dynamic_params.mul_mod_carry1_part0_suboffset;
    let mul_mod_carry1_part1_suboffset = dynamic_params.mul_mod_carry1_part1_suboffset;
    let mul_mod_carry1_part2_suboffset = dynamic_params.mul_mod_carry1_part2_suboffset;
    let mul_mod_carry1_part3_suboffset = dynamic_params.mul_mod_carry1_part3_suboffset;
    let mul_mod_carry1_part4_suboffset = dynamic_params.mul_mod_carry1_part4_suboffset;
    let mul_mod_carry1_part5_suboffset = dynamic_params.mul_mod_carry1_part5_suboffset;
    let mul_mod_carry1_part6_suboffset = dynamic_params.mul_mod_carry1_part6_suboffset;
    let mul_mod_carry2_part0_suboffset = dynamic_params.mul_mod_carry2_part0_suboffset;
    let mul_mod_carry2_part1_suboffset = dynamic_params.mul_mod_carry2_part1_suboffset;
    let mul_mod_carry2_part2_suboffset = dynamic_params.mul_mod_carry2_part2_suboffset;
    let mul_mod_carry2_part3_suboffset = dynamic_params.mul_mod_carry2_part3_suboffset;
    let mul_mod_carry2_part4_suboffset = dynamic_params.mul_mod_carry2_part4_suboffset;
    let mul_mod_carry2_part5_suboffset = dynamic_params.mul_mod_carry2_part5_suboffset;
    let mul_mod_carry2_part6_suboffset = dynamic_params.mul_mod_carry2_part6_suboffset;
    let mul_mod_carry3_part0_suboffset = dynamic_params.mul_mod_carry3_part0_suboffset;
    let mul_mod_carry3_part1_suboffset = dynamic_params.mul_mod_carry3_part1_suboffset;
    let mul_mod_carry3_part2_suboffset = dynamic_params.mul_mod_carry3_part2_suboffset;
    let mul_mod_carry3_part3_suboffset = dynamic_params.mul_mod_carry3_part3_suboffset;
    let mul_mod_carry3_part4_suboffset = dynamic_params.mul_mod_carry3_part4_suboffset;
    let mul_mod_carry3_part5_suboffset = dynamic_params.mul_mod_carry3_part5_suboffset;
    let mul_mod_carry3_part6_suboffset = dynamic_params.mul_mod_carry3_part6_suboffset;
    let mul_mod_carry4_part0_suboffset = dynamic_params.mul_mod_carry4_part0_suboffset;
    let mul_mod_carry4_part1_suboffset = dynamic_params.mul_mod_carry4_part1_suboffset;
    let mul_mod_carry4_part2_suboffset = dynamic_params.mul_mod_carry4_part2_suboffset;
    let mul_mod_carry4_part3_suboffset = dynamic_params.mul_mod_carry4_part3_suboffset;
    let mul_mod_carry4_part4_suboffset = dynamic_params.mul_mod_carry4_part4_suboffset;
    let mul_mod_carry4_part5_suboffset = dynamic_params.mul_mod_carry4_part5_suboffset;
    let mul_mod_carry4_part6_suboffset = dynamic_params.mul_mod_carry4_part6_suboffset;
    let mul_mod_carry5_part0_suboffset = dynamic_params.mul_mod_carry5_part0_suboffset;
    let mul_mod_carry5_part1_suboffset = dynamic_params.mul_mod_carry5_part1_suboffset;
    let mul_mod_carry5_part2_suboffset = dynamic_params.mul_mod_carry5_part2_suboffset;
    let mul_mod_carry5_part3_suboffset = dynamic_params.mul_mod_carry5_part3_suboffset;
    let mul_mod_carry5_part4_suboffset = dynamic_params.mul_mod_carry5_part4_suboffset;
    let mul_mod_carry5_part5_suboffset = dynamic_params.mul_mod_carry5_part5_suboffset;
    let mul_mod_carry5_part6_suboffset = dynamic_params.mul_mod_carry5_part6_suboffset;
    let mul_mod_n_suboffset = dynamic_params.mul_mod_n_suboffset;
    let mul_mod_offsets_ptr_suboffset = dynamic_params.mul_mod_offsets_ptr_suboffset;
    let mul_mod_p0_suboffset = dynamic_params.mul_mod_p0_suboffset;
    let mul_mod_p1_suboffset = dynamic_params.mul_mod_p1_suboffset;
    let mul_mod_p2_suboffset = dynamic_params.mul_mod_p2_suboffset;
    let mul_mod_p3_suboffset = dynamic_params.mul_mod_p3_suboffset;
    let mul_mod_p_multiplier0_part0_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part0_suboffset;
    let mul_mod_p_multiplier0_part1_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part1_suboffset;
    let mul_mod_p_multiplier0_part2_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part2_suboffset;
    let mul_mod_p_multiplier0_part3_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part3_suboffset;
    let mul_mod_p_multiplier0_part4_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part4_suboffset;
    let mul_mod_p_multiplier0_part5_suboffset =
        dynamic_params.mul_mod_p_multiplier0_part5_suboffset;
    let mul_mod_p_multiplier1_part0_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part0_suboffset;
    let mul_mod_p_multiplier1_part1_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part1_suboffset;
    let mul_mod_p_multiplier1_part2_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part2_suboffset;
    let mul_mod_p_multiplier1_part3_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part3_suboffset;
    let mul_mod_p_multiplier1_part4_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part4_suboffset;
    let mul_mod_p_multiplier1_part5_suboffset =
        dynamic_params.mul_mod_p_multiplier1_part5_suboffset;
    let mul_mod_p_multiplier2_part0_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part0_suboffset;
    let mul_mod_p_multiplier2_part1_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part1_suboffset;
    let mul_mod_p_multiplier2_part2_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part2_suboffset;
    let mul_mod_p_multiplier2_part3_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part3_suboffset;
    let mul_mod_p_multiplier2_part4_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part4_suboffset;
    let mul_mod_p_multiplier2_part5_suboffset =
        dynamic_params.mul_mod_p_multiplier2_part5_suboffset;
    let mul_mod_p_multiplier3_part0_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part0_suboffset;
    let mul_mod_p_multiplier3_part1_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part1_suboffset;
    let mul_mod_p_multiplier3_part2_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part2_suboffset;
    let mul_mod_p_multiplier3_part3_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part3_suboffset;
    let mul_mod_p_multiplier3_part4_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part4_suboffset;
    let mul_mod_p_multiplier3_part5_suboffset =
        dynamic_params.mul_mod_p_multiplier3_part5_suboffset;
    let mul_mod_row_ratio = dynamic_params.mul_mod_row_ratio;
    let mul_mod_values_ptr_suboffset = dynamic_params.mul_mod_values_ptr_suboffset;
    let num_columns_first = dynamic_params.num_columns_first;
    let num_columns_second = dynamic_params.num_columns_second;
    let orig_public_memory_suboffset = dynamic_params.orig_public_memory_suboffset;
    let pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_column;
    let pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_offset;
    let pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_column;
    let pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_offset;
    let pedersen_hash0_ec_subset_sum_partial_sum_x_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_x_column;
    let pedersen_hash0_ec_subset_sum_partial_sum_x_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_x_offset;
    let pedersen_hash0_ec_subset_sum_partial_sum_y_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_y_column;
    let pedersen_hash0_ec_subset_sum_partial_sum_y_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_partial_sum_y_offset;
    let pedersen_hash0_ec_subset_sum_selector_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_selector_column;
    let pedersen_hash0_ec_subset_sum_selector_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_selector_offset;
    let pedersen_hash0_ec_subset_sum_slope_column =
        dynamic_params.pedersen_hash0_ec_subset_sum_slope_column;
    let pedersen_hash0_ec_subset_sum_slope_offset =
        dynamic_params.pedersen_hash0_ec_subset_sum_slope_offset;
    let pedersen_input0_suboffset = dynamic_params.pedersen_input0_suboffset;
    let pedersen_input1_suboffset = dynamic_params.pedersen_input1_suboffset;
    let pedersen_output_suboffset = dynamic_params.pedersen_output_suboffset;
    let pedersen_builtin_row_ratio = dynamic_params.pedersen_builtin_row_ratio;
    let poseidon_param_0_input_output_suboffset =
        dynamic_params.poseidon_param_0_input_output_suboffset;
    let poseidon_param_1_input_output_suboffset =
        dynamic_params.poseidon_param_1_input_output_suboffset;
    let poseidon_param_2_input_output_suboffset =
        dynamic_params.poseidon_param_2_input_output_suboffset;
    let poseidon_poseidon_full_rounds_state0_column =
        dynamic_params.poseidon_poseidon_full_rounds_state0_column;
    let poseidon_poseidon_full_rounds_state0_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state0_offset;
    let poseidon_poseidon_full_rounds_state0_squared_column =
        dynamic_params.poseidon_poseidon_full_rounds_state0_squared_column;
    let poseidon_poseidon_full_rounds_state0_squared_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state0_squared_offset;
    let poseidon_poseidon_full_rounds_state1_column =
        dynamic_params.poseidon_poseidon_full_rounds_state1_column;
    let poseidon_poseidon_full_rounds_state1_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state1_offset;
    let poseidon_poseidon_full_rounds_state1_squared_column =
        dynamic_params.poseidon_poseidon_full_rounds_state1_squared_column;
    let poseidon_poseidon_full_rounds_state1_squared_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state1_squared_offset;
    let poseidon_poseidon_full_rounds_state2_column =
        dynamic_params.poseidon_poseidon_full_rounds_state2_column;
    let poseidon_poseidon_full_rounds_state2_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state2_offset;
    let poseidon_poseidon_full_rounds_state2_squared_column =
        dynamic_params.poseidon_poseidon_full_rounds_state2_squared_column;
    let poseidon_poseidon_full_rounds_state2_squared_offset =
        dynamic_params.poseidon_poseidon_full_rounds_state2_squared_offset;
    let poseidon_poseidon_partial_rounds_state0_column =
        dynamic_params.poseidon_poseidon_partial_rounds_state0_column;
    let poseidon_poseidon_partial_rounds_state0_offset =
        dynamic_params.poseidon_poseidon_partial_rounds_state0_offset;
    let poseidon_poseidon_partial_rounds_state0_squared_column =
        dynamic_params.poseidon_poseidon_partial_rounds_state0_squared_column;
    let poseidon_poseidon_partial_rounds_state0_squared_offset =
        dynamic_params.poseidon_poseidon_partial_rounds_state0_squared_offset;
    let poseidon_poseidon_partial_rounds_state1_column =
        dynamic_params.poseidon_poseidon_partial_rounds_state1_column;
    let poseidon_poseidon_partial_rounds_state1_offset =
        dynamic_params.poseidon_poseidon_partial_rounds_state1_offset;
    let poseidon_poseidon_partial_rounds_state1_squared_column =
        dynamic_params.poseidon_poseidon_partial_rounds_state1_squared_column;
    let poseidon_poseidon_partial_rounds_state1_squared_offset =
        dynamic_params.poseidon_poseidon_partial_rounds_state1_squared_offset;
    let poseidon_row_ratio = dynamic_params.poseidon_row_ratio;
    let range_check16_perm_cum_prod0_column = dynamic_params.range_check16_perm_cum_prod0_column;
    let range_check16_perm_cum_prod0_offset = dynamic_params.range_check16_perm_cum_prod0_offset;
    let range_check16_sorted_column = dynamic_params.range_check16_sorted_column;
    let range_check16_sorted_offset = dynamic_params.range_check16_sorted_offset;
    let range_check16_pool_column = dynamic_params.range_check16_pool_column;
    let range_check16_pool_offset = dynamic_params.range_check16_pool_offset;
    let range_check96_builtin_inner_range_check0_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check0_suboffset;
    let range_check96_builtin_inner_range_check1_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check1_suboffset;
    let range_check96_builtin_inner_range_check2_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check2_suboffset;
    let range_check96_builtin_inner_range_check3_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check3_suboffset;
    let range_check96_builtin_inner_range_check4_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check4_suboffset;
    let range_check96_builtin_inner_range_check5_suboffset =
        dynamic_params.range_check96_builtin_inner_range_check5_suboffset;
    let range_check96_builtin_mem_suboffset = dynamic_params.range_check96_builtin_mem_suboffset;
    let range_check96_builtin_row_ratio = dynamic_params.range_check96_builtin_row_ratio;
    let range_check_builtin_inner_range_check_suboffset =
        dynamic_params.range_check_builtin_inner_range_check_suboffset;
    let range_check_builtin_mem_suboffset = dynamic_params.range_check_builtin_mem_suboffset;
    let range_check_builtin_row_ratio = dynamic_params.range_check_builtin_row_ratio;
    let range_check_units_row_ratio = dynamic_params.range_check_units_row_ratio;
    let uses_add_mod_builtin = dynamic_params.uses_add_mod_builtin;
    let uses_bitwise_builtin = dynamic_params.uses_bitwise_builtin;
    let uses_ec_op_builtin = dynamic_params.uses_ec_op_builtin;
    let uses_ecdsa_builtin = dynamic_params.uses_ecdsa_builtin;
    let uses_keccak_builtin = dynamic_params.uses_keccak_builtin;
    let uses_mul_mod_builtin = dynamic_params.uses_mul_mod_builtin;
    let uses_pedersen_builtin = dynamic_params.uses_pedersen_builtin;
    let uses_poseidon_builtin = dynamic_params.uses_poseidon_builtin;
    let uses_range_check96_builtin = dynamic_params.uses_range_check96_builtin;
    let uses_range_check_builtin = dynamic_params.uses_range_check_builtin;

    todo!()
}

pub fn check_asserts(
    _dynamic_params: &DynamicParams,
    _stark_domains: StarkDomains,
) -> Result<(), CheckAssertsError> {
    Ok(())
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum CheckAssertsError {
    #[error("value is not power of two")]
    ValueNotPowerOfTwo,

    #[error("value out of range")]
    ValueOutOfRange,
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum CheckAssertsError {
    #[error("value is not power of two")]
    ValueNotPowerOfTwo,

    #[error("value out of range")]
    ValueOutOfRange,
}
