use crate::{consts::*, felt_nonzero, layout::small::GlobalValues};
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
    let pow0 = point.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_8192))));
    let pow1 = pow0 * pow0; // pow(point, (safe_div(global_values.trace_length, 4096))).
    let pow2 = point.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_512))));
    let pow3 = pow2 * pow2; // pow(point, (safe_div(global_values.trace_length, 256))).
    let pow4 = pow3 * pow3; // pow(point, (safe_div(global_values.trace_length, 128))).
    let pow5 = point.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_32))));
    let pow6 = pow5 * pow5; // pow(point, (safe_div(global_values.trace_length, 16))).
    let pow7 = pow6 * pow6; // pow(point, (safe_div(global_values.trace_length, 8))).
    let pow8 = point.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_2))));
    let pow9 = pow8 * pow8; // pow(point, global_values.trace_length).
    let pow10 = trace_generator.pow_felt(&(global_values.trace_length - FELT_8192));
    let pow11 = trace_generator.pow_felt(&(global_values.trace_length - FELT_128));
    let pow12 = trace_generator.pow_felt(&(global_values.trace_length - FELT_1));
    let pow13 = trace_generator.pow_felt(&(global_values.trace_length - FELT_2));
    let pow14 = trace_generator.pow_felt(&(global_values.trace_length - FELT_16));
    let pow15 = trace_generator
        .pow_felt(&((FELT_251 * global_values.trace_length).floor_div(&felt_nonzero!(FELT_256))));
    let pow16 =
        trace_generator.pow_felt(&(global_values.trace_length.floor_div(&felt_nonzero!(FELT_2))));
    let pow17 = trace_generator
        .pow_felt(&((FELT_63 * global_values.trace_length).floor_div(&felt_nonzero!(FELT_64))));
    let pow18 = trace_generator
        .pow_felt(&((FELT_255 * global_values.trace_length).floor_div(&felt_nonzero!(FELT_256))));
    let pow19 = trace_generator
        .pow_felt(&((FELT_15 * global_values.trace_length).floor_div(&felt_nonzero!(FELT_16))));

    // Compute domains.
    let domain0 = pow9 - FELT_1;
    let domain1 = pow8 - FELT_1;
    let domain2 = pow7 - FELT_1;
    let domain3 = pow6 - pow19;
    let domain4 = pow6 - FELT_1;
    let domain5 = pow5 - FELT_1;
    let domain6 = pow4 - FELT_1;
    let domain7 = pow3 - FELT_1;
    let domain8 = pow3 - pow18;
    let domain9 = pow3 - pow17;
    let domain10 = pow2 - pow16;
    let domain11 = pow2 - FELT_1;
    let domain12 = pow1 - pow18;
    let domain13 = pow1 - pow15;
    let domain14 = pow1 - FELT_1;
    let domain15 = pow0 - pow18;
    let domain16 = pow0 - pow15;
    let domain17 = pow0 - FELT_1;
    let domain18 = point - pow14;
    let domain19 = point - FELT_1;
    let domain20 = point - pow13;
    let domain21 = point - pow12;
    let domain22 = point - pow11;
    let domain23 = point - pow10;

    // Fetch mask variables.
    let column0_row0 = mask_values[0];
    let column0_row1 = mask_values[1];
    let column0_row4 = mask_values[2];
    let column0_row8 = mask_values[3];
    let column0_row12 = mask_values[4];
    let column0_row28 = mask_values[5];
    let column0_row44 = mask_values[6];
    let column0_row60 = mask_values[7];
    let column0_row76 = mask_values[8];
    let column0_row92 = mask_values[9];
    let column0_row108 = mask_values[10];
    let column0_row124 = mask_values[11];
    let column1_row0 = mask_values[12];
    let column1_row1 = mask_values[13];
    let column1_row2 = mask_values[14];
    let column1_row3 = mask_values[15];
    let column1_row4 = mask_values[16];
    let column1_row5 = mask_values[17];
    let column1_row6 = mask_values[18];
    let column1_row7 = mask_values[19];
    let column1_row8 = mask_values[20];
    let column1_row9 = mask_values[21];
    let column1_row10 = mask_values[22];
    let column1_row11 = mask_values[23];
    let column1_row12 = mask_values[24];
    let column1_row13 = mask_values[25];
    let column1_row14 = mask_values[26];
    let column1_row15 = mask_values[27];
    let column2_row0 = mask_values[28];
    let column2_row1 = mask_values[29];
    let column3_row0 = mask_values[30];
    let column3_row1 = mask_values[31];
    let column3_row255 = mask_values[32];
    let column3_row256 = mask_values[33];
    let column3_row511 = mask_values[34];
    let column4_row0 = mask_values[35];
    let column4_row1 = mask_values[36];
    let column4_row255 = mask_values[37];
    let column4_row256 = mask_values[38];
    let column5_row0 = mask_values[39];
    let column5_row1 = mask_values[40];
    let column5_row192 = mask_values[41];
    let column5_row193 = mask_values[42];
    let column5_row196 = mask_values[43];
    let column5_row197 = mask_values[44];
    let column5_row251 = mask_values[45];
    let column5_row252 = mask_values[46];
    let column5_row256 = mask_values[47];
    let column6_row0 = mask_values[48];
    let column6_row1 = mask_values[49];
    let column6_row255 = mask_values[50];
    let column6_row256 = mask_values[51];
    let column6_row511 = mask_values[52];
    let column7_row0 = mask_values[53];
    let column7_row1 = mask_values[54];
    let column7_row255 = mask_values[55];
    let column7_row256 = mask_values[56];
    let column8_row0 = mask_values[57];
    let column8_row1 = mask_values[58];
    let column8_row192 = mask_values[59];
    let column8_row193 = mask_values[60];
    let column8_row196 = mask_values[61];
    let column8_row197 = mask_values[62];
    let column8_row251 = mask_values[63];
    let column8_row252 = mask_values[64];
    let column8_row256 = mask_values[65];
    let column9_row0 = mask_values[66];
    let column9_row1 = mask_values[67];
    let column9_row255 = mask_values[68];
    let column9_row256 = mask_values[69];
    let column9_row511 = mask_values[70];
    let column10_row0 = mask_values[71];
    let column10_row1 = mask_values[72];
    let column10_row255 = mask_values[73];
    let column10_row256 = mask_values[74];
    let column11_row0 = mask_values[75];
    let column11_row1 = mask_values[76];
    let column11_row192 = mask_values[77];
    let column11_row193 = mask_values[78];
    let column11_row196 = mask_values[79];
    let column11_row197 = mask_values[80];
    let column11_row251 = mask_values[81];
    let column11_row252 = mask_values[82];
    let column11_row256 = mask_values[83];
    let column12_row0 = mask_values[84];
    let column12_row1 = mask_values[85];
    let column12_row255 = mask_values[86];
    let column12_row256 = mask_values[87];
    let column12_row511 = mask_values[88];
    let column13_row0 = mask_values[89];
    let column13_row1 = mask_values[90];
    let column13_row255 = mask_values[91];
    let column13_row256 = mask_values[92];
    let column14_row0 = mask_values[93];
    let column14_row1 = mask_values[94];
    let column14_row192 = mask_values[95];
    let column14_row193 = mask_values[96];
    let column14_row196 = mask_values[97];
    let column14_row197 = mask_values[98];
    let column14_row251 = mask_values[99];
    let column14_row252 = mask_values[100];
    let column14_row256 = mask_values[101];
    let column15_row0 = mask_values[102];
    let column15_row255 = mask_values[103];
    let column16_row0 = mask_values[104];
    let column16_row255 = mask_values[105];
    let column17_row0 = mask_values[106];
    let column17_row255 = mask_values[107];
    let column18_row0 = mask_values[108];
    let column18_row255 = mask_values[109];
    let column19_row0 = mask_values[110];
    let column19_row1 = mask_values[111];
    let column19_row2 = mask_values[112];
    let column19_row3 = mask_values[113];
    let column19_row4 = mask_values[114];
    let column19_row5 = mask_values[115];
    let column19_row6 = mask_values[116];
    let column19_row7 = mask_values[117];
    let column19_row8 = mask_values[118];
    let column19_row9 = mask_values[119];
    let column19_row12 = mask_values[120];
    let column19_row13 = mask_values[121];
    let column19_row16 = mask_values[122];
    let column19_row22 = mask_values[123];
    let column19_row23 = mask_values[124];
    let column19_row38 = mask_values[125];
    let column19_row39 = mask_values[126];
    let column19_row70 = mask_values[127];
    let column19_row71 = mask_values[128];
    let column19_row102 = mask_values[129];
    let column19_row103 = mask_values[130];
    let column19_row134 = mask_values[131];
    let column19_row135 = mask_values[132];
    let column19_row167 = mask_values[133];
    let column19_row199 = mask_values[134];
    let column19_row230 = mask_values[135];
    let column19_row263 = mask_values[136];
    let column19_row295 = mask_values[137];
    let column19_row327 = mask_values[138];
    let column19_row391 = mask_values[139];
    let column19_row423 = mask_values[140];
    let column19_row455 = mask_values[141];
    let column19_row4118 = mask_values[142];
    let column19_row4119 = mask_values[143];
    let column19_row8214 = mask_values[144];
    let column20_row0 = mask_values[145];
    let column20_row1 = mask_values[146];
    let column20_row2 = mask_values[147];
    let column20_row3 = mask_values[148];
    let column21_row0 = mask_values[149];
    let column21_row1 = mask_values[150];
    let column21_row2 = mask_values[151];
    let column21_row3 = mask_values[152];
    let column21_row4 = mask_values[153];
    let column21_row5 = mask_values[154];
    let column21_row6 = mask_values[155];
    let column21_row7 = mask_values[156];
    let column21_row8 = mask_values[157];
    let column21_row9 = mask_values[158];
    let column21_row10 = mask_values[159];
    let column21_row11 = mask_values[160];
    let column21_row12 = mask_values[161];
    let column21_row13 = mask_values[162];
    let column21_row14 = mask_values[163];
    let column21_row15 = mask_values[164];
    let column21_row16 = mask_values[165];
    let column21_row17 = mask_values[166];
    let column21_row21 = mask_values[167];
    let column21_row22 = mask_values[168];
    let column21_row23 = mask_values[169];
    let column21_row24 = mask_values[170];
    let column21_row25 = mask_values[171];
    let column21_row30 = mask_values[172];
    let column21_row31 = mask_values[173];
    let column21_row39 = mask_values[174];
    let column21_row47 = mask_values[175];
    let column21_row55 = mask_values[176];
    let column21_row4081 = mask_values[177];
    let column21_row4083 = mask_values[178];
    let column21_row4089 = mask_values[179];
    let column21_row4091 = mask_values[180];
    let column21_row4093 = mask_values[181];
    let column21_row4102 = mask_values[182];
    let column21_row4110 = mask_values[183];
    let column21_row8167 = mask_values[184];
    let column21_row8177 = mask_values[185];
    let column21_row8179 = mask_values[186];
    let column21_row8183 = mask_values[187];
    let column21_row8185 = mask_values[188];
    let column21_row8187 = mask_values[189];
    let column21_row8191 = mask_values[190];
    let column22_row0 = mask_values[191];
    let column22_row16 = mask_values[192];
    let column22_row80 = mask_values[193];
    let column22_row144 = mask_values[194];
    let column22_row208 = mask_values[195];
    let column22_row8160 = mask_values[196];
    let column23_inter1_row0 = mask_values[197];
    let column23_inter1_row1 = mask_values[198];
    let column24_inter1_row0 = mask_values[199];
    let column24_inter1_row2 = mask_values[200];

    // Compute intermediate values.
    let cpu_decode_opcode_range_check_bit_0 = column1_row0 - (column1_row1 + column1_row1);
    let cpu_decode_opcode_range_check_bit_2 = column1_row2 - (column1_row3 + column1_row3);
    let cpu_decode_opcode_range_check_bit_4 = column1_row4 - (column1_row5 + column1_row5);
    let cpu_decode_opcode_range_check_bit_3 = column1_row3 - (column1_row4 + column1_row4);
    let cpu_decode_flag_op1_base_op0_0 = FELT_1
        - (cpu_decode_opcode_range_check_bit_2
            + cpu_decode_opcode_range_check_bit_4
            + cpu_decode_opcode_range_check_bit_3);
    let cpu_decode_opcode_range_check_bit_5 = column1_row5 - (column1_row6 + column1_row6);
    let cpu_decode_opcode_range_check_bit_6 = column1_row6 - (column1_row7 + column1_row7);
    let cpu_decode_opcode_range_check_bit_9 = column1_row9 - (column1_row10 + column1_row10);
    let cpu_decode_flag_res_op1_0 = FELT_1
        - (cpu_decode_opcode_range_check_bit_5
            + cpu_decode_opcode_range_check_bit_6
            + cpu_decode_opcode_range_check_bit_9);
    let cpu_decode_opcode_range_check_bit_7 = column1_row7 - (column1_row8 + column1_row8);
    let cpu_decode_opcode_range_check_bit_8 = column1_row8 - (column1_row9 + column1_row9);
    let cpu_decode_flag_pc_update_regular_0 = FELT_1
        - (cpu_decode_opcode_range_check_bit_7
            + cpu_decode_opcode_range_check_bit_8
            + cpu_decode_opcode_range_check_bit_9);
    let cpu_decode_opcode_range_check_bit_12 = column1_row12 - (column1_row13 + column1_row13);
    let cpu_decode_opcode_range_check_bit_13 = column1_row13 - (column1_row14 + column1_row14);
    let cpu_decode_fp_update_regular_0 =
        FELT_1 - (cpu_decode_opcode_range_check_bit_12 + cpu_decode_opcode_range_check_bit_13);
    let cpu_decode_opcode_range_check_bit_1 = column1_row1 - (column1_row2 + column1_row2);
    let npc_reg_0 = column19_row0 + cpu_decode_opcode_range_check_bit_2 + FELT_1;
    let cpu_decode_opcode_range_check_bit_10 = column1_row10 - (column1_row11 + column1_row11);
    let cpu_decode_opcode_range_check_bit_11 = column1_row11 - (column1_row12 + column1_row12);
    let cpu_decode_opcode_range_check_bit_14 = column1_row14 - (column1_row15 + column1_row15);
    let memory_address_diff_0 = column20_row2 - column20_row0;
    let range_check16_diff_0 = column2_row1 - column2_row0;
    let pedersen_hash0_ec_subset_sum_bit_0 = column5_row0 - (column5_row1 + column5_row1);
    let pedersen_hash0_ec_subset_sum_bit_neg_0 = FELT_1 - pedersen_hash0_ec_subset_sum_bit_0;
    let pedersen_hash1_ec_subset_sum_bit_0 = column8_row0 - (column8_row1 + column8_row1);
    let pedersen_hash1_ec_subset_sum_bit_neg_0 = FELT_1 - pedersen_hash1_ec_subset_sum_bit_0;
    let pedersen_hash2_ec_subset_sum_bit_0 = column11_row0 - (column11_row1 + column11_row1);
    let pedersen_hash2_ec_subset_sum_bit_neg_0 = FELT_1 - pedersen_hash2_ec_subset_sum_bit_0;
    let pedersen_hash3_ec_subset_sum_bit_0 = column14_row0 - (column14_row1 + column14_row1);
    let pedersen_hash3_ec_subset_sum_bit_neg_0 = FELT_1 - pedersen_hash3_ec_subset_sum_bit_0;
    let range_check_builtin_value0_0 = column0_row12;
    let range_check_builtin_value1_0 =
        range_check_builtin_value0_0 * global_values.offset_size + column0_row28;
    let range_check_builtin_value2_0 =
        range_check_builtin_value1_0 * global_values.offset_size + column0_row44;
    let range_check_builtin_value3_0 =
        range_check_builtin_value2_0 * global_values.offset_size + column0_row60;
    let range_check_builtin_value4_0 =
        range_check_builtin_value3_0 * global_values.offset_size + column0_row76;
    let range_check_builtin_value5_0 =
        range_check_builtin_value4_0 * global_values.offset_size + column0_row92;
    let range_check_builtin_value6_0 =
        range_check_builtin_value5_0 * global_values.offset_size + column0_row108;
    let range_check_builtin_value7_0 =
        range_check_builtin_value6_0 * global_values.offset_size + column0_row124;
    let ecdsa_signature0_doubling_key_x_squared = column21_row6 * column21_row6;
    let ecdsa_signature0_exponentiate_generator_bit_0 =
        column21_row15 - (column21_row47 + column21_row47);
    let ecdsa_signature0_exponentiate_generator_bit_neg_0 =
        FELT_1 - ecdsa_signature0_exponentiate_generator_bit_0;
    let ecdsa_signature0_exponentiate_key_bit_0 = column21_row5 - (column21_row21 + column21_row21);
    let ecdsa_signature0_exponentiate_key_bit_neg_0 =
        FELT_1 - ecdsa_signature0_exponentiate_key_bit_0;

    // Sum constraints.
    let total_sum = FELT_0;

    // Constraint: cpu/decode/opcode_range_check/bit.
    let value = (cpu_decode_opcode_range_check_bit_0 * cpu_decode_opcode_range_check_bit_0
        - cpu_decode_opcode_range_check_bit_0)
        * domain3.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[0] * value;

    // Constraint: cpu/decode/opcode_range_check/zero.
    let value = (column1_row0).field_div(&felt_nonzero!(domain3));
    let total_sum = total_sum + constraint_coefficients[1] * value;

    // Constraint: cpu/decode/opcode_range_check_input.
    let value = (column19_row1
        - (((column1_row0 * global_values.offset_size + column0_row4)
            * global_values.offset_size
            + column0_row8)
            * global_values.offset_size
            + column0_row0))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[2] * value;

    // Constraint: cpu/decode/flag_op1_base_op0_bit.
    let value = (cpu_decode_flag_op1_base_op0_0 * cpu_decode_flag_op1_base_op0_0
        - cpu_decode_flag_op1_base_op0_0)
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[3] * value;

    // Constraint: cpu/decode/flag_res_op1_bit.
    let value = (cpu_decode_flag_res_op1_0 * cpu_decode_flag_res_op1_0 - cpu_decode_flag_res_op1_0)
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[4] * value;

    // Constraint: cpu/decode/flag_pc_update_regular_bit.
    let value = (cpu_decode_flag_pc_update_regular_0 * cpu_decode_flag_pc_update_regular_0
        - cpu_decode_flag_pc_update_regular_0)
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[5] * value;

    // Constraint: cpu/decode/fp_update_regular_bit.
    let value = (cpu_decode_fp_update_regular_0 * cpu_decode_fp_update_regular_0
        - cpu_decode_fp_update_regular_0)
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[6] * value;

    // Constraint: cpu/operands/mem_dst_addr.
    let value = (column19_row8 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_0 * column21_row8
            + (FELT_1 - cpu_decode_opcode_range_check_bit_0) * column21_row0
            + column0_row0))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[7] * value;

    // Constraint: cpu/operands/mem0_addr.
    let value = (column19_row4 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_1 * column21_row8
            + (FELT_1 - cpu_decode_opcode_range_check_bit_1) * column21_row0
            + column0_row8))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[8] * value;

    // Constraint: cpu/operands/mem1_addr.
    let value = (column19_row12 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_2 * column19_row0
            + cpu_decode_opcode_range_check_bit_4 * column21_row0
            + cpu_decode_opcode_range_check_bit_3 * column21_row8
            + cpu_decode_flag_op1_base_op0_0 * column19_row5
            + column0_row4))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[9] * value;

    // Constraint: cpu/operands/ops_mul.
    let value = (column21_row4 - column19_row5 * column19_row13).field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[10] * value;

    // Constraint: cpu/operands/res.
    let value = ((FELT_1 - cpu_decode_opcode_range_check_bit_9) * column21_row12
        - (cpu_decode_opcode_range_check_bit_5 * (column19_row5 + column19_row13)
            + cpu_decode_opcode_range_check_bit_6 * column21_row4
            + cpu_decode_flag_res_op1_0 * column19_row13))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[11] * value;

    // Constraint: cpu/update_registers/update_pc/tmp0.
    let value = (column21_row2 - cpu_decode_opcode_range_check_bit_9 * column19_row9)
        * domain18.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[12] * value;

    // Constraint: cpu/update_registers/update_pc/tmp1.
    let value = (column21_row10 - column21_row2 * column21_row12)
        * domain18.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[13] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_negative.
    let value = ((FELT_1 - cpu_decode_opcode_range_check_bit_9) * column19_row16
        + column21_row2 * (column19_row16 - (column19_row0 + column19_row13))
        - (cpu_decode_flag_pc_update_regular_0 * npc_reg_0
            + cpu_decode_opcode_range_check_bit_7 * column21_row12
            + cpu_decode_opcode_range_check_bit_8 * (column19_row0 + column21_row12)))
        * domain18.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[14] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_positive.
    let value = ((column21_row10 - cpu_decode_opcode_range_check_bit_9)
        * (column19_row16 - npc_reg_0))
        * domain18.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[15] * value;

    // Constraint: cpu/update_registers/update_ap/ap_update.
    let value = (column21_row16
        - (column21_row0
            + cpu_decode_opcode_range_check_bit_10 * column21_row12
            + cpu_decode_opcode_range_check_bit_11
            + cpu_decode_opcode_range_check_bit_12 * FELT_2))
        * domain18.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[16] * value;

    // Constraint: cpu/update_registers/update_fp/fp_update.
    let value = (column21_row24
        - (cpu_decode_fp_update_regular_0 * column21_row8
            + cpu_decode_opcode_range_check_bit_13 * column19_row9
            + cpu_decode_opcode_range_check_bit_12 * (column21_row0 + FELT_2)))
        * domain18.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[17] * value;

    // Constraint: cpu/opcodes/call/push_fp.
    let value = (cpu_decode_opcode_range_check_bit_12 * (column19_row9 - column21_row8))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[18] * value;

    // Constraint: cpu/opcodes/call/push_pc.
    let value = (cpu_decode_opcode_range_check_bit_12
        * (column19_row5 - (column19_row0 + cpu_decode_opcode_range_check_bit_2 + FELT_1)))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[19] * value;

    // Constraint: cpu/opcodes/call/off0.
    let value = (cpu_decode_opcode_range_check_bit_12
        * (column0_row0 - global_values.half_offset_size))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[20] * value;

    // Constraint: cpu/opcodes/call/off1.
    let value = (cpu_decode_opcode_range_check_bit_12
        * (column0_row8 - (global_values.half_offset_size + FELT_1)))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[21] * value;

    // Constraint: cpu/opcodes/call/flags.
    let value = (cpu_decode_opcode_range_check_bit_12
        * (cpu_decode_opcode_range_check_bit_12
            + cpu_decode_opcode_range_check_bit_12
            + FELT_1
            + FELT_1
            - (cpu_decode_opcode_range_check_bit_0
                + cpu_decode_opcode_range_check_bit_1
                + FELT_4)))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[22] * value;

    // Constraint: cpu/opcodes/ret/off0.
    let value = (cpu_decode_opcode_range_check_bit_13
        * (column0_row0 + FELT_2 - global_values.half_offset_size))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[23] * value;

    // Constraint: cpu/opcodes/ret/off2.
    let value = (cpu_decode_opcode_range_check_bit_13
        * (column0_row4 + FELT_1 - global_values.half_offset_size))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[24] * value;

    // Constraint: cpu/opcodes/ret/flags.
    let value = (cpu_decode_opcode_range_check_bit_13
        * (cpu_decode_opcode_range_check_bit_7
            + cpu_decode_opcode_range_check_bit_0
            + cpu_decode_opcode_range_check_bit_3
            + cpu_decode_flag_res_op1_0
            - FELT_4))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[25] * value;

    // Constraint: cpu/opcodes/assert_eq/assert_eq.
    let value = (cpu_decode_opcode_range_check_bit_14 * (column19_row9 - column21_row12))
        .field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[26] * value;

    // Constraint: initial_ap.
    let value = (column21_row0 - global_values.initial_ap).field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[27] * value;

    // Constraint: initial_fp.
    let value = (column21_row8 - global_values.initial_ap).field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[28] * value;

    // Constraint: initial_pc.
    let value = (column19_row0 - global_values.initial_pc).field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[29] * value;

    // Constraint: final_ap.
    let value = (column21_row0 - global_values.final_ap).field_div(&felt_nonzero!(domain18));
    let total_sum = total_sum + constraint_coefficients[30] * value;

    // Constraint: final_fp.
    let value = (column21_row8 - global_values.initial_ap).field_div(&felt_nonzero!(domain18));
    let total_sum = total_sum + constraint_coefficients[31] * value;

    // Constraint: final_pc.
    let value = (column19_row0 - global_values.final_pc).field_div(&felt_nonzero!(domain18));
    let total_sum = total_sum + constraint_coefficients[32] * value;

    // Constraint: memory/multi_column_perm/perm/init0.
    let value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column20_row0
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column20_row1))
        * column24_inter1_row0
        + column19_row0
        + global_values.memory_multi_column_perm_hash_interaction_elm0 * column19_row1
        - global_values.memory_multi_column_perm_perm_interaction_elm)
        .field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[33] * value;

    // Constraint: memory/multi_column_perm/perm/step0.
    let value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column20_row2
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column20_row3))
        * column24_inter1_row2
        - (global_values.memory_multi_column_perm_perm_interaction_elm
            - (column19_row2
                + global_values.memory_multi_column_perm_hash_interaction_elm0 * column19_row3))
            * column24_inter1_row0)
        * domain20.field_div(&felt_nonzero!(domain1));
    let total_sum = total_sum + constraint_coefficients[34] * value;

    // Constraint: memory/multi_column_perm/perm/last.
    let value = (column24_inter1_row0
        - global_values.memory_multi_column_perm_perm_public_memory_prod)
        .field_div(&felt_nonzero!(domain20));
    let total_sum = total_sum + constraint_coefficients[35] * value;

    // Constraint: memory/diff_is_bit.
    let value = (memory_address_diff_0 * memory_address_diff_0 - memory_address_diff_0)
        * domain20.field_div(&felt_nonzero!(domain1));
    let total_sum = total_sum + constraint_coefficients[36] * value;

    // Constraint: memory/is_func.
    let value = ((memory_address_diff_0 - FELT_1) * (column20_row1 - column20_row3))
        * domain20.field_div(&felt_nonzero!(domain1));
    let total_sum = total_sum + constraint_coefficients[37] * value;

    // Constraint: memory/initial_addr.
    let value = (column20_row0 - FELT_1).field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[38] * value;

    // Constraint: public_memory_addr_zero.
    let value = (column19_row2).field_div(&felt_nonzero!(domain2));
    let total_sum = total_sum + constraint_coefficients[39] * value;

    // Constraint: public_memory_value_zero.
    let value = (column19_row3).field_div(&felt_nonzero!(domain2));
    let total_sum = total_sum + constraint_coefficients[40] * value;

    // Constraint: range_check16/perm/init0.
    let value = ((global_values.range_check16_perm_interaction_elm - column2_row0)
        * column23_inter1_row0
        + column0_row0
        - global_values.range_check16_perm_interaction_elm)
        .field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[41] * value;

    // Constraint: range_check16/perm/step0.
    let value = ((global_values.range_check16_perm_interaction_elm - column2_row1)
        * column23_inter1_row1
        - (global_values.range_check16_perm_interaction_elm - column0_row1) * column23_inter1_row0)
        * domain21.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[42] * value;

    // Constraint: range_check16/perm/last.
    let value = (column23_inter1_row0 - global_values.range_check16_perm_public_memory_prod)
        .field_div(&felt_nonzero!(domain21));
    let total_sum = total_sum + constraint_coefficients[43] * value;

    // Constraint: range_check16/diff_is_bit.
    let value = (range_check16_diff_0 * range_check16_diff_0 - range_check16_diff_0)
        * domain21.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[44] * value;

    // Constraint: range_check16/minimum.
    let value = (column2_row0 - global_values.range_check_min).field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[45] * value;

    // Constraint: range_check16/maximum.
    let value = (column2_row0 - global_values.range_check_max).field_div(&felt_nonzero!(domain21));
    let total_sum = total_sum + constraint_coefficients[46] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/last_one_is_zero.
    let value = (column16_row255 * (column5_row0 - (column5_row1 + column5_row1)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[47] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    let value = (column16_row255
        * (column5_row1
            - FELT_3138550867693340381917894711603833208051177722232017256448 * column5_row192))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[48] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit192.
    let value = (column16_row255
        - column15_row255 * (column5_row192 - (column5_row193 + column5_row193)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[49] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    let value = (column15_row255 * (column5_row193 - FELT_8 * column5_row196))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[50] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit196.
    let value = (column15_row255
        - (column5_row251 - (column5_row252 + column5_row252))
            * (column5_row196 - (column5_row197 + column5_row197)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[51] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    let value = ((column5_row251 - (column5_row252 + column5_row252))
        * (column5_row197 - FELT_18014398509481984 * column5_row251))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[52] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/booleanity_test.
    let value = (pedersen_hash0_ec_subset_sum_bit_0
        * (pedersen_hash0_ec_subset_sum_bit_0 - FELT_1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[53] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_extraction_end.
    let value = (column5_row0).field_div(&felt_nonzero!(domain9));
    let total_sum = total_sum + constraint_coefficients[54] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/zeros_tail.
    let value = (column5_row0).field_div(&felt_nonzero!(domain8));
    let total_sum = total_sum + constraint_coefficients[55] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/slope.
    let value = (pedersen_hash0_ec_subset_sum_bit_0
        * (column4_row0 - global_values.pedersen_points_y)
        - column15_row0 * (column3_row0 - global_values.pedersen_points_x))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[56] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/x.
    let value = (column15_row0 * column15_row0
        - pedersen_hash0_ec_subset_sum_bit_0
            * (column3_row0 + global_values.pedersen_points_x + column3_row1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[57] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/y.
    let value = (pedersen_hash0_ec_subset_sum_bit_0 * (column4_row0 + column4_row1)
        - column15_row0 * (column3_row0 - column3_row1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[58] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/x.
    let value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column3_row1 - column3_row0))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[59] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/y.
    let value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column4_row1 - column4_row0))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[60] * value;

    // Constraint: pedersen/hash0/copy_point/x.
    let value = (column3_row256 - column3_row255) * domain10.field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[61] * value;

    // Constraint: pedersen/hash0/copy_point/y.
    let value = (column4_row256 - column4_row255) * domain10.field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[62] * value;

    // Constraint: pedersen/hash0/init/x.
    let value =
        (column3_row0 - global_values.pedersen_shift_point.x).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[63] * value;

    // Constraint: pedersen/hash0/init/y.
    let value =
        (column4_row0 - global_values.pedersen_shift_point.y).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[64] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/last_one_is_zero.
    let value = (column18_row255 * (column8_row0 - (column8_row1 + column8_row1)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[65] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    let value = (column18_row255
        * (column8_row1
            - FELT_3138550867693340381917894711603833208051177722232017256448 * column8_row192))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[66] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/cumulative_bit192.
    let value = (column18_row255
        - column17_row255 * (column8_row192 - (column8_row193 + column8_row193)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[67] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    let value = (column17_row255 * (column8_row193 - FELT_8 * column8_row196))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[68] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/cumulative_bit196.
    let value = (column17_row255
        - (column8_row251 - (column8_row252 + column8_row252))
            * (column8_row196 - (column8_row197 + column8_row197)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[69] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    let value = ((column8_row251 - (column8_row252 + column8_row252))
        * (column8_row197 - FELT_18014398509481984 * column8_row251))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[70] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/booleanity_test.
    let value = (pedersen_hash1_ec_subset_sum_bit_0
        * (pedersen_hash1_ec_subset_sum_bit_0 - FELT_1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[71] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_extraction_end.
    let value = (column8_row0).field_div(&felt_nonzero!(domain9));
    let total_sum = total_sum + constraint_coefficients[72] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/zeros_tail.
    let value = (column8_row0).field_div(&felt_nonzero!(domain8));
    let total_sum = total_sum + constraint_coefficients[73] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/add_points/slope.
    let value = (pedersen_hash1_ec_subset_sum_bit_0
        * (column7_row0 - global_values.pedersen_points_y)
        - column16_row0 * (column6_row0 - global_values.pedersen_points_x))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[74] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/add_points/x.
    let value = (column16_row0 * column16_row0
        - pedersen_hash1_ec_subset_sum_bit_0
            * (column6_row0 + global_values.pedersen_points_x + column6_row1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[75] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/add_points/y.
    let value = (pedersen_hash1_ec_subset_sum_bit_0 * (column7_row0 + column7_row1)
        - column16_row0 * (column6_row0 - column6_row1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[76] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/copy_point/x.
    let value = (pedersen_hash1_ec_subset_sum_bit_neg_0 * (column6_row1 - column6_row0))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[77] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/copy_point/y.
    let value = (pedersen_hash1_ec_subset_sum_bit_neg_0 * (column7_row1 - column7_row0))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[78] * value;

    // Constraint: pedersen/hash1/copy_point/x.
    let value = (column6_row256 - column6_row255) * domain10.field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[79] * value;

    // Constraint: pedersen/hash1/copy_point/y.
    let value = (column7_row256 - column7_row255) * domain10.field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[80] * value;

    // Constraint: pedersen/hash1/init/x.
    let value =
        (column6_row0 - global_values.pedersen_shift_point.x).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[81] * value;

    // Constraint: pedersen/hash1/init/y.
    let value =
        (column7_row0 - global_values.pedersen_shift_point.y).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[82] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/last_one_is_zero.
    let value = (column22_row144 * (column11_row0 - (column11_row1 + column11_row1)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[83] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    let value = (column22_row144
        * (column11_row1
            - FELT_3138550867693340381917894711603833208051177722232017256448 * column11_row192))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[84] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/cumulative_bit192.
    let value = (column22_row144
        - column22_row16 * (column11_row192 - (column11_row193 + column11_row193)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[85] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    let value = (column22_row16 * (column11_row193 - FELT_8 * column11_row196))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[86] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/cumulative_bit196.
    let value = (column22_row16
        - (column11_row251 - (column11_row252 + column11_row252))
            * (column11_row196 - (column11_row197 + column11_row197)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[87] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    let value = ((column11_row251 - (column11_row252 + column11_row252))
        * (column11_row197 - FELT_18014398509481984 * column11_row251))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[88] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/booleanity_test.
    let value = (pedersen_hash2_ec_subset_sum_bit_0
        * (pedersen_hash2_ec_subset_sum_bit_0 - FELT_1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[89] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_extraction_end.
    let value = (column11_row0).field_div(&felt_nonzero!(domain9));
    let total_sum = total_sum + constraint_coefficients[90] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/zeros_tail.
    let value = (column11_row0).field_div(&felt_nonzero!(domain8));
    let total_sum = total_sum + constraint_coefficients[91] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/add_points/slope.
    let value = (pedersen_hash2_ec_subset_sum_bit_0
        * (column10_row0 - global_values.pedersen_points_y)
        - column17_row0 * (column9_row0 - global_values.pedersen_points_x))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[92] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/add_points/x.
    let value = (column17_row0 * column17_row0
        - pedersen_hash2_ec_subset_sum_bit_0
            * (column9_row0 + global_values.pedersen_points_x + column9_row1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[93] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/add_points/y.
    let value = (pedersen_hash2_ec_subset_sum_bit_0 * (column10_row0 + column10_row1)
        - column17_row0 * (column9_row0 - column9_row1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[94] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/copy_point/x.
    let value = (pedersen_hash2_ec_subset_sum_bit_neg_0 * (column9_row1 - column9_row0))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[95] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/copy_point/y.
    let value = (pedersen_hash2_ec_subset_sum_bit_neg_0 * (column10_row1 - column10_row0))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[96] * value;

    // Constraint: pedersen/hash2/copy_point/x.
    let value = (column9_row256 - column9_row255) * domain10.field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[97] * value;

    // Constraint: pedersen/hash2/copy_point/y.
    let value = (column10_row256 - column10_row255) * domain10.field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[98] * value;

    // Constraint: pedersen/hash2/init/x.
    let value =
        (column9_row0 - global_values.pedersen_shift_point.x).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[99] * value;

    // Constraint: pedersen/hash2/init/y.
    let value =
        (column10_row0 - global_values.pedersen_shift_point.y).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[100] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/last_one_is_zero.
    let value = (column22_row208 * (column14_row0 - (column14_row1 + column14_row1)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[101] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    let value = (column22_row208
        * (column14_row1
            - FELT_3138550867693340381917894711603833208051177722232017256448 * column14_row192))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[102] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/cumulative_bit192.
    let value = (column22_row208
        - column22_row80 * (column14_row192 - (column14_row193 + column14_row193)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[103] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    let value = (column22_row80 * (column14_row193 - FELT_8 * column14_row196))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[104] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/cumulative_bit196.
    let value = (column22_row80
        - (column14_row251 - (column14_row252 + column14_row252))
            * (column14_row196 - (column14_row197 + column14_row197)))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[105] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    let value = ((column14_row251 - (column14_row252 + column14_row252))
        * (column14_row197 - FELT_18014398509481984 * column14_row251))
        .field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[106] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/booleanity_test.
    let value = (pedersen_hash3_ec_subset_sum_bit_0
        * (pedersen_hash3_ec_subset_sum_bit_0 - FELT_1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[107] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_extraction_end.
    let value = (column14_row0).field_div(&felt_nonzero!(domain9));
    let total_sum = total_sum + constraint_coefficients[108] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/zeros_tail.
    let value = (column14_row0).field_div(&felt_nonzero!(domain8));
    let total_sum = total_sum + constraint_coefficients[109] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/add_points/slope.
    let value = (pedersen_hash3_ec_subset_sum_bit_0
        * (column13_row0 - global_values.pedersen_points_y)
        - column18_row0 * (column12_row0 - global_values.pedersen_points_x))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[110] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/add_points/x.
    let value = (column18_row0 * column18_row0
        - pedersen_hash3_ec_subset_sum_bit_0
            * (column12_row0 + global_values.pedersen_points_x + column12_row1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[111] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/add_points/y.
    let value = (pedersen_hash3_ec_subset_sum_bit_0 * (column13_row0 + column13_row1)
        - column18_row0 * (column12_row0 - column12_row1))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[112] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/copy_point/x.
    let value = (pedersen_hash3_ec_subset_sum_bit_neg_0 * (column12_row1 - column12_row0))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[113] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/copy_point/y.
    let value = (pedersen_hash3_ec_subset_sum_bit_neg_0 * (column13_row1 - column13_row0))
        * domain8.field_div(&felt_nonzero!(domain0));
    let total_sum = total_sum + constraint_coefficients[114] * value;

    // Constraint: pedersen/hash3/copy_point/x.
    let value = (column12_row256 - column12_row255) * domain10.field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[115] * value;

    // Constraint: pedersen/hash3/copy_point/y.
    let value = (column13_row256 - column13_row255) * domain10.field_div(&felt_nonzero!(domain7));
    let total_sum = total_sum + constraint_coefficients[116] * value;

    // Constraint: pedersen/hash3/init/x.
    let value =
        (column12_row0 - global_values.pedersen_shift_point.x).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[117] * value;

    // Constraint: pedersen/hash3/init/y.
    let value =
        (column13_row0 - global_values.pedersen_shift_point.y).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[118] * value;

    // Constraint: pedersen/input0_value0.
    let value = (column19_row7 - column5_row0).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[119] * value;

    // Constraint: pedersen/input0_value1.
    let value = (column19_row135 - column8_row0).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[120] * value;

    // Constraint: pedersen/input0_value2.
    let value = (column19_row263 - column11_row0).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[121] * value;

    // Constraint: pedersen/input0_value3.
    let value = (column19_row391 - column14_row0).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[122] * value;

    // Constraint: pedersen/input0_addr.
    let value =
        (column19_row134 - (column19_row38 + FELT_1)) * domain22.field_div(&felt_nonzero!(domain6));
    let total_sum = total_sum + constraint_coefficients[123] * value;

    // Constraint: pedersen/init_addr.
    let value =
        (column19_row6 - global_values.initial_pedersen_addr).field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[124] * value;

    // Constraint: pedersen/input1_value0.
    let value = (column19_row71 - column5_row256).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[125] * value;

    // Constraint: pedersen/input1_value1.
    let value = (column19_row199 - column8_row256).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[126] * value;

    // Constraint: pedersen/input1_value2.
    let value = (column19_row327 - column11_row256).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[127] * value;

    // Constraint: pedersen/input1_value3.
    let value = (column19_row455 - column14_row256).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[128] * value;

    // Constraint: pedersen/input1_addr.
    let value = (column19_row70 - (column19_row6 + FELT_1)).field_div(&felt_nonzero!(domain6));
    let total_sum = total_sum + constraint_coefficients[129] * value;

    // Constraint: pedersen/output_value0.
    let value = (column19_row39 - column3_row511).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[130] * value;

    // Constraint: pedersen/output_value1.
    let value = (column19_row167 - column6_row511).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[131] * value;

    // Constraint: pedersen/output_value2.
    let value = (column19_row295 - column9_row511).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[132] * value;

    // Constraint: pedersen/output_value3.
    let value = (column19_row423 - column12_row511).field_div(&felt_nonzero!(domain11));
    let total_sum = total_sum + constraint_coefficients[133] * value;

    // Constraint: pedersen/output_addr.
    let value = (column19_row38 - (column19_row70 + FELT_1)).field_div(&felt_nonzero!(domain6));
    let total_sum = total_sum + constraint_coefficients[134] * value;

    // Constraint: range_check_builtin/value.
    let value = (range_check_builtin_value7_0 - column19_row103).field_div(&felt_nonzero!(domain6));
    let total_sum = total_sum + constraint_coefficients[135] * value;

    // Constraint: range_check_builtin/addr_step.
    let value = (column19_row230 - (column19_row102 + FELT_1))
        * domain22.field_div(&felt_nonzero!(domain6));
    let total_sum = total_sum + constraint_coefficients[136] * value;

    // Constraint: range_check_builtin/init_addr.
    let value = (column19_row102 - global_values.initial_range_check_addr)
        .field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[137] * value;

    // Constraint: ecdsa/signature0/doubling_key/slope.
    let value = (ecdsa_signature0_doubling_key_x_squared
        + ecdsa_signature0_doubling_key_x_squared
        + ecdsa_signature0_doubling_key_x_squared
        + global_values.ecdsa_sig_config.alpha
        - (column21_row14 + column21_row14) * column21_row13)
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[138] * value;

    // Constraint: ecdsa/signature0/doubling_key/x.
    let value = (column21_row13 * column21_row13
        - (column21_row6 + column21_row6 + column21_row22))
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[139] * value;

    // Constraint: ecdsa/signature0/doubling_key/y.
    let value = (column21_row14 + column21_row30
        - column21_row13 * (column21_row6 - column21_row22))
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[140] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/booleanity_test.
    let value = (ecdsa_signature0_exponentiate_generator_bit_0
        * (ecdsa_signature0_exponentiate_generator_bit_0 - FELT_1))
        * domain15.field_div(&felt_nonzero!(domain5));
    let total_sum = total_sum + constraint_coefficients[141] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/bit_extraction_end.
    let value = (column21_row15).field_div(&felt_nonzero!(domain16));
    let total_sum = total_sum + constraint_coefficients[142] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/zeros_tail.
    let value = (column21_row15).field_div(&felt_nonzero!(domain15));
    let total_sum = total_sum + constraint_coefficients[143] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/add_points/slope.
    let value = (ecdsa_signature0_exponentiate_generator_bit_0
        * (column21_row23 - global_values.ecdsa_generator_points_y)
        - column21_row31 * (column21_row7 - global_values.ecdsa_generator_points_x))
        * domain15.field_div(&felt_nonzero!(domain5));
    let total_sum = total_sum + constraint_coefficients[144] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/add_points/x.
    let value = (column21_row31 * column21_row31
        - ecdsa_signature0_exponentiate_generator_bit_0
            * (column21_row7 + global_values.ecdsa_generator_points_x + column21_row39))
        * domain15.field_div(&felt_nonzero!(domain5));
    let total_sum = total_sum + constraint_coefficients[145] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/add_points/y.
    let value = (ecdsa_signature0_exponentiate_generator_bit_0 * (column21_row23 + column21_row55)
        - column21_row31 * (column21_row7 - column21_row39))
        * domain15.field_div(&felt_nonzero!(domain5));
    let total_sum = total_sum + constraint_coefficients[146] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/add_points/x_diff_inv.
    let value = (column22_row0 * (column21_row7 - global_values.ecdsa_generator_points_x) - FELT_1)
        * domain15.field_div(&felt_nonzero!(domain5));
    let total_sum = total_sum + constraint_coefficients[147] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/copy_point/x.
    let value = (ecdsa_signature0_exponentiate_generator_bit_neg_0
        * (column21_row39 - column21_row7))
        * domain15.field_div(&felt_nonzero!(domain5));
    let total_sum = total_sum + constraint_coefficients[148] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/copy_point/y.
    let value = (ecdsa_signature0_exponentiate_generator_bit_neg_0
        * (column21_row55 - column21_row23))
        * domain15.field_div(&felt_nonzero!(domain5));
    let total_sum = total_sum + constraint_coefficients[149] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/booleanity_test.
    let value = (ecdsa_signature0_exponentiate_key_bit_0
        * (ecdsa_signature0_exponentiate_key_bit_0 - FELT_1))
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[150] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/bit_extraction_end.
    let value = (column21_row5).field_div(&felt_nonzero!(domain13));
    let total_sum = total_sum + constraint_coefficients[151] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/zeros_tail.
    let value = (column21_row5).field_div(&felt_nonzero!(domain12));
    let total_sum = total_sum + constraint_coefficients[152] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/add_points/slope.
    let value = (ecdsa_signature0_exponentiate_key_bit_0 * (column21_row9 - column21_row14)
        - column21_row3 * (column21_row1 - column21_row6))
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[153] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/add_points/x.
    let value = (column21_row3 * column21_row3
        - ecdsa_signature0_exponentiate_key_bit_0
            * (column21_row1 + column21_row6 + column21_row17))
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[154] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/add_points/y.
    let value = (ecdsa_signature0_exponentiate_key_bit_0 * (column21_row9 + column21_row25)
        - column21_row3 * (column21_row1 - column21_row17))
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[155] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/add_points/x_diff_inv.
    let value = (column21_row11 * (column21_row1 - column21_row6) - FELT_1)
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[156] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/copy_point/x.
    let value = (ecdsa_signature0_exponentiate_key_bit_neg_0 * (column21_row17 - column21_row1))
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[157] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/copy_point/y.
    let value = (ecdsa_signature0_exponentiate_key_bit_neg_0 * (column21_row25 - column21_row9))
        * domain12.field_div(&felt_nonzero!(domain4));
    let total_sum = total_sum + constraint_coefficients[158] * value;

    // Constraint: ecdsa/signature0/init_gen/x.
    let value = (column21_row7 - global_values.ecdsa_sig_config.shift_point.x)
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[159] * value;

    // Constraint: ecdsa/signature0/init_gen/y.
    let value = (column21_row23 + global_values.ecdsa_sig_config.shift_point.y)
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[160] * value;

    // Constraint: ecdsa/signature0/init_key/x.
    let value = (column21_row1 - global_values.ecdsa_sig_config.shift_point.x)
        .field_div(&felt_nonzero!(domain14));
    let total_sum = total_sum + constraint_coefficients[161] * value;

    // Constraint: ecdsa/signature0/init_key/y.
    let value = (column21_row9 - global_values.ecdsa_sig_config.shift_point.y)
        .field_div(&felt_nonzero!(domain14));
    let total_sum = total_sum + constraint_coefficients[162] * value;

    // Constraint: ecdsa/signature0/add_results/slope.
    let value = (column21_row8183
        - (column21_row4089 + column21_row8191 * (column21_row8167 - column21_row4081)))
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[163] * value;

    // Constraint: ecdsa/signature0/add_results/x.
    let value = (column21_row8191 * column21_row8191
        - (column21_row8167 + column21_row4081 + column21_row4102))
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[164] * value;

    // Constraint: ecdsa/signature0/add_results/y.
    let value = (column21_row8183 + column21_row4110
        - column21_row8191 * (column21_row8167 - column21_row4102))
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[165] * value;

    // Constraint: ecdsa/signature0/add_results/x_diff_inv.
    let value = (column22_row8160 * (column21_row8167 - column21_row4081) - FELT_1)
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[166] * value;

    // Constraint: ecdsa/signature0/extract_r/slope.
    let value = (column21_row8185 + global_values.ecdsa_sig_config.shift_point.y
        - column21_row4083 * (column21_row8177 - global_values.ecdsa_sig_config.shift_point.x))
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[167] * value;

    // Constraint: ecdsa/signature0/extract_r/x.
    let value = (column21_row4083 * column21_row4083
        - (column21_row8177 + global_values.ecdsa_sig_config.shift_point.x + column21_row5))
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[168] * value;

    // Constraint: ecdsa/signature0/extract_r/x_diff_inv.
    let value = (column21_row8179
        * (column21_row8177 - global_values.ecdsa_sig_config.shift_point.x)
        - FELT_1)
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[169] * value;

    // Constraint: ecdsa/signature0/z_nonzero.
    let value = (column21_row15 * column21_row4091 - FELT_1).field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[170] * value;

    // Constraint: ecdsa/signature0/r_and_w_nonzero.
    let value = (column21_row5 * column21_row4093 - FELT_1).field_div(&felt_nonzero!(domain14));
    let total_sum = total_sum + constraint_coefficients[171] * value;

    // Constraint: ecdsa/signature0/q_on_curve/x_squared.
    let value =
        (column21_row8187 - column21_row6 * column21_row6).field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[172] * value;

    // Constraint: ecdsa/signature0/q_on_curve/on_curve.
    let value = (column21_row14 * column21_row14
        - (column21_row6 * column21_row8187
            + global_values.ecdsa_sig_config.alpha * column21_row6
            + global_values.ecdsa_sig_config.beta))
        .field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[173] * value;

    // Constraint: ecdsa/init_addr.
    let value =
        (column19_row22 - global_values.initial_ecdsa_addr).field_div(&felt_nonzero!(domain19));
    let total_sum = total_sum + constraint_coefficients[174] * value;

    // Constraint: ecdsa/message_addr.
    let value = (column19_row4118 - (column19_row22 + FELT_1)).field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[175] * value;

    // Constraint: ecdsa/pubkey_addr.
    let value = (column19_row8214 - (column19_row4118 + FELT_1))
        * domain23.field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[176] * value;

    // Constraint: ecdsa/message_value0.
    let value = (column19_row4119 - column21_row15).field_div(&felt_nonzero!(domain17));
    let total_sum = total_sum + constraint_coefficients[177] * value;

    // Constraint: ecdsa/pubkey_value0.
    let value = (column19_row23 - column21_row6).field_div(&felt_nonzero!(domain17));

    total_sum + constraint_coefficients[178] * value
}
