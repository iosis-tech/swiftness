use super::global_values::GlobalValues;
use crate::layout::{LayoutTrait, StaticLayoutTrait};
use starknet_core::types::NonZeroFelt;
use starknet_crypto::Felt;

pub fn eval_composition_polynomial_inner(
    mask_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    trace_generator: &Felt,
    global_values: &GlobalValues,
) -> Felt {
    // Compute powers.
    let pow0 = point.pow_felt(
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(2048))),
    );
    let pow1 = pow0 * pow0; // pow(point, (safe_div(global_values.trace_length, 1024))).
    let pow2 = point.pow_felt(
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(128))),
    );
    let pow3 = point.pow_felt(
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(32))),
    );
    let pow4 = pow3 * pow3; // pow(point, (safe_div(global_values.trace_length, 16))).
    let pow5 = point.pow_felt(
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(4))),
    );
    let pow6 = pow5 * pow5; // pow(point, (safe_div(global_values.trace_length, 2))).
    let pow7 = pow6 * pow6; // pow(point, global_values.trace_length).
    let pow8 = trace_generator.pow_felt(&(global_values.trace_length - Felt::from(128)));
    let pow9 = trace_generator.pow_felt(&(global_values.trace_length - Felt::from(2048)));
    let pow10 = trace_generator.pow_felt(&(global_values.trace_length - Felt::from(1)));
    let pow11 = trace_generator.pow_felt(&(global_values.trace_length - Felt::from(4)));
    let pow12 = trace_generator.pow_felt(&(global_values.trace_length - Felt::from(2)));
    let pow13 = trace_generator.pow_felt(&(global_values.trace_length - Felt::from(16)));
    let pow14 = trace_generator.pow_felt(
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(2))),
    );
    let pow15 = trace_generator.pow_felt(
        &(Felt::from(255)
            * global_values
                .trace_length
                .floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(256)))),
    );
    let pow16 = trace_generator.pow_felt(
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(64))),
    );
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
    let pow31 = trace_generator.pow_felt(
        &(Felt::THREE
            * global_values
                .trace_length
                .floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(4)))),
    );
    let pow32 = pow27 * pow31; // pow(trace_generator, (safe_div((safe_mult(15, global_values.trace_length)), 16))).
    let pow33 = pow18 * pow32; // pow(trace_generator, (safe_div((safe_mult(63, global_values.trace_length)), 64))).

    // Compute domains.
    let domain0 = pow7 - 1;
    let domain1 = pow6 - 1;
    let domain2 = pow5 - 1;
    let domain3 = pow4 - pow32;
    let domain4 = pow4 - 1;
    let domain5 = pow3 - 1;
    let domain6 = pow2 - 1;
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
    let domain9 = pow1 - 1;
    let domain10 = pow1 - pow15;
    let domain11 = pow1 - pow33;
    let domain12 = pow0 - pow14;
    let domain13 = pow0 - 1;
    let domain14 = point - pow13;
    let domain15 = point - 1;
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
    let cpu_decode_flag_op1_base_op0_0 = Felt::ONE
        - (cpu_decode_opcode_range_check_bit_2
            + cpu_decode_opcode_range_check_bit_4
            + cpu_decode_opcode_range_check_bit_3);
    let cpu_decode_opcode_range_check_bit_5 = column0_row5 - (column0_row6 + column0_row6);
    let cpu_decode_opcode_range_check_bit_6 = column0_row6 - (column0_row7 + column0_row7);
    let cpu_decode_opcode_range_check_bit_9 = column0_row9 - (column0_row10 + column0_row10);
    let cpu_decode_flag_res_op1_0 = Felt::ONE
        - (cpu_decode_opcode_range_check_bit_5
            + cpu_decode_opcode_range_check_bit_6
            + cpu_decode_opcode_range_check_bit_9);
    let cpu_decode_opcode_range_check_bit_7 = column0_row7 - (column0_row8 + column0_row8);
    let cpu_decode_opcode_range_check_bit_8 = column0_row8 - (column0_row9 + column0_row9);
    let cpu_decode_flag_pc_update_regular_0 = Felt::ONE
        - (cpu_decode_opcode_range_check_bit_7
            + cpu_decode_opcode_range_check_bit_8
            + cpu_decode_opcode_range_check_bit_9);
    let cpu_decode_opcode_range_check_bit_12 = column0_row12 - (column0_row13 + column0_row13);
    let cpu_decode_opcode_range_check_bit_13 = column0_row13 - (column0_row14 + column0_row14);
    let cpu_decode_fp_update_regular_0 =
        Felt::ONE - (cpu_decode_opcode_range_check_bit_12 + cpu_decode_opcode_range_check_bit_13);
    let cpu_decode_opcode_range_check_bit_1 = column0_row1 - (column0_row2 + column0_row2);
    let npc_reg_0 = column3_row0 + cpu_decode_opcode_range_check_bit_2 + Felt::ONE;
    let cpu_decode_opcode_range_check_bit_10 = column0_row10 - (column0_row11 + column0_row11);
    let cpu_decode_opcode_range_check_bit_11 = column0_row11 - (column0_row12 + column0_row12);
    let cpu_decode_opcode_range_check_bit_14 = column0_row14 - (column0_row15 + column0_row15);
    let memory_address_diff_0 = column4_row2 - column4_row0;
    let range_check16_diff_0 = column5_row6 - column5_row2;
    let pedersen_hash0_ec_subset_sum_bit_0 = column6_row0 - (column6_row4 + column6_row4);
    let pedersen_hash0_ec_subset_sum_bit_neg_0 = Felt::ONE - pedersen_hash0_ec_subset_sum_bit_0;
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
        + column1_row2 * Felt::from_hex_unchecked("0x2")
        + column1_row4 * Felt::from_hex_unchecked("0x4")
        + column1_row6 * Felt::from_hex_unchecked("0x8")
        + column1_row8 * Felt::from_hex_unchecked("0x10000000000000000")
        + column1_row10 * Felt::from_hex_unchecked("0x20000000000000000")
        + column1_row12 * Felt::from_hex_unchecked("0x40000000000000000")
        + column1_row14 * Felt::from_hex_unchecked("0x80000000000000000");
    let bitwise_sum_var_8_0 = column1_row16
        * Felt::from_hex_unchecked("0x100000000000000000000000000000000")
        + column1_row18 * Felt::from_hex_unchecked("0x200000000000000000000000000000000")
        + column1_row20 * Felt::from_hex_unchecked("0x400000000000000000000000000000000")
        + column1_row22 * Felt::from_hex_unchecked("0x800000000000000000000000000000000")
        + column1_row24
            * Felt::from_hex_unchecked("0x1000000000000000000000000000000000000000000000000")
        + column1_row26
            * Felt::from_hex_unchecked("0x2000000000000000000000000000000000000000000000000")
        + column1_row28
            * Felt::from_hex_unchecked("0x4000000000000000000000000000000000000000000000000")
        + column1_row30
            * Felt::from_hex_unchecked("0x8000000000000000000000000000000000000000000000000");

    // Sum constraints.
    let mut total_sum = Felt::ZERO;

    // Constraint: cpu/decode/opcode_range_check/bit.
    let mut value = (cpu_decode_opcode_range_check_bit_0 * cpu_decode_opcode_range_check_bit_0
        - cpu_decode_opcode_range_check_bit_0)
        * domain3.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[0] * value;

    // Constraint: cpu/decode/opcode_range_check/zero.
    value = (column0_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[1] * value;

    // Constraint: cpu/decode/opcode_range_check_input.
    value = (column3_row1
        - (((column0_row0 * global_values.offset_size + column5_row4)
            * global_values.offset_size
            + column5_row8)
            * global_values.offset_size
            + column5_row0))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[2] * value;

    // Constraint: cpu/decode/flag_op1_base_op0_bit.
    value = (cpu_decode_flag_op1_base_op0_0 * cpu_decode_flag_op1_base_op0_0
        - cpu_decode_flag_op1_base_op0_0)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[3] * value;

    // Constraint: cpu/decode/flag_res_op1_bit.
    value = (cpu_decode_flag_res_op1_0 * cpu_decode_flag_res_op1_0 - cpu_decode_flag_res_op1_0)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[4] * value;

    // Constraint: cpu/decode/flag_pc_update_regular_bit.
    value = (cpu_decode_flag_pc_update_regular_0 * cpu_decode_flag_pc_update_regular_0
        - cpu_decode_flag_pc_update_regular_0)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[5] * value;

    // Constraint: cpu/decode/fp_update_regular_bit.
    value = (cpu_decode_fp_update_regular_0 * cpu_decode_fp_update_regular_0
        - cpu_decode_fp_update_regular_0)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[6] * value;

    // Constraint: cpu/operands/mem_dst_addr.
    value = (column3_row8 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_0 * column6_row9
            + (Felt::ONE - cpu_decode_opcode_range_check_bit_0) * column6_row1
            + column5_row0))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[7] * value;

    // Constraint: cpu/operands/mem0_addr.
    value = (column3_row4 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_1 * column6_row9
            + (Felt::ONE - cpu_decode_opcode_range_check_bit_1) * column6_row1
            + column5_row8))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[8] * value;

    // Constraint: cpu/operands/mem1_addr.
    value = (column3_row12 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_2 * column3_row0
            + cpu_decode_opcode_range_check_bit_4 * column6_row1
            + cpu_decode_opcode_range_check_bit_3 * column6_row9
            + cpu_decode_flag_op1_base_op0_0 * column3_row5
            + column5_row4))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[9] * value;

    // Constraint: cpu/operands/ops_mul.
    value = (column6_row5 - column3_row5 * column3_row13)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[10] * value;

    // Constraint: cpu/operands/res.
    value = ((Felt::ONE - cpu_decode_opcode_range_check_bit_9) * column6_row13
        - (cpu_decode_opcode_range_check_bit_5 * (column3_row5 + column3_row13)
            + cpu_decode_opcode_range_check_bit_6 * column6_row5
            + cpu_decode_flag_res_op1_0 * column3_row13))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[11] * value;

    // Constraint: cpu/update_registers/update_pc/tmp0.
    value = (column6_row3 - cpu_decode_opcode_range_check_bit_9 * column3_row9)
        * domain14.field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[12] * value;

    // Constraint: cpu/update_registers/update_pc/tmp1.
    value = (column6_row11 - column6_row3 * column6_row13)
        * domain14.field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[13] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_negative.
    value = ((Felt::ONE - cpu_decode_opcode_range_check_bit_9) * column3_row16
        + column6_row3 * (column3_row16 - (column3_row0 + column3_row13))
        - (cpu_decode_flag_pc_update_regular_0 * npc_reg_0
            + cpu_decode_opcode_range_check_bit_7 * column6_row13
            + cpu_decode_opcode_range_check_bit_8 * (column3_row0 + column6_row13)))
        * domain14.field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[14] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_positive.
    value = ((column6_row11 - cpu_decode_opcode_range_check_bit_9) * (column3_row16 - npc_reg_0))
        * domain14.field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[15] * value;

    // Constraint: cpu/update_registers/update_ap/ap_update.
    value = (column6_row17
        - (column6_row1
            + cpu_decode_opcode_range_check_bit_10 * column6_row13
            + cpu_decode_opcode_range_check_bit_11
            + cpu_decode_opcode_range_check_bit_12 * Felt::TWO))
        * domain14.field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[16] * value;

    // Constraint: cpu/update_registers/update_fp/fp_update.
    value = (column6_row25
        - (cpu_decode_fp_update_regular_0 * column6_row9
            + cpu_decode_opcode_range_check_bit_13 * column3_row9
            + cpu_decode_opcode_range_check_bit_12 * (column6_row1 + Felt::TWO)))
        * domain14.field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[17] * value;

    // Constraint: cpu/opcodes/call/push_fp.
    value = (cpu_decode_opcode_range_check_bit_12 * (column3_row9 - column6_row9))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[18] * value;

    // Constraint: cpu/opcodes/call/push_pc.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column3_row5 - (column3_row0 + cpu_decode_opcode_range_check_bit_2 + 1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[19] * value;

    // Constraint: cpu/opcodes/call/off0.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column5_row0 - global_values.half_offset_size))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[20] * value;

    // Constraint: cpu/opcodes/call/off1.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column5_row8 - (global_values.half_offset_size + 1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[21] * value;

    // Constraint: cpu/opcodes/call/flags.
    value = (cpu_decode_opcode_range_check_bit_12
        * (cpu_decode_opcode_range_check_bit_12 + cpu_decode_opcode_range_check_bit_12 + 2
            - (cpu_decode_opcode_range_check_bit_0 + cpu_decode_opcode_range_check_bit_1 + 4)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[22] * value;

    // Constraint: cpu/opcodes/ret/off0.
    value = (cpu_decode_opcode_range_check_bit_13
        * (column5_row0 + 2 - global_values.half_offset_size))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[23] * value;

    // Constraint: cpu/opcodes/ret/off2.
    value = (cpu_decode_opcode_range_check_bit_13
        * (column5_row4 + 1 - global_values.half_offset_size))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[24] * value;

    // Constraint: cpu/opcodes/ret/flags.
    value = (cpu_decode_opcode_range_check_bit_13
        * (cpu_decode_opcode_range_check_bit_7
            + cpu_decode_opcode_range_check_bit_0
            + cpu_decode_opcode_range_check_bit_3
            + cpu_decode_flag_res_op1_0
            - 4))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[25] * value;

    // Constraint: cpu/opcodes/assert_eq/assert_eq.
    value = (cpu_decode_opcode_range_check_bit_14 * (column3_row9 - column6_row13))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[26] * value;

    // Constraint: initial_ap.
    value = (column6_row1 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[27] * value;

    // Constraint: initial_fp.
    value = (column6_row9 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[28] * value;

    // Constraint: initial_pc.
    value = (column3_row0 - global_values.initial_pc)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[29] * value;

    // Constraint: final_ap.
    value = (column6_row1 - global_values.final_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain14));
    total_sum += constraint_coefficients[30] * value;

    // Constraint: final_fp.
    value = (column6_row9 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain14));
    total_sum += constraint_coefficients[31] * value;

    // Constraint: final_pc.
    value = (column3_row0 - global_values.final_pc)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain14));
    total_sum += constraint_coefficients[32] * value;

    // Constraint: memory/multi_column_perm/perm/init0.
    value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column4_row0
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column4_row1))
        * column9_inter1_row0
        + column3_row0
        + global_values.memory_multi_column_perm_hash_interaction_elm0 * column3_row1
        - global_values.memory_multi_column_perm_perm_interaction_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[33] * value;

    // Constraint: memory/multi_column_perm/perm/step0.
    value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column4_row2
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column4_row3))
        * column9_inter1_row2
        - (global_values.memory_multi_column_perm_perm_interaction_elm
            - (column3_row2
                + global_values.memory_multi_column_perm_hash_interaction_elm0 * column3_row3))
            * column9_inter1_row0)
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[34] * value;

    // Constraint: memory/multi_column_perm/perm/last.
    value = (column9_inter1_row0 - global_values.memory_multi_column_perm_perm_public_memory_prod)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[35] * value;

    // Constraint: memory/diff_is_bit.
    value = (memory_address_diff_0 * memory_address_diff_0 - memory_address_diff_0)
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[36] * value;

    // Constraint: memory/is_func.
    value = ((memory_address_diff_0 - 1) * (column4_row1 - column4_row3))
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[37] * value;

    // Constraint: memory/initial_addr.
    value = (column4_row0 - 1).field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[38] * value;

    // Constraint: public_memory_addr_zero.
    value = (column3_row2).field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[39] * value;

    // Constraint: public_memory_value_zero.
    value = (column3_row3).field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[40] * value;

    // Constraint: range_check16/perm/init0.
    value = ((global_values.range_check16_perm_interaction_elm - column5_row2)
        * column9_inter1_row1
        + column5_row0
        - global_values.range_check16_perm_interaction_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[41] * value;

    // Constraint: range_check16/perm/step0.
    value = ((global_values.range_check16_perm_interaction_elm - column5_row6)
        * column9_inter1_row5
        - (global_values.range_check16_perm_interaction_elm - column5_row4) * column9_inter1_row1)
        * domain17.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[42] * value;

    // Constraint: range_check16/perm/last.
    value = (column9_inter1_row1 - global_values.range_check16_perm_public_memory_prod)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain17));
    total_sum += constraint_coefficients[43] * value;

    // Constraint: range_check16/diff_is_bit.
    value = (range_check16_diff_0 * range_check16_diff_0 - range_check16_diff_0)
        * domain17.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[44] * value;

    // Constraint: range_check16/minimum.
    value = (column5_row2 - global_values.range_check_min)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[45] * value;

    // Constraint: range_check16/maximum.
    value = (column5_row2 - global_values.range_check_max)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain17));
    total_sum += constraint_coefficients[46] * value;

    // Constraint: diluted_check/permutation/init0.
    value = ((global_values.diluted_check_permutation_interaction_elm - column2_row0)
        * column8_inter1_row0
        + column1_row0
        - global_values.diluted_check_permutation_interaction_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[47] * value;

    // Constraint: diluted_check/permutation/step0.
    value = ((global_values.diluted_check_permutation_interaction_elm - column2_row1)
        * column8_inter1_row1
        - (global_values.diluted_check_permutation_interaction_elm - column1_row1)
            * column8_inter1_row0)
        * domain18.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[48] * value;

    // Constraint: diluted_check/permutation/last.
    value = (column8_inter1_row0 - global_values.diluted_check_permutation_public_memory_prod)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[49] * value;

    // Constraint: diluted_check/init.
    value = (column7_inter1_row0 - 1).field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[50] * value;

    // Constraint: diluted_check/first_element.
    value = (column2_row0 - global_values.diluted_check_first_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[51] * value;

    // Constraint: diluted_check/step.
    value = (column7_inter1_row1
        - (column7_inter1_row0
            * (Felt::ONE
                + global_values.diluted_check_interaction_z * (column2_row1 - column2_row0))
            + global_values.diluted_check_interaction_alpha
                * (column2_row1 - column2_row0)
                * (column2_row1 - column2_row0)))
        * domain18.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[52] * value;

    // Constraint: diluted_check/last.
    value = (column7_inter1_row0 - global_values.diluted_check_final_cum_val)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[53] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/last_one_is_zero.
    value = (column6_row7 * (column6_row0 - (column6_row4 + column6_row4)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[54] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    value = (column6_row7
        * (column6_row4
            - Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000")
                * column6_row768))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[55] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit192.
    value = (column6_row7 - column6_row1022 * (column6_row768 - (column6_row772 + column6_row772)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[56] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    value = (column6_row1022 * (column6_row772 - Felt::from_hex_unchecked("0x8") * column6_row784))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[57] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit196.
    value = (column6_row1022
        - (column6_row1004 - (column6_row1008 + column6_row1008))
            * (column6_row784 - (column6_row788 + column6_row788)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[58] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    value = ((column6_row1004 - (column6_row1008 + column6_row1008))
        * (column6_row788 - Felt::from_hex_unchecked("0x40000000000000") * column6_row1004))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[59] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/booleanity_test.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (pedersen_hash0_ec_subset_sum_bit_0 - 1))
        * domain10.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[60] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_extraction_end.
    value = (column6_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain11));
    total_sum += constraint_coefficients[61] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/zeros_tail.
    value = (column6_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain10));
    total_sum += constraint_coefficients[62] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/slope.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (column5_row3 - global_values.pedersen_points_y)
        - column6_row2 * (column5_row1 - global_values.pedersen_points_x))
        * domain10.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[63] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/x.
    value = (column6_row2 * column6_row2
        - pedersen_hash0_ec_subset_sum_bit_0
            * (column5_row1 + global_values.pedersen_points_x + column5_row5))
        * domain10.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[64] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/y.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (column5_row3 + column5_row7)
        - column6_row2 * (column5_row1 - column5_row5))
        * domain10.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[65] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/x.
    value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column5_row5 - column5_row1))
        * domain10.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[66] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/y.
    value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column5_row7 - column5_row3))
        * domain10.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[67] * value;

    // Constraint: pedersen/hash0/copy_point/x.
    value = (column5_row1025 - column5_row1021)
        * domain12.field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[68] * value;

    // Constraint: pedersen/hash0/copy_point/y.
    value = (column5_row1027 - column5_row1023)
        * domain12.field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[69] * value;

    // Constraint: pedersen/hash0/init/x.
    value = (column5_row1 - global_values.pedersen_shift_point.x)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[70] * value;

    // Constraint: pedersen/hash0/init/y.
    value = (column5_row3 - global_values.pedersen_shift_point.y)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[71] * value;

    // Constraint: pedersen/input0_value0.
    value = (column3_row11 - column6_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[72] * value;

    // Constraint: pedersen/input0_addr.
    value = (column3_row2058 - (column3_row522 + Felt::from_hex_unchecked("0x1")))
        * domain19.field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[73] * value;

    // Constraint: pedersen/init_addr.
    value = (column3_row10 - global_values.initial_pedersen_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[74] * value;

    // Constraint: pedersen/input1_value0.
    value =
        (column3_row1035 - column6_row1024).field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[75] * value;

    // Constraint: pedersen/input1_addr.
    value = (column3_row1034 - (column3_row10 + Felt::from_hex_unchecked("0x1")))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[76] * value;

    // Constraint: pedersen/output_value0.
    value =
        (column3_row523 - column5_row2045).field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[77] * value;

    // Constraint: pedersen/output_addr.
    value = (column3_row522 - (column3_row1034 + Felt::from_hex_unchecked("0x1")))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[78] * value;

    // Constraint: range_check_builtin/value.
    value = (range_check_builtin_value7_0 - column3_row75)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[79] * value;

    // Constraint: range_check_builtin/addr_step.
    value = (column3_row202 - (column3_row74 + Felt::from_hex_unchecked("0x1")))
        * domain20.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[80] * value;

    // Constraint: range_check_builtin/init_addr.
    value = (column3_row74 - global_values.initial_range_check_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[81] * value;

    // Constraint: bitwise/init_var_pool_addr.
    value = (column3_row26 - global_values.initial_bitwise_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[82] * value;

    // Constraint: bitwise/step_var_pool_addr.
    value = (column3_row58 - (column3_row26 + Felt::from_hex_unchecked("0x1")))
        * domain7.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[83] * value;

    // Constraint: bitwise/x_or_y_addr.
    value = (column3_row42 - (column3_row122 + Felt::from_hex_unchecked("0x1")))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[84] * value;

    // Constraint: bitwise/next_var_pool_addr.
    value = (column3_row154 - (column3_row42 + Felt::from_hex_unchecked("0x1")))
        * domain20.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[85] * value;

    // Constraint: bitwise/partition.
    value = (bitwise_sum_var_0_0 + bitwise_sum_var_8_0 - column3_row27)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[86] * value;

    // Constraint: bitwise/or_is_and_plus_xor.
    value = (column3_row43 - (column3_row91 + column3_row123))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[87] * value;

    // Constraint: bitwise/addition_is_xor_with_and.
    value = (column1_row0 + column1_row32 - (column1_row96 + column1_row64 + column1_row64))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[88] * value;

    // Constraint: bitwise/unique_unpacking192.
    value = ((column1_row88 + column1_row120) * Felt::from_hex_unchecked("0x10") - column1_row1)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[89] * value;

    // Constraint: bitwise/unique_unpacking193.
    value = ((column1_row90 + column1_row122) * Felt::from_hex_unchecked("0x10") - column1_row65)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[90] * value;

    // Constraint: bitwise/unique_unpacking194.
    value = ((column1_row92 + column1_row124) * Felt::from_hex_unchecked("0x10") - column1_row33)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[91] * value;

    // Constraint: bitwise/unique_unpacking195.
    value = ((column1_row94 + column1_row126) * Felt::from_hex_unchecked("0x100") - column1_row97)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[92] * value;

    total_sum
}

pub fn eval_oods_polynomial_inner<Layout: StaticLayoutTrait + LayoutTrait>(
    column_values: &[Felt],
    oods_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    oods_point: &Felt,
    trace_generator: &Felt,
) -> Felt {
    // Compute powers.
    let pow0 = trace_generator.pow(0_u128);
    let pow1 = trace_generator.pow(1004_u128);
    let pow2 = trace_generator.pow(768_u128);
    let pow3 = trace_generator.pow(522_u128);
    let pow4 = trace_generator.pow(1_u128);
    let pow5 = pow3 * pow4; // pow(trace_generator, 523).
    let pow6 = pow4 * pow4; // pow(trace_generator, 2).
    let pow7 = pow4 * pow6; // pow(trace_generator, 3).
    let pow8 = pow4 * pow7; // pow(trace_generator, 4).
    let pow9 = pow1 * pow8; // pow(trace_generator, 1008).
    let pow10 = pow2 * pow8; // pow(trace_generator, 772).
    let pow11 = pow4 * pow8; // pow(trace_generator, 5).
    let pow12 = pow4 * pow11; // pow(trace_generator, 6).
    let pow13 = pow4 * pow12; // pow(trace_generator, 7).
    let pow14 = pow4 * pow13; // pow(trace_generator, 8).
    let pow15 = pow4 * pow14; // pow(trace_generator, 9).
    let pow16 = pow4 * pow15; // pow(trace_generator, 10).
    let pow17 = pow4 * pow16; // pow(trace_generator, 11).
    let pow18 = pow4 * pow17; // pow(trace_generator, 12).
    let pow19 = pow4 * pow18; // pow(trace_generator, 13).
    let pow20 = pow4 * pow19; // pow(trace_generator, 14).
    let pow21 = pow4 * pow20; // pow(trace_generator, 15).
    let pow22 = pow4 * pow21; // pow(trace_generator, 16).
    let pow23 = pow2 * pow22; // pow(trace_generator, 784).
    let pow24 = pow4 * pow22; // pow(trace_generator, 17).
    let pow25 = pow1 * pow24; // pow(trace_generator, 1021).
    let pow26 = pow4 * pow24; // pow(trace_generator, 18).
    let pow27 = pow1 * pow26; // pow(trace_generator, 1022).
    let pow28 = pow4 * pow27; // pow(trace_generator, 1023).
    let pow29 = pow6 * pow26; // pow(trace_generator, 20).
    let pow30 = pow6 * pow29; // pow(trace_generator, 22).
    let pow31 = pow6 * pow30; // pow(trace_generator, 24).
    let pow32 = pow4 * pow31; // pow(trace_generator, 25).
    let pow33 = pow4 * pow32; // pow(trace_generator, 26).
    let pow34 = pow1 * pow29; // pow(trace_generator, 1024).
    let pow35 = pow25 * pow34; // pow(trace_generator, 2045).
    let pow36 = pow4 * pow34; // pow(trace_generator, 1025).
    let pow37 = pow6 * pow36; // pow(trace_generator, 1027).
    let pow38 = pow4 * pow33; // pow(trace_generator, 27).
    let pow39 = pow4 * pow38; // pow(trace_generator, 28).
    let pow40 = pow6 * pow39; // pow(trace_generator, 30).
    let pow41 = pow6 * pow40; // pow(trace_generator, 32).
    let pow42 = pow4 * pow41; // pow(trace_generator, 33).
    let pow43 = pow1 * pow40; // pow(trace_generator, 1034).
    let pow44 = pow4 * pow43; // pow(trace_generator, 1035).
    let pow45 = pow19 * pow35; // pow(trace_generator, 2058).
    let pow46 = pow15 * pow42; // pow(trace_generator, 42).
    let pow47 = pow4 * pow46; // pow(trace_generator, 43).
    let pow48 = pow4 * pow47; // pow(trace_generator, 44).
    let pow49 = pow20 * pow48; // pow(trace_generator, 58).
    let pow50 = pow6 * pow49; // pow(trace_generator, 60).
    let pow51 = pow2 * pow29; // pow(trace_generator, 788).
    let pow52 = pow8 * pow50; // pow(trace_generator, 64).
    let pow53 = pow4 * pow52; // pow(trace_generator, 65).
    let pow54 = pow15 * pow53; // pow(trace_generator, 74).
    let pow55 = pow4 * pow54; // pow(trace_generator, 75).
    let pow56 = pow4 * pow55; // pow(trace_generator, 76).
    let pow57 = pow18 * pow56; // pow(trace_generator, 88).
    let pow58 = pow6 * pow57; // pow(trace_generator, 90).
    let pow59 = pow4 * pow58; // pow(trace_generator, 91).
    let pow60 = pow4 * pow59; // pow(trace_generator, 92).
    let pow61 = pow6 * pow60; // pow(trace_generator, 94).
    let pow62 = pow6 * pow61; // pow(trace_generator, 96).
    let pow63 = pow4 * pow62; // pow(trace_generator, 97).
    let pow64 = pow17 * pow63; // pow(trace_generator, 108).
    let pow65 = pow18 * pow64; // pow(trace_generator, 120).
    let pow66 = pow6 * pow65; // pow(trace_generator, 122).
    let pow67 = pow4 * pow66; // pow(trace_generator, 123).
    let pow68 = pow4 * pow67; // pow(trace_generator, 124).
    let pow69 = pow6 * pow68; // pow(trace_generator, 126).
    let pow70 = pow56 * pow69; // pow(trace_generator, 202).
    let pow71 = pow39 * pow69; // pow(trace_generator, 154).

    // Fetch columns.
    let column0 = column_values[0];
    let column1 = column_values[1];
    let column2 = column_values[2];
    let column3 = column_values[3];
    let column4 = column_values[4];
    let column5 = column_values[5];
    let column6 = column_values[6];
    let column7 = column_values[7];
    let column8 = column_values[8];
    let column9 = column_values[9];

    // Sum the OODS constraints on the trace polynomials.
    let mut value: Felt;
    let mut total_sum = Felt::ZERO;

    value = (column0 - oods_values[0])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[0] * value;

    value = (column0 - oods_values[1])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[1] * value;

    value = (column0 - oods_values[2])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[2] * value;

    value = (column0 - oods_values[3])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[3] * value;

    value = (column0 - oods_values[4])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[4] * value;

    value = (column0 - oods_values[5])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[5] * value;

    value = (column0 - oods_values[6])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow12 * oods_point));
    total_sum += constraint_coefficients[6] * value;

    value = (column0 - oods_values[7])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow13 * oods_point));
    total_sum += constraint_coefficients[7] * value;

    value = (column0 - oods_values[8])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[8] * value;

    value = (column0 - oods_values[9])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow15 * oods_point));
    total_sum += constraint_coefficients[9] * value;

    value = (column0 - oods_values[10])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[10] * value;

    value = (column0 - oods_values[11])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow17 * oods_point));
    total_sum += constraint_coefficients[11] * value;

    value = (column0 - oods_values[12])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[12] * value;

    value = (column0 - oods_values[13])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow19 * oods_point));
    total_sum += constraint_coefficients[13] * value;

    value = (column0 - oods_values[14])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow20 * oods_point));
    total_sum += constraint_coefficients[14] * value;

    value = (column0 - oods_values[15])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow21 * oods_point));
    total_sum += constraint_coefficients[15] * value;

    value = (column1 - oods_values[16])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[16] * value;

    value = (column1 - oods_values[17])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[17] * value;

    value = (column1 - oods_values[18])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[18] * value;

    value = (column1 - oods_values[19])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[19] * value;

    value = (column1 - oods_values[20])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow12 * oods_point));
    total_sum += constraint_coefficients[20] * value;

    value = (column1 - oods_values[21])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[21] * value;

    value = (column1 - oods_values[22])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[22] * value;

    value = (column1 - oods_values[23])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[23] * value;

    value = (column1 - oods_values[24])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow20 * oods_point));
    total_sum += constraint_coefficients[24] * value;

    value = (column1 - oods_values[25])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow22 * oods_point));
    total_sum += constraint_coefficients[25] * value;

    value = (column1 - oods_values[26])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow26 * oods_point));
    total_sum += constraint_coefficients[26] * value;

    value = (column1 - oods_values[27])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow29 * oods_point));
    total_sum += constraint_coefficients[27] * value;

    value = (column1 - oods_values[28])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow30 * oods_point));
    total_sum += constraint_coefficients[28] * value;

    value = (column1 - oods_values[29])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow31 * oods_point));
    total_sum += constraint_coefficients[29] * value;

    value = (column1 - oods_values[30])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow33 * oods_point));
    total_sum += constraint_coefficients[30] * value;

    value = (column1 - oods_values[31])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow39 * oods_point));
    total_sum += constraint_coefficients[31] * value;

    value = (column1 - oods_values[32])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow40 * oods_point));
    total_sum += constraint_coefficients[32] * value;

    value = (column1 - oods_values[33])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow41 * oods_point));
    total_sum += constraint_coefficients[33] * value;

    value = (column1 - oods_values[34])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow42 * oods_point));
    total_sum += constraint_coefficients[34] * value;

    value = (column1 - oods_values[35])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow52 * oods_point));
    total_sum += constraint_coefficients[35] * value;

    value = (column1 - oods_values[36])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow53 * oods_point));
    total_sum += constraint_coefficients[36] * value;

    value = (column1 - oods_values[37])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow57 * oods_point));
    total_sum += constraint_coefficients[37] * value;

    value = (column1 - oods_values[38])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow58 * oods_point));
    total_sum += constraint_coefficients[38] * value;

    value = (column1 - oods_values[39])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow60 * oods_point));
    total_sum += constraint_coefficients[39] * value;

    value = (column1 - oods_values[40])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow61 * oods_point));
    total_sum += constraint_coefficients[40] * value;

    value = (column1 - oods_values[41])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow62 * oods_point));
    total_sum += constraint_coefficients[41] * value;

    value = (column1 - oods_values[42])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow63 * oods_point));
    total_sum += constraint_coefficients[42] * value;

    value = (column1 - oods_values[43])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow65 * oods_point));
    total_sum += constraint_coefficients[43] * value;

    value = (column1 - oods_values[44])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow66 * oods_point));
    total_sum += constraint_coefficients[44] * value;

    value = (column1 - oods_values[45])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow68 * oods_point));
    total_sum += constraint_coefficients[45] * value;

    value = (column1 - oods_values[46])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow69 * oods_point));
    total_sum += constraint_coefficients[46] * value;

    value = (column2 - oods_values[47])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[47] * value;

    value = (column2 - oods_values[48])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[48] * value;

    value = (column3 - oods_values[49])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[49] * value;

    value = (column3 - oods_values[50])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[50] * value;

    value = (column3 - oods_values[51])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[51] * value;

    value = (column3 - oods_values[52])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[52] * value;

    value = (column3 - oods_values[53])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[53] * value;

    value = (column3 - oods_values[54])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[54] * value;

    value = (column3 - oods_values[55])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[55] * value;

    value = (column3 - oods_values[56])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow15 * oods_point));
    total_sum += constraint_coefficients[56] * value;

    value = (column3 - oods_values[57])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[57] * value;

    value = (column3 - oods_values[58])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow17 * oods_point));
    total_sum += constraint_coefficients[58] * value;

    value = (column3 - oods_values[59])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[59] * value;

    value = (column3 - oods_values[60])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow19 * oods_point));
    total_sum += constraint_coefficients[60] * value;

    value = (column3 - oods_values[61])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow22 * oods_point));
    total_sum += constraint_coefficients[61] * value;

    value = (column3 - oods_values[62])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow33 * oods_point));
    total_sum += constraint_coefficients[62] * value;

    value = (column3 - oods_values[63])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow38 * oods_point));
    total_sum += constraint_coefficients[63] * value;

    value = (column3 - oods_values[64])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow46 * oods_point));
    total_sum += constraint_coefficients[64] * value;

    value = (column3 - oods_values[65])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow47 * oods_point));
    total_sum += constraint_coefficients[65] * value;

    value = (column3 - oods_values[66])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow49 * oods_point));
    total_sum += constraint_coefficients[66] * value;

    value = (column3 - oods_values[67])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow54 * oods_point));
    total_sum += constraint_coefficients[67] * value;

    value = (column3 - oods_values[68])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow55 * oods_point));
    total_sum += constraint_coefficients[68] * value;

    value = (column3 - oods_values[69])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow59 * oods_point));
    total_sum += constraint_coefficients[69] * value;

    value = (column3 - oods_values[70])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow66 * oods_point));
    total_sum += constraint_coefficients[70] * value;

    value = (column3 - oods_values[71])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow67 * oods_point));
    total_sum += constraint_coefficients[71] * value;

    value = (column3 - oods_values[72])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow71 * oods_point));
    total_sum += constraint_coefficients[72] * value;

    value = (column3 - oods_values[73])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow70 * oods_point));
    total_sum += constraint_coefficients[73] * value;

    value = (column3 - oods_values[74])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[74] * value;

    value = (column3 - oods_values[75])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[75] * value;

    value = (column3 - oods_values[76])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow43 * oods_point));
    total_sum += constraint_coefficients[76] * value;

    value = (column3 - oods_values[77])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow44 * oods_point));
    total_sum += constraint_coefficients[77] * value;

    value = (column3 - oods_values[78])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow45 * oods_point));
    total_sum += constraint_coefficients[78] * value;

    value = (column4 - oods_values[79])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[79] * value;

    value = (column4 - oods_values[80])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[80] * value;

    value = (column4 - oods_values[81])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[81] * value;

    value = (column4 - oods_values[82])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[82] * value;

    value = (column5 - oods_values[83])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[83] * value;

    value = (column5 - oods_values[84])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[84] * value;

    value = (column5 - oods_values[85])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[85] * value;

    value = (column5 - oods_values[86])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[86] * value;

    value = (column5 - oods_values[87])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[87] * value;

    value = (column5 - oods_values[88])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[88] * value;

    value = (column5 - oods_values[89])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow12 * oods_point));
    total_sum += constraint_coefficients[89] * value;

    value = (column5 - oods_values[90])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow13 * oods_point));
    total_sum += constraint_coefficients[90] * value;

    value = (column5 - oods_values[91])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[91] * value;

    value = (column5 - oods_values[92])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[92] * value;

    value = (column5 - oods_values[93])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow39 * oods_point));
    total_sum += constraint_coefficients[93] * value;

    value = (column5 - oods_values[94])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow48 * oods_point));
    total_sum += constraint_coefficients[94] * value;

    value = (column5 - oods_values[95])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow50 * oods_point));
    total_sum += constraint_coefficients[95] * value;

    value = (column5 - oods_values[96])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow56 * oods_point));
    total_sum += constraint_coefficients[96] * value;

    value = (column5 - oods_values[97])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow60 * oods_point));
    total_sum += constraint_coefficients[97] * value;

    value = (column5 - oods_values[98])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow64 * oods_point));
    total_sum += constraint_coefficients[98] * value;

    value = (column5 - oods_values[99])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow68 * oods_point));
    total_sum += constraint_coefficients[99] * value;

    value = (column5 - oods_values[100])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow25 * oods_point));
    total_sum += constraint_coefficients[100] * value;

    value = (column5 - oods_values[101])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow28 * oods_point));
    total_sum += constraint_coefficients[101] * value;

    value = (column5 - oods_values[102])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow36 * oods_point));
    total_sum += constraint_coefficients[102] * value;

    value = (column5 - oods_values[103])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow37 * oods_point));
    total_sum += constraint_coefficients[103] * value;

    value = (column5 - oods_values[104])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow35 * oods_point));
    total_sum += constraint_coefficients[104] * value;

    value = (column6 - oods_values[105])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[105] * value;

    value = (column6 - oods_values[106])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[106] * value;

    value = (column6 - oods_values[107])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[107] * value;

    value = (column6 - oods_values[108])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[108] * value;

    value = (column6 - oods_values[109])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[109] * value;

    value = (column6 - oods_values[110])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[110] * value;

    value = (column6 - oods_values[111])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow13 * oods_point));
    total_sum += constraint_coefficients[111] * value;

    value = (column6 - oods_values[112])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow15 * oods_point));
    total_sum += constraint_coefficients[112] * value;

    value = (column6 - oods_values[113])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow17 * oods_point));
    total_sum += constraint_coefficients[113] * value;

    value = (column6 - oods_values[114])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow19 * oods_point));
    total_sum += constraint_coefficients[114] * value;

    value = (column6 - oods_values[115])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow24 * oods_point));
    total_sum += constraint_coefficients[115] * value;

    value = (column6 - oods_values[116])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow32 * oods_point));
    total_sum += constraint_coefficients[116] * value;

    value = (column6 - oods_values[117])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow2 * oods_point));
    total_sum += constraint_coefficients[117] * value;

    value = (column6 - oods_values[118])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow10 * oods_point));
    total_sum += constraint_coefficients[118] * value;

    value = (column6 - oods_values[119])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow23 * oods_point));
    total_sum += constraint_coefficients[119] * value;

    value = (column6 - oods_values[120])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow51 * oods_point));
    total_sum += constraint_coefficients[120] * value;

    value = (column6 - oods_values[121])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow1 * oods_point));
    total_sum += constraint_coefficients[121] * value;

    value = (column6 - oods_values[122])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow9 * oods_point));
    total_sum += constraint_coefficients[122] * value;

    value = (column6 - oods_values[123])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow27 * oods_point));
    total_sum += constraint_coefficients[123] * value;

    value = (column6 - oods_values[124])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow34 * oods_point));
    total_sum += constraint_coefficients[124] * value;

    value = (column7 - oods_values[125])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[125] * value;

    value = (column7 - oods_values[126])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[126] * value;

    value = (column8 - oods_values[127])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[127] * value;

    value = (column8 - oods_values[128])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[128] * value;

    value = (column9 - oods_values[129])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[129] * value;

    value = (column9 - oods_values[130])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[130] * value;

    value = (column9 - oods_values[131])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[131] * value;

    value = (column9 - oods_values[132])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[132] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow(Layout::CONSTRAINT_DEGREE as u128);

    value = (column_values[Layout::NUM_COLUMNS_FIRST + Layout::NUM_COLUMNS_SECOND]
        - oods_values[133])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - oods_point_to_deg));
    total_sum += constraint_coefficients[133] * value;

    value = (column_values[Layout::NUM_COLUMNS_FIRST + Layout::NUM_COLUMNS_SECOND + 1]
        - oods_values[134])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - oods_point_to_deg));
    total_sum += constraint_coefficients[134] * value;

    total_sum
}
