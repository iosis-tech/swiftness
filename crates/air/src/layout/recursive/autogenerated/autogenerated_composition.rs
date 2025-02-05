use crate::{consts::*, felt_nonzero, layout::recursive::GlobalValues};
use starknet_crypto::Felt;
use starknet_types_core::felt::NonZeroFelt;

pub fn eval_composition_polynomial_inner(
    mask_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    trace_generator: &Felt,
    global_values: &GlobalValues,
) -> Felt {
    // Compute powers.
    let pow0 = point.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_2048))));
    let pow1 = pow0 * pow0; // pow(point, (safe_div(global_values.trace_length, 1024))).
    let pow2 = point.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_128))));
    let pow3 = point.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_32))));
    let pow4 = pow3 * pow3; // pow(point, (safe_div(global_values.trace_length, 16))).
    let pow5 = point.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_4))));
    let pow6 = pow5 * pow5; // pow(point, (safe_div(global_values.trace_length, 2))).
    let pow7 = pow6 * pow6; // pow(point, global_values.trace_length).
    let pow8 = trace_generator.pow_felt(&(global_values.trace_length - FELT_128));
    let pow9 = trace_generator.pow_felt(&(global_values.trace_length - FELT_2048));
    let pow10 = trace_generator.pow_felt(&(global_values.trace_length - FELT_1));
    let pow11 = trace_generator.pow_felt(&(global_values.trace_length - FELT_4));
    let pow12 = trace_generator.pow_felt(&(global_values.trace_length - FELT_2));
    let pow13 = trace_generator.pow_felt(&(global_values.trace_length - FELT_16));
    let pow14 =
        trace_generator.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_2))));
    let pow15 = trace_generator
        .pow_felt(&((FELT_255 * global_values.trace_length).floor_div(&felt_nonzero!(FELT_256))));
    let pow16 =
        trace_generator.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_64))));
    let pow17 = pow16 * pow16; // pow(trace_generator, (safe_div(global_values.trace_length, 32))).
    let pow18 = pow16 * pow17; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 64))).
    let pow19 = pow16 * pow18; // pow(trace_generator, (safe_div(global_values.trace_length, 16))).
    let pow20 = pow16 * pow19; // pow(trace_generator, (safe_div((safe_mult(5, global_values.trace_length)), 64))).
    let pow21 = pow16 * pow20; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 32))).
    let pow22 = pow16 * pow21; // pow(trace_generator, (safe_div((safe_mult(7, global_values.trace_length)), 64))).
    let pow23 = pow16 * pow22; // pow(trace_generator, (safe_div(global_values.trace_length, 8))).
    let pow24 = pow16 * pow23; // pow(trace_generator, (safe_div((safe_mult(9, global_values.trace_length)), 64))).
    let pow25 = pow16 * pow24; // pow(trace_generator, (safe_div((safe_mult(5, global_values.trace_length)), 32))).
    let pow26 = pow16 * pow25; // pow(trace_generator, (safe_div((safe_mult(11, global_values.trace_length)), 64))).
    let pow27 = pow16 * pow26; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 16))).
    let pow28 = pow16 * pow27; // pow(trace_generator, (safe_div((safe_mult(13, global_values.trace_length)), 64))).
    let pow29 = pow16 * pow28; // pow(trace_generator, (safe_div((safe_mult(7, global_values.trace_length)), 32))).
    let pow30 = pow16 * pow29; // pow(trace_generator, (safe_div((safe_mult(15, global_values.trace_length)), 64))).
    let pow31 = trace_generator
        .pow_felt(&((FELT_3 * global_values.trace_length).floor_div(&felt_nonzero!(FELT_4))));
    let pow32 = pow27 * pow31; // pow(trace_generator, (safe_div((safe_mult(15, global_values.trace_length)), 16))).
    let pow33 = pow18 * pow32; // pow(trace_generator, (safe_div((safe_mult(63, global_values.trace_length)), 64))).

    // Compute domains.
    let domain0 = pow7 - FELT_1;
    let domain1 = pow6 - FELT_1;
    let domain2 = pow5 - FELT_1;
    let domain3 = pow4 - pow32;
    let domain4 = pow4 - FELT_1;
    let domain5 = pow3 - FELT_1;
    let domain6 = pow2 - FELT_1;
    let domain7 = pow2 - pow31;
    let temp = pow2 - pow16;
    let temp = temp * (pow2 - pow17);
    let temp = temp * (pow2 - pow18);
    let temp = temp * (pow2 - pow19);
    let temp = temp * (pow2 - pow20);
    let temp = temp * (pow2 - pow21);
    let temp = temp * (pow2 - pow22);
    let temp = temp * (pow2 - pow23);
    let temp = temp * (pow2 - pow24);
    let temp = temp * (pow2 - pow25);
    let temp = temp * (pow2 - pow26);
    let temp = temp * (pow2 - pow27);
    let temp = temp * (pow2 - pow28);
    let temp = temp * (pow2 - pow29);
    let temp = temp * (pow2 - pow30);
    let domain8 = temp * (domain6);
    let domain9 = pow1 - FELT_1;
    let domain10 = pow1 - pow15;
    let domain11 = pow1 - pow33;
    let domain12 = pow0 - pow14;
    let domain13 = pow0 - FELT_1;
    let domain14 = point - pow13;
    let domain15 = point - FELT_1;
    let domain16 = point - pow12;
    let domain17 = point - pow11;
    let domain18 = point - pow10;
    let domain19 = point - pow9;
    let domain20 = point - pow8;

    // Fetch mask variables.
    let column0_row0 = mask_values[0];
    let column0_row1 = mask_values[1];
    let column0_row2 = mask_values[2];
    let column0_row3 = mask_values[3];
    let column0_row4 = mask_values[4];
    let column0_row5 = mask_values[5];
    let column0_row6 = mask_values[6];
    let column0_row7 = mask_values[7];
    let column0_row8 = mask_values[8];
    let column0_row9 = mask_values[9];
    let column0_row10 = mask_values[10];
    let column0_row11 = mask_values[11];
    let column0_row12 = mask_values[12];
    let column0_row13 = mask_values[13];
    let column0_row14 = mask_values[14];
    let column0_row15 = mask_values[15];
    let column1_row0 = mask_values[16];
    let column1_row1 = mask_values[17];
    let column1_row2 = mask_values[18];
    let column1_row4 = mask_values[19];
    let column1_row6 = mask_values[20];
    let column1_row8 = mask_values[21];
    let column1_row10 = mask_values[22];
    let column1_row12 = mask_values[23];
    let column1_row14 = mask_values[24];
    let column1_row16 = mask_values[25];
    let column1_row18 = mask_values[26];
    let column1_row20 = mask_values[27];
    let column1_row22 = mask_values[28];
    let column1_row24 = mask_values[29];
    let column1_row26 = mask_values[30];
    let column1_row28 = mask_values[31];
    let column1_row30 = mask_values[32];
    let column1_row32 = mask_values[33];
    let column1_row33 = mask_values[34];
    let column1_row64 = mask_values[35];
    let column1_row65 = mask_values[36];
    let column1_row88 = mask_values[37];
    let column1_row90 = mask_values[38];
    let column1_row92 = mask_values[39];
    let column1_row94 = mask_values[40];
    let column1_row96 = mask_values[41];
    let column1_row97 = mask_values[42];
    let column1_row120 = mask_values[43];
    let column1_row122 = mask_values[44];
    let column1_row124 = mask_values[45];
    let column1_row126 = mask_values[46];
    let column2_row0 = mask_values[47];
    let column2_row1 = mask_values[48];
    let column3_row0 = mask_values[49];
    let column3_row1 = mask_values[50];
    let column3_row2 = mask_values[51];
    let column3_row3 = mask_values[52];
    let column3_row4 = mask_values[53];
    let column3_row5 = mask_values[54];
    let column3_row8 = mask_values[55];
    let column3_row9 = mask_values[56];
    let column3_row10 = mask_values[57];
    let column3_row11 = mask_values[58];
    let column3_row12 = mask_values[59];
    let column3_row13 = mask_values[60];
    let column3_row16 = mask_values[61];
    let column3_row26 = mask_values[62];
    let column3_row27 = mask_values[63];
    let column3_row42 = mask_values[64];
    let column3_row43 = mask_values[65];
    let column3_row58 = mask_values[66];
    let column3_row74 = mask_values[67];
    let column3_row75 = mask_values[68];
    let column3_row91 = mask_values[69];
    let column3_row122 = mask_values[70];
    let column3_row123 = mask_values[71];
    let column3_row154 = mask_values[72];
    let column3_row202 = mask_values[73];
    let column3_row522 = mask_values[74];
    let column3_row523 = mask_values[75];
    let column3_row1034 = mask_values[76];
    let column3_row1035 = mask_values[77];
    let column3_row2058 = mask_values[78];
    let column4_row0 = mask_values[79];
    let column4_row1 = mask_values[80];
    let column4_row2 = mask_values[81];
    let column4_row3 = mask_values[82];
    let column5_row0 = mask_values[83];
    let column5_row1 = mask_values[84];
    let column5_row2 = mask_values[85];
    let column5_row3 = mask_values[86];
    let column5_row4 = mask_values[87];
    let column5_row5 = mask_values[88];
    let column5_row6 = mask_values[89];
    let column5_row7 = mask_values[90];
    let column5_row8 = mask_values[91];
    let column5_row12 = mask_values[92];
    let column5_row28 = mask_values[93];
    let column5_row44 = mask_values[94];
    let column5_row60 = mask_values[95];
    let column5_row76 = mask_values[96];
    let column5_row92 = mask_values[97];
    let column5_row108 = mask_values[98];
    let column5_row124 = mask_values[99];
    let column5_row1021 = mask_values[100];
    let column5_row1023 = mask_values[101];
    let column5_row1025 = mask_values[102];
    let column5_row1027 = mask_values[103];
    let column5_row2045 = mask_values[104];
    let column6_row0 = mask_values[105];
    let column6_row1 = mask_values[106];
    let column6_row2 = mask_values[107];
    let column6_row3 = mask_values[108];
    let column6_row4 = mask_values[109];
    let column6_row5 = mask_values[110];
    let column6_row7 = mask_values[111];
    let column6_row9 = mask_values[112];
    let column6_row11 = mask_values[113];
    let column6_row13 = mask_values[114];
    let column6_row17 = mask_values[115];
    let column6_row25 = mask_values[116];
    let column6_row768 = mask_values[117];
    let column6_row772 = mask_values[118];
    let column6_row784 = mask_values[119];
    let column6_row788 = mask_values[120];
    let column6_row1004 = mask_values[121];
    let column6_row1008 = mask_values[122];
    let column6_row1022 = mask_values[123];
    let column6_row1024 = mask_values[124];
    let column7_inter1_row0 = mask_values[125];
    let column7_inter1_row1 = mask_values[126];
    let column8_inter1_row0 = mask_values[127];
    let column8_inter1_row1 = mask_values[128];
    let column9_inter1_row0 = mask_values[129];
    let column9_inter1_row1 = mask_values[130];
    let column9_inter1_row2 = mask_values[131];
    let column9_inter1_row5 = mask_values[132];

    // Compute intermediate values.
    let cpu_decode_opcode_range_check_bit_0 = column0_row0 - (column0_row1 + column0_row1);
    let cpu_decode_opcode_range_check_bit_2 = column0_row2 - (column0_row3 + column0_row3);
    let cpu_decode_opcode_range_check_bit_4 = column0_row4 - (column0_row5 + column0_row5);
    let cpu_decode_opcode_range_check_bit_3 = column0_row3 - (column0_row4 + column0_row4);
    let cpu_decode_flag_op1_base_op0_0 = FELT_1
        - (cpu_decode_opcode_range_check_bit_2
            + cpu_decode_opcode_range_check_bit_4
            + cpu_decode_opcode_range_check_bit_3);
    let cpu_decode_opcode_range_check_bit_5 = column0_row5 - (column0_row6 + column0_row6);
    let cpu_decode_opcode_range_check_bit_6 = column0_row6 - (column0_row7 + column0_row7);
    let cpu_decode_opcode_range_check_bit_9 = column0_row9 - (column0_row10 + column0_row10);
    let cpu_decode_flag_res_op1_0 = FELT_1
        - (cpu_decode_opcode_range_check_bit_5
            + cpu_decode_opcode_range_check_bit_6
            + cpu_decode_opcode_range_check_bit_9);
    let cpu_decode_opcode_range_check_bit_7 = column0_row7 - (column0_row8 + column0_row8);
    let cpu_decode_opcode_range_check_bit_8 = column0_row8 - (column0_row9 + column0_row9);
    let cpu_decode_flag_pc_update_regular_0 = FELT_1
        - (cpu_decode_opcode_range_check_bit_7
            + cpu_decode_opcode_range_check_bit_8
            + cpu_decode_opcode_range_check_bit_9);
    let cpu_decode_opcode_range_check_bit_12 = column0_row12 - (column0_row13 + column0_row13);
    let cpu_decode_opcode_range_check_bit_13 = column0_row13 - (column0_row14 + column0_row14);
    let cpu_decode_fp_update_regular_0 =
        FELT_1 - (cpu_decode_opcode_range_check_bit_12 + cpu_decode_opcode_range_check_bit_13);
    let cpu_decode_opcode_range_check_bit_1 = column0_row1 - (column0_row2 + column0_row2);
    let npc_reg_0 = column3_row0 + cpu_decode_opcode_range_check_bit_2 + FELT_1;
    let cpu_decode_opcode_range_check_bit_10 = column0_row10 - (column0_row11 + column0_row11);
    let cpu_decode_opcode_range_check_bit_11 = column0_row11 - (column0_row12 + column0_row12);
    let cpu_decode_opcode_range_check_bit_14 = column0_row14 - (column0_row15 + column0_row15);
    let memory_address_diff_0 = column4_row2 - column4_row0;
    let range_check16_diff_0 = column5_row6 - column5_row2;
    let pedersen_hash0_ec_subset_sum_bit_0 = column6_row0 - (column6_row4 + column6_row4);
    let pedersen_hash0_ec_subset_sum_bit_neg_0 = FELT_1 - pedersen_hash0_ec_subset_sum_bit_0;
    let range_check_builtin_value0_0 = column5_row12;
    let range_check_builtin_value1_0 =
        range_check_builtin_value0_0 * global_values.offset_size + column5_row28;
    let range_check_builtin_value2_0 =
        range_check_builtin_value1_0 * global_values.offset_size + column5_row44;
    let range_check_builtin_value3_0 =
        range_check_builtin_value2_0 * global_values.offset_size + column5_row60;
    let range_check_builtin_value4_0 =
        range_check_builtin_value3_0 * global_values.offset_size + column5_row76;
    let range_check_builtin_value5_0 =
        range_check_builtin_value4_0 * global_values.offset_size + column5_row92;
    let range_check_builtin_value6_0 =
        range_check_builtin_value5_0 * global_values.offset_size + column5_row108;
    let range_check_builtin_value7_0 =
        range_check_builtin_value6_0 * global_values.offset_size + column5_row124;
    let bitwise_sum_var_0_0 = column1_row0
        + column1_row2 * FELT_2
        + column1_row4 * FELT_4
        + column1_row6 * FELT_8
        + column1_row8 * FELT_18446744073709551616
        + column1_row10 * FELT_36893488147419103232
        + column1_row12 * FELT_73786976294838206464
        + column1_row14 * FELT_147573952589676412928;
    let bitwise_sum_var_8_0 = column1_row16 * FELT_340282366920938463463374607431768211456
        + column1_row18 * FELT_680564733841876926926749214863536422912
        + column1_row20 * FELT_1361129467683753853853498429727072845824
        + column1_row22 * FELT_2722258935367507707706996859454145691648
        + column1_row24 * FELT_6277101735386680763835789423207666416102355444464034512896
        + column1_row26 * FELT_12554203470773361527671578846415332832204710888928069025792
        + column1_row28 * FELT_25108406941546723055343157692830665664409421777856138051584
        + column1_row30 * FELT_50216813883093446110686315385661331328818843555712276103168;

    // Sum constraints.
    let mut total_sum = FELT_0;
    let mut value = FELT_0;

    // Constraint: cpu/decode/opcode_range_check/bit.
    value = (cpu_decode_opcode_range_check_bit_0 * cpu_decode_opcode_range_check_bit_0
        - cpu_decode_opcode_range_check_bit_0)
        * domain3.field_div(&felt_nonzero!(domain0));
    total_sum = total_sum + constraint_coefficients[0] * value;

    // Constraint: cpu/decode/opcode_range_check/zero.
    value = (column0_row0).field_div(&felt_nonzero!(domain3));
    total_sum = total_sum + constraint_coefficients[1] * value;

    // Constraint: cpu/decode/opcode_range_check_input.
    value = (column3_row1
        - (((column0_row0 * global_values.offset_size + column5_row4)
            * global_values.offset_size
            + column5_row8)
            * global_values.offset_size
            + column5_row0))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[2] * value;

    // Constraint: cpu/decode/flag_op1_base_op0_bit.
    value = (cpu_decode_flag_op1_base_op0_0 * cpu_decode_flag_op1_base_op0_0
        - cpu_decode_flag_op1_base_op0_0)
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[3] * value;

    // Constraint: cpu/decode/flag_res_op1_bit.
    value = (cpu_decode_flag_res_op1_0 * cpu_decode_flag_res_op1_0 - cpu_decode_flag_res_op1_0)
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[4] * value;

    // Constraint: cpu/decode/flag_pc_update_regular_bit.
    value = (cpu_decode_flag_pc_update_regular_0 * cpu_decode_flag_pc_update_regular_0
        - cpu_decode_flag_pc_update_regular_0)
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[5] * value;

    // Constraint: cpu/decode/fp_update_regular_bit.
    value = (cpu_decode_fp_update_regular_0 * cpu_decode_fp_update_regular_0
        - cpu_decode_fp_update_regular_0)
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[6] * value;

    // Constraint: cpu/operands/mem_dst_addr.
    value = (column3_row8 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_0 * column6_row9
            + (FELT_1 - cpu_decode_opcode_range_check_bit_0) * column6_row1
            + column5_row0))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[7] * value;

    // Constraint: cpu/operands/mem0_addr.
    value = (column3_row4 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_1 * column6_row9
            + (FELT_1 - cpu_decode_opcode_range_check_bit_1) * column6_row1
            + column5_row8))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[8] * value;

    // Constraint: cpu/operands/mem1_addr.
    value = (column3_row12 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_2 * column3_row0
            + cpu_decode_opcode_range_check_bit_4 * column6_row1
            + cpu_decode_opcode_range_check_bit_3 * column6_row9
            + cpu_decode_flag_op1_base_op0_0 * column3_row5
            + column5_row4))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[9] * value;

    // Constraint: cpu/operands/ops_mul.
    value = (column6_row5 - column3_row5 * column3_row13).field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[10] * value;

    // Constraint: cpu/operands/res.
    value = ((FELT_1 - cpu_decode_opcode_range_check_bit_9) * column6_row13
        - (cpu_decode_opcode_range_check_bit_5 * (column3_row5 + column3_row13)
            + cpu_decode_opcode_range_check_bit_6 * column6_row5
            + cpu_decode_flag_res_op1_0 * column3_row13))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[11] * value;

    // Constraint: cpu/update_registers/update_pc/tmp0.
    value = (column6_row3 - cpu_decode_opcode_range_check_bit_9 * column3_row9)
        * domain14.field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[12] * value;

    // Constraint: cpu/update_registers/update_pc/tmp1.
    value = (column6_row11 - column6_row3 * column6_row13)
        * domain14.field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[13] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_negative.
    value = ((FELT_1 - cpu_decode_opcode_range_check_bit_9) * column3_row16
        + column6_row3 * (column3_row16 - (column3_row0 + column3_row13))
        - (cpu_decode_flag_pc_update_regular_0 * npc_reg_0
            + cpu_decode_opcode_range_check_bit_7 * column6_row13
            + cpu_decode_opcode_range_check_bit_8 * (column3_row0 + column6_row13)))
        * domain14.field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[14] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_positive.
    value = ((column6_row11 - cpu_decode_opcode_range_check_bit_9) * (column3_row16 - npc_reg_0))
        * domain14.field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[15] * value;

    // Constraint: cpu/update_registers/update_ap/ap_update.
    value = (column6_row17
        - (column6_row1
            + cpu_decode_opcode_range_check_bit_10 * column6_row13
            + cpu_decode_opcode_range_check_bit_11
            + cpu_decode_opcode_range_check_bit_12 * FELT_2))
        * domain14.field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[16] * value;

    // Constraint: cpu/update_registers/update_fp/fp_update.
    value = (column6_row25
        - (cpu_decode_fp_update_regular_0 * column6_row9
            + cpu_decode_opcode_range_check_bit_13 * column3_row9
            + cpu_decode_opcode_range_check_bit_12 * (column6_row1 + FELT_2)))
        * domain14.field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[17] * value;

    // Constraint: cpu/opcodes/call/push_fp.
    value = (cpu_decode_opcode_range_check_bit_12 * (column3_row9 - column6_row9))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[18] * value;

    // Constraint: cpu/opcodes/call/push_pc.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column3_row5 - (column3_row0 + cpu_decode_opcode_range_check_bit_2 + FELT_1)))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[19] * value;

    // Constraint: cpu/opcodes/call/off0.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column5_row0 - global_values.half_offset_size))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[20] * value;

    // Constraint: cpu/opcodes/call/off1.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column5_row8 - (global_values.half_offset_size + FELT_1)))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[21] * value;

    // Constraint: cpu/opcodes/call/flags.
    value = (cpu_decode_opcode_range_check_bit_12
        * (cpu_decode_opcode_range_check_bit_12
            + cpu_decode_opcode_range_check_bit_12
            + FELT_1
            + FELT_1
            - (cpu_decode_opcode_range_check_bit_0
                + cpu_decode_opcode_range_check_bit_1
                + FELT_4)))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[22] * value;

    // Constraint: cpu/opcodes/ret/off0.
    value = (cpu_decode_opcode_range_check_bit_13
        * (column5_row0 + FELT_2 - global_values.half_offset_size))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[23] * value;

    // Constraint: cpu/opcodes/ret/off2.
    value = (cpu_decode_opcode_range_check_bit_13
        * (column5_row4 + FELT_1 - global_values.half_offset_size))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[24] * value;

    // Constraint: cpu/opcodes/ret/flags.
    value = (cpu_decode_opcode_range_check_bit_13
        * (cpu_decode_opcode_range_check_bit_7
            + cpu_decode_opcode_range_check_bit_0
            + cpu_decode_opcode_range_check_bit_3
            + cpu_decode_flag_res_op1_0
            - FELT_4))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[25] * value;

    // Constraint: cpu/opcodes/assert_eq/assert_eq.
    value = (cpu_decode_opcode_range_check_bit_14 * (column3_row9 - column6_row13))
        .field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[26] * value;

    // Constraint: initial_ap.
    value = (column6_row1 - global_values.initial_ap).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[27] * value;

    // Constraint: initial_fp.
    value = (column6_row9 - global_values.initial_ap).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[28] * value;

    // Constraint: initial_pc.
    value = (column3_row0 - global_values.initial_pc).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[29] * value;

    // Constraint: final_ap.
    value = (column6_row1 - global_values.final_ap).field_div(&felt_nonzero!(domain14));
    total_sum = total_sum + constraint_coefficients[30] * value;

    // Constraint: final_fp.
    value = (column6_row9 - global_values.initial_ap).field_div(&felt_nonzero!(domain14));
    total_sum = total_sum + constraint_coefficients[31] * value;

    // Constraint: final_pc.
    value = (column3_row0 - global_values.final_pc).field_div(&felt_nonzero!(domain14));
    total_sum = total_sum + constraint_coefficients[32] * value;

    // Constraint: memory/multi_column_perm/perm/init0.
    value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column4_row0
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column4_row1))
        * column9_inter1_row0
        + column3_row0
        + global_values.memory_multi_column_perm_hash_interaction_elm0 * column3_row1
        - global_values.memory_multi_column_perm_perm_interaction_elm)
        .field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[33] * value;

    // Constraint: memory/multi_column_perm/perm/step0.
    value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column4_row2
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column4_row3))
        * column9_inter1_row2
        - (global_values.memory_multi_column_perm_perm_interaction_elm
            - (column3_row2
                + global_values.memory_multi_column_perm_hash_interaction_elm0 * column3_row3))
            * column9_inter1_row0)
        * domain16.field_div(&felt_nonzero!(domain1));
    total_sum = total_sum + constraint_coefficients[34] * value;

    // Constraint: memory/multi_column_perm/perm/last.
    value = (column9_inter1_row0 - global_values.memory_multi_column_perm_perm_public_memory_prod)
        .field_div(&felt_nonzero!(domain16));
    total_sum = total_sum + constraint_coefficients[35] * value;

    // Constraint: memory/diff_is_bit.
    value = (memory_address_diff_0 * memory_address_diff_0 - memory_address_diff_0)
        * domain16.field_div(&felt_nonzero!(domain1));
    total_sum = total_sum + constraint_coefficients[36] * value;

    // Constraint: memory/is_func.
    value = ((memory_address_diff_0 - FELT_1) * (column4_row1 - column4_row3))
        * domain16.field_div(&felt_nonzero!(domain1));
    total_sum = total_sum + constraint_coefficients[37] * value;

    // Constraint: memory/initial_addr.
    value = (column4_row0 - FELT_1).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[38] * value;

    // Constraint: public_memory_addr_zero.
    value = (column3_row2).field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[39] * value;

    // Constraint: public_memory_value_zero.
    value = (column3_row3).field_div(&felt_nonzero!(domain4));
    total_sum = total_sum + constraint_coefficients[40] * value;

    // Constraint: range_check16/perm/init0.
    value = ((global_values.range_check16_perm_interaction_elm - column5_row2)
        * column9_inter1_row1
        + column5_row0
        - global_values.range_check16_perm_interaction_elm)
        .field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[41] * value;

    // Constraint: range_check16/perm/step0.
    value = ((global_values.range_check16_perm_interaction_elm - column5_row6)
        * column9_inter1_row5
        - (global_values.range_check16_perm_interaction_elm - column5_row4) * column9_inter1_row1)
        * domain17.field_div(&felt_nonzero!(domain2));
    total_sum = total_sum + constraint_coefficients[42] * value;

    // Constraint: range_check16/perm/last.
    value = (column9_inter1_row1 - global_values.range_check16_perm_public_memory_prod)
        .field_div(&felt_nonzero!(domain17));
    total_sum = total_sum + constraint_coefficients[43] * value;

    // Constraint: range_check16/diff_is_bit.
    value = (range_check16_diff_0 * range_check16_diff_0 - range_check16_diff_0)
        * domain17.field_div(&felt_nonzero!(domain2));
    total_sum = total_sum + constraint_coefficients[44] * value;

    // Constraint: range_check16/minimum.
    value = (column5_row2 - global_values.range_check_min).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[45] * value;

    // Constraint: range_check16/maximum.
    value = (column5_row2 - global_values.range_check_max).field_div(&felt_nonzero!(domain17));
    total_sum = total_sum + constraint_coefficients[46] * value;

    // Constraint: diluted_check/permutation/init0.
    value = ((global_values.diluted_check_permutation_interaction_elm - column2_row0)
        * column8_inter1_row0
        + column1_row0
        - global_values.diluted_check_permutation_interaction_elm)
        .field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[47] * value;

    // Constraint: diluted_check/permutation/step0.
    value = ((global_values.diluted_check_permutation_interaction_elm - column2_row1)
        * column8_inter1_row1
        - (global_values.diluted_check_permutation_interaction_elm - column1_row1)
            * column8_inter1_row0)
        * domain18.field_div(&felt_nonzero!(domain0));
    total_sum = total_sum + constraint_coefficients[48] * value;

    // Constraint: diluted_check/permutation/last.
    value = (column8_inter1_row0 - global_values.diluted_check_permutation_public_memory_prod)
        .field_div(&felt_nonzero!(domain18));
    total_sum = total_sum + constraint_coefficients[49] * value;

    // Constraint: diluted_check/init.
    value = (column7_inter1_row0 - FELT_1).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[50] * value;

    // Constraint: diluted_check/first_element.
    value =
        (column2_row0 - global_values.diluted_check_first_elm).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[51] * value;

    // Constraint: diluted_check/step.
    value = (column7_inter1_row1
        - (column7_inter1_row0
            * (FELT_1
                + global_values.diluted_check_interaction_z * (column2_row1 - column2_row0))
            + global_values.diluted_check_interaction_alpha
                * (column2_row1 - column2_row0)
                * (column2_row1 - column2_row0)))
        * domain18.field_div(&felt_nonzero!(domain0));
    total_sum = total_sum + constraint_coefficients[52] * value;

    // Constraint: diluted_check/last.
    value = (column7_inter1_row0 - global_values.diluted_check_final_cum_val)
        .field_div(&felt_nonzero!(domain18));
    total_sum = total_sum + constraint_coefficients[53] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/last_one_is_zero.
    value = (column6_row7 * (column6_row0 - (column6_row4 + column6_row4)))
        .field_div(&felt_nonzero!(domain9));
    total_sum = total_sum + constraint_coefficients[54] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    value = (column6_row7
        * (column6_row4
            - FELT_3138550867693340381917894711603833208051177722232017256448 * column6_row768))
        .field_div(&felt_nonzero!(domain9));
    total_sum = total_sum + constraint_coefficients[55] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit192.
    value = (column6_row7 - column6_row1022 * (column6_row768 - (column6_row772 + column6_row772)))
        .field_div(&felt_nonzero!(domain9));
    total_sum = total_sum + constraint_coefficients[56] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    value = (column6_row1022 * (column6_row772 - FELT_8 * column6_row784))
        .field_div(&felt_nonzero!(domain9));
    total_sum = total_sum + constraint_coefficients[57] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit196.
    value = (column6_row1022
        - (column6_row1004 - (column6_row1008 + column6_row1008))
            * (column6_row784 - (column6_row788 + column6_row788)))
        .field_div(&felt_nonzero!(domain9));
    total_sum = total_sum + constraint_coefficients[58] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    value = ((column6_row1004 - (column6_row1008 + column6_row1008))
        * (column6_row788 - FELT_18014398509481984 * column6_row1004))
        .field_div(&felt_nonzero!(domain9));
    total_sum = total_sum + constraint_coefficients[59] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/booleanity_test.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (pedersen_hash0_ec_subset_sum_bit_0 - FELT_1))
        * domain10.field_div(&felt_nonzero!(domain2));
    total_sum = total_sum + constraint_coefficients[60] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_extraction_end.
    value = (column6_row0).field_div(&felt_nonzero!(domain11));
    total_sum = total_sum + constraint_coefficients[61] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/zeros_tail.
    value = (column6_row0).field_div(&felt_nonzero!(domain10));
    total_sum = total_sum + constraint_coefficients[62] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/slope.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (column5_row3 - global_values.pedersen_points_y)
        - column6_row2 * (column5_row1 - global_values.pedersen_points_x))
        * domain10.field_div(&felt_nonzero!(domain2));
    total_sum = total_sum + constraint_coefficients[63] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/x.
    value = (column6_row2 * column6_row2
        - pedersen_hash0_ec_subset_sum_bit_0
            * (column5_row1 + global_values.pedersen_points_x + column5_row5))
        * domain10.field_div(&felt_nonzero!(domain2));
    total_sum = total_sum + constraint_coefficients[64] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/y.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (column5_row3 + column5_row7)
        - column6_row2 * (column5_row1 - column5_row5))
        * domain10.field_div(&felt_nonzero!(domain2));
    total_sum = total_sum + constraint_coefficients[65] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/x.
    value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column5_row5 - column5_row1))
        * domain10.field_div(&felt_nonzero!(domain2));
    total_sum = total_sum + constraint_coefficients[66] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/y.
    value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column5_row7 - column5_row3))
        * domain10.field_div(&felt_nonzero!(domain2));
    total_sum = total_sum + constraint_coefficients[67] * value;

    // Constraint: pedersen/hash0/copy_point/x.
    value = (column5_row1025 - column5_row1021) * domain12.field_div(&felt_nonzero!(domain9));
    total_sum = total_sum + constraint_coefficients[68] * value;

    // Constraint: pedersen/hash0/copy_point/y.
    value = (column5_row1027 - column5_row1023) * domain12.field_div(&felt_nonzero!(domain9));
    total_sum = total_sum + constraint_coefficients[69] * value;

    // Constraint: pedersen/hash0/init/x.
    value =
        (column5_row1 - global_values.pedersen_shift_point.x).field_div(&felt_nonzero!(domain13));
    total_sum = total_sum + constraint_coefficients[70] * value;

    // Constraint: pedersen/hash0/init/y.
    value =
        (column5_row3 - global_values.pedersen_shift_point.y).field_div(&felt_nonzero!(domain13));
    total_sum = total_sum + constraint_coefficients[71] * value;

    // Constraint: pedersen/input0_value0.
    value = (column3_row11 - column6_row0).field_div(&felt_nonzero!(domain13));
    total_sum = total_sum + constraint_coefficients[72] * value;

    // Constraint: pedersen/input0_addr.
    value = (column3_row2058 - (column3_row522 + FELT_1))
        * domain19.field_div(&felt_nonzero!(domain13));
    total_sum = total_sum + constraint_coefficients[73] * value;

    // Constraint: pedersen/init_addr.
    value =
        (column3_row10 - global_values.initial_pedersen_addr).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[74] * value;

    // Constraint: pedersen/input1_value0.
    value = (column3_row1035 - column6_row1024).field_div(&felt_nonzero!(domain13));
    total_sum = total_sum + constraint_coefficients[75] * value;

    // Constraint: pedersen/input1_addr.
    value = (column3_row1034 - (column3_row10 + FELT_1)).field_div(&felt_nonzero!(domain13));
    total_sum = total_sum + constraint_coefficients[76] * value;

    // Constraint: pedersen/output_value0.
    value = (column3_row523 - column5_row2045).field_div(&felt_nonzero!(domain13));
    total_sum = total_sum + constraint_coefficients[77] * value;

    // Constraint: pedersen/output_addr.
    value = (column3_row522 - (column3_row1034 + FELT_1)).field_div(&felt_nonzero!(domain13));
    total_sum = total_sum + constraint_coefficients[78] * value;

    // Constraint: range_check_builtin/value.
    value = (range_check_builtin_value7_0 - column3_row75).field_div(&felt_nonzero!(domain6));
    total_sum = total_sum + constraint_coefficients[79] * value;

    // Constraint: range_check_builtin/addr_step.
    value =
        (column3_row202 - (column3_row74 + FELT_1)) * domain20.field_div(&felt_nonzero!(domain6));
    total_sum = total_sum + constraint_coefficients[80] * value;

    // Constraint: range_check_builtin/init_addr.
    value = (column3_row74 - global_values.initial_range_check_addr)
        .field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[81] * value;

    // Constraint: bitwise/init_var_pool_addr.
    value =
        (column3_row26 - global_values.initial_bitwise_addr).field_div(&felt_nonzero!(domain15));
    total_sum = total_sum + constraint_coefficients[82] * value;

    // Constraint: bitwise/step_var_pool_addr.
    value = (column3_row58 - (column3_row26 + FELT_1)) * domain7.field_div(&felt_nonzero!(domain5));
    total_sum = total_sum + constraint_coefficients[83] * value;

    // Constraint: bitwise/x_or_y_addr.
    value = (column3_row42 - (column3_row122 + FELT_1)).field_div(&felt_nonzero!(domain6));
    total_sum = total_sum + constraint_coefficients[84] * value;

    // Constraint: bitwise/next_var_pool_addr.
    value =
        (column3_row154 - (column3_row42 + FELT_1)) * domain20.field_div(&felt_nonzero!(domain6));
    total_sum = total_sum + constraint_coefficients[85] * value;

    // Constraint: bitwise/partition.
    value = (bitwise_sum_var_0_0 + bitwise_sum_var_8_0 - column3_row27)
        .field_div(&felt_nonzero!(domain5));
    total_sum = total_sum + constraint_coefficients[86] * value;

    // Constraint: bitwise/or_is_and_plus_xor.
    value = (column3_row43 - (column3_row91 + column3_row123)).field_div(&felt_nonzero!(domain6));
    total_sum = total_sum + constraint_coefficients[87] * value;

    // Constraint: bitwise/addition_is_xor_with_and.
    value = (column1_row0 + column1_row32 - (column1_row96 + column1_row64 + column1_row64))
        .field_div(&felt_nonzero!(domain8));
    total_sum = total_sum + constraint_coefficients[88] * value;

    // Constraint: bitwise/unique_unpacking192.
    value = ((column1_row88 + column1_row120) * FELT_16 - column1_row1)
        .field_div(&felt_nonzero!(domain6));
    total_sum = total_sum + constraint_coefficients[89] * value;

    // Constraint: bitwise/unique_unpacking193.
    value = ((column1_row90 + column1_row122) * FELT_16 - column1_row65)
        .field_div(&felt_nonzero!(domain6));
    total_sum = total_sum + constraint_coefficients[90] * value;

    // Constraint: bitwise/unique_unpacking194.
    value = ((column1_row92 + column1_row124) * FELT_16 - column1_row33)
        .field_div(&felt_nonzero!(domain6));
    total_sum = total_sum + constraint_coefficients[91] * value;

    // Constraint: bitwise/unique_unpacking195.
    value = ((column1_row94 + column1_row126) * FELT_256 - column1_row97)
        .field_div(&felt_nonzero!(domain6));

    total_sum + constraint_coefficients[92] * value
}
