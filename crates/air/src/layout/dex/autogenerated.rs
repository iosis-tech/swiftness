use super::global_values::GlobalValues;
use super::{CONSTRAINT_DEGREE, NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
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
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(8192))),
    );
    let pow1 = pow0 * pow0; // pow(point, (safe_div(global_values.trace_length, 4096))).
    let pow2 = point.pow_felt(
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(512))),
    );
    let pow3 = pow2 * pow2; // pow(point, (safe_div(global_values.trace_length, 256))).
    let pow4 = pow3 * pow3; // pow(point, (safe_div(global_values.trace_length, 128))).
    let pow5 = point.pow_felt(
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(32))),
    );
    let pow6 = pow5 * pow5; // pow(point, (safe_div(global_values.trace_length, 16))).
    let pow7 = pow6 * pow6; // pow(point, (safe_div(global_values.trace_length, 8))).
    let pow8 = pow7 * pow7; // pow(point, (safe_div(global_values.trace_length, 4))).
    let pow9 = pow8 * pow8; // pow(point, (safe_div(global_values.trace_length, 2))).
    let pow10 = pow9 * pow9; // pow(point, global_values.trace_length).
    let pow11 = trace_generator.pow_felt(&(global_values.trace_length - 8192));
    let pow12 = trace_generator.pow_felt(&(global_values.trace_length - 128));
    let pow13 = trace_generator.pow_felt(&(global_values.trace_length - 4));
    let pow14 = trace_generator.pow_felt(&(global_values.trace_length - 2));
    let pow15 = trace_generator.pow_felt(&(global_values.trace_length - 16));
    let pow16 = trace_generator.pow_felt(
        &(Felt::from(251)
            * global_values
                .trace_length
                .floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(256)))),
    );
    let pow17 = trace_generator.pow_felt(
        &(global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(2)))),
    );
    let pow18 = trace_generator.pow_felt(
        &(Felt::from(63)
            * global_values
                .trace_length
                .floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(64)))),
    );
    let pow19 = trace_generator.pow_felt(
        &(Felt::from(255)
            * global_values
                .trace_length
                .floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(256)))),
    );
    let pow20 = trace_generator.pow_felt(
        &(Felt::from(15)
            * global_values
                .trace_length
                .floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(16)))),
    );

    // Compute domains.
    let domain0 = pow10 - 1;
    let domain1 = pow9 - 1;
    let domain2 = pow8 - 1;
    let domain3 = pow7 - 1;
    let domain4 = pow6 - pow20;
    let domain5 = pow6 - 1;
    let domain6 = pow5 - 1;
    let domain7 = pow4 - 1;
    let domain8 = pow3 - 1;
    let domain9 = pow3 - pow19;
    let domain10 = pow3 - pow18;
    let domain11 = pow2 - pow17;
    let domain12 = pow2 - 1;
    let domain13 = pow1 - pow19;
    let domain14 = pow1 - pow16;
    let domain15 = pow1 - 1;
    let domain16 = pow0 - pow19;
    let domain17 = pow0 - pow16;
    let domain18 = pow0 - 1;
    let domain19 = point - pow15;
    let domain20 = point - 1;
    let domain21 = point - pow14;
    let domain22 = point - pow13;
    let domain23 = point - pow12;
    let domain24 = point - pow11;

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
    let column1_row255 = mask_values[18];
    let column1_row256 = mask_values[19];
    let column1_row511 = mask_values[20];
    let column2_row0 = mask_values[21];
    let column2_row1 = mask_values[22];
    let column2_row255 = mask_values[23];
    let column2_row256 = mask_values[24];
    let column3_row0 = mask_values[25];
    let column3_row1 = mask_values[26];
    let column3_row192 = mask_values[27];
    let column3_row193 = mask_values[28];
    let column3_row196 = mask_values[29];
    let column3_row197 = mask_values[30];
    let column3_row251 = mask_values[31];
    let column3_row252 = mask_values[32];
    let column3_row256 = mask_values[33];
    let column4_row0 = mask_values[34];
    let column4_row1 = mask_values[35];
    let column4_row255 = mask_values[36];
    let column4_row256 = mask_values[37];
    let column4_row511 = mask_values[38];
    let column5_row0 = mask_values[39];
    let column5_row1 = mask_values[40];
    let column5_row255 = mask_values[41];
    let column5_row256 = mask_values[42];
    let column6_row0 = mask_values[43];
    let column6_row1 = mask_values[44];
    let column6_row192 = mask_values[45];
    let column6_row193 = mask_values[46];
    let column6_row196 = mask_values[47];
    let column6_row197 = mask_values[48];
    let column6_row251 = mask_values[49];
    let column6_row252 = mask_values[50];
    let column6_row256 = mask_values[51];
    let column7_row0 = mask_values[52];
    let column7_row1 = mask_values[53];
    let column7_row255 = mask_values[54];
    let column7_row256 = mask_values[55];
    let column7_row511 = mask_values[56];
    let column8_row0 = mask_values[57];
    let column8_row1 = mask_values[58];
    let column8_row255 = mask_values[59];
    let column8_row256 = mask_values[60];
    let column9_row0 = mask_values[61];
    let column9_row1 = mask_values[62];
    let column9_row192 = mask_values[63];
    let column9_row193 = mask_values[64];
    let column9_row196 = mask_values[65];
    let column9_row197 = mask_values[66];
    let column9_row251 = mask_values[67];
    let column9_row252 = mask_values[68];
    let column9_row256 = mask_values[69];
    let column10_row0 = mask_values[70];
    let column10_row1 = mask_values[71];
    let column10_row255 = mask_values[72];
    let column10_row256 = mask_values[73];
    let column10_row511 = mask_values[74];
    let column11_row0 = mask_values[75];
    let column11_row1 = mask_values[76];
    let column11_row255 = mask_values[77];
    let column11_row256 = mask_values[78];
    let column12_row0 = mask_values[79];
    let column12_row1 = mask_values[80];
    let column12_row192 = mask_values[81];
    let column12_row193 = mask_values[82];
    let column12_row196 = mask_values[83];
    let column12_row197 = mask_values[84];
    let column12_row251 = mask_values[85];
    let column12_row252 = mask_values[86];
    let column12_row256 = mask_values[87];
    let column13_row0 = mask_values[88];
    let column13_row255 = mask_values[89];
    let column14_row0 = mask_values[90];
    let column14_row255 = mask_values[91];
    let column15_row0 = mask_values[92];
    let column15_row255 = mask_values[93];
    let column16_row0 = mask_values[94];
    let column16_row255 = mask_values[95];
    let column17_row0 = mask_values[96];
    let column17_row1 = mask_values[97];
    let column17_row2 = mask_values[98];
    let column17_row3 = mask_values[99];
    let column17_row4 = mask_values[100];
    let column17_row5 = mask_values[101];
    let column17_row6 = mask_values[102];
    let column17_row7 = mask_values[103];
    let column17_row8 = mask_values[104];
    let column17_row9 = mask_values[105];
    let column17_row12 = mask_values[106];
    let column17_row13 = mask_values[107];
    let column17_row16 = mask_values[108];
    let column17_row22 = mask_values[109];
    let column17_row23 = mask_values[110];
    let column17_row38 = mask_values[111];
    let column17_row39 = mask_values[112];
    let column17_row70 = mask_values[113];
    let column17_row71 = mask_values[114];
    let column17_row102 = mask_values[115];
    let column17_row103 = mask_values[116];
    let column17_row134 = mask_values[117];
    let column17_row135 = mask_values[118];
    let column17_row167 = mask_values[119];
    let column17_row199 = mask_values[120];
    let column17_row230 = mask_values[121];
    let column17_row263 = mask_values[122];
    let column17_row295 = mask_values[123];
    let column17_row327 = mask_values[124];
    let column17_row391 = mask_values[125];
    let column17_row423 = mask_values[126];
    let column17_row455 = mask_values[127];
    let column17_row4118 = mask_values[128];
    let column17_row4119 = mask_values[129];
    let column17_row8214 = mask_values[130];
    let column18_row0 = mask_values[131];
    let column18_row1 = mask_values[132];
    let column18_row2 = mask_values[133];
    let column18_row3 = mask_values[134];
    let column19_row0 = mask_values[135];
    let column19_row1 = mask_values[136];
    let column19_row2 = mask_values[137];
    let column19_row3 = mask_values[138];
    let column19_row4 = mask_values[139];
    let column19_row5 = mask_values[140];
    let column19_row6 = mask_values[141];
    let column19_row7 = mask_values[142];
    let column19_row8 = mask_values[143];
    let column19_row9 = mask_values[144];
    let column19_row11 = mask_values[145];
    let column19_row12 = mask_values[146];
    let column19_row13 = mask_values[147];
    let column19_row15 = mask_values[148];
    let column19_row17 = mask_values[149];
    let column19_row23 = mask_values[150];
    let column19_row25 = mask_values[151];
    let column19_row28 = mask_values[152];
    let column19_row31 = mask_values[153];
    let column19_row44 = mask_values[154];
    let column19_row60 = mask_values[155];
    let column19_row76 = mask_values[156];
    let column19_row92 = mask_values[157];
    let column19_row108 = mask_values[158];
    let column19_row124 = mask_values[159];
    let column19_row4103 = mask_values[160];
    let column19_row4111 = mask_values[161];
    let column20_row0 = mask_values[162];
    let column20_row1 = mask_values[163];
    let column20_row2 = mask_values[164];
    let column20_row4 = mask_values[165];
    let column20_row6 = mask_values[166];
    let column20_row8 = mask_values[167];
    let column20_row10 = mask_values[168];
    let column20_row12 = mask_values[169];
    let column20_row14 = mask_values[170];
    let column20_row16 = mask_values[171];
    let column20_row17 = mask_values[172];
    let column20_row20 = mask_values[173];
    let column20_row22 = mask_values[174];
    let column20_row24 = mask_values[175];
    let column20_row30 = mask_values[176];
    let column20_row38 = mask_values[177];
    let column20_row46 = mask_values[178];
    let column20_row54 = mask_values[179];
    let column20_row81 = mask_values[180];
    let column20_row145 = mask_values[181];
    let column20_row209 = mask_values[182];
    let column20_row4080 = mask_values[183];
    let column20_row4082 = mask_values[184];
    let column20_row4088 = mask_values[185];
    let column20_row4090 = mask_values[186];
    let column20_row4092 = mask_values[187];
    let column20_row8161 = mask_values[188];
    let column20_row8166 = mask_values[189];
    let column20_row8176 = mask_values[190];
    let column20_row8178 = mask_values[191];
    let column20_row8182 = mask_values[192];
    let column20_row8184 = mask_values[193];
    let column20_row8186 = mask_values[194];
    let column20_row8190 = mask_values[195];
    let column21_inter1_row0 = mask_values[196];
    let column21_inter1_row1 = mask_values[197];
    let column21_inter1_row2 = mask_values[198];
    let column21_inter1_row5 = mask_values[199];

    // Compute intermediate values.
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
    let npc_reg_0 = column17_row0 + cpu_decode_opcode_range_check_bit_2 + 1;
    let cpu_decode_opcode_range_check_bit_10 = column0_row10 - (column0_row11 + column0_row11);
    let cpu_decode_opcode_range_check_bit_11 = column0_row11 - (column0_row12 + column0_row12);
    let cpu_decode_opcode_range_check_bit_14 = column0_row14 - (column0_row15 + column0_row15);
    let memory_address_diff_0 = column18_row2 - column18_row0;
    let range_check16_diff_0 = column19_row6 - column19_row2;
    let pedersen_hash0_ec_subset_sum_bit_0 = column3_row0 - (column3_row1 + column3_row1);
    let pedersen_hash0_ec_subset_sum_bit_neg_0 = Felt::ONE - pedersen_hash0_ec_subset_sum_bit_0;
    let pedersen_hash1_ec_subset_sum_bit_0 = column6_row0 - (column6_row1 + column6_row1);
    let pedersen_hash1_ec_subset_sum_bit_neg_0 = Felt::ONE - pedersen_hash1_ec_subset_sum_bit_0;
    let pedersen_hash2_ec_subset_sum_bit_0 = column9_row0 - (column9_row1 + column9_row1);
    let pedersen_hash2_ec_subset_sum_bit_neg_0 = Felt::ONE - pedersen_hash2_ec_subset_sum_bit_0;
    let pedersen_hash3_ec_subset_sum_bit_0 = column12_row0 - (column12_row1 + column12_row1);
    let pedersen_hash3_ec_subset_sum_bit_neg_0 = Felt::ONE - pedersen_hash3_ec_subset_sum_bit_0;
    let range_check_builtin_value0_0 = column19_row12;
    let range_check_builtin_value1_0 =
        range_check_builtin_value0_0 * global_values.offset_size + column19_row28;
    let range_check_builtin_value2_0 =
        range_check_builtin_value1_0 * global_values.offset_size + column19_row44;
    let range_check_builtin_value3_0 =
        range_check_builtin_value2_0 * global_values.offset_size + column19_row60;
    let range_check_builtin_value4_0 =
        range_check_builtin_value3_0 * global_values.offset_size + column19_row76;
    let range_check_builtin_value5_0 =
        range_check_builtin_value4_0 * global_values.offset_size + column19_row92;
    let range_check_builtin_value6_0 =
        range_check_builtin_value5_0 * global_values.offset_size + column19_row108;
    let range_check_builtin_value7_0 =
        range_check_builtin_value6_0 * global_values.offset_size + column19_row124;
    let ecdsa_signature0_doubling_key_x_squared = column19_row7 * column19_row7;
    let ecdsa_signature0_exponentiate_generator_bit_0 =
        column20_row14 - (column20_row46 + column20_row46);
    let ecdsa_signature0_exponentiate_generator_bit_neg_0 =
        Felt::ONE - ecdsa_signature0_exponentiate_generator_bit_0;
    let ecdsa_signature0_exponentiate_key_bit_0 = column20_row4 - (column20_row20 + column20_row20);
    let ecdsa_signature0_exponentiate_key_bit_neg_0 =
        Felt::ONE - ecdsa_signature0_exponentiate_key_bit_0;

    // Sum constraints.
    let mut total_sum = Felt::ZERO;

    // Constraint: cpu/decode/opcode_range_check/bit.
    let mut value = (cpu_decode_opcode_range_check_bit_0 * cpu_decode_opcode_range_check_bit_0
        - cpu_decode_opcode_range_check_bit_0)
        * domain4.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[0] * value;

    // Constraint: cpu/decode/opcode_range_check/zero.
    value = (column0_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain4));
    total_sum += constraint_coefficients[1] * value;

    // Constraint: cpu/decode/opcode_range_check_input.
    value = (column17_row1
        - (((column0_row0 * global_values.offset_size + column19_row4)
            * global_values.offset_size
            + column19_row8)
            * global_values.offset_size
            + column19_row0))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[2] * value;

    // Constraint: cpu/decode/flag_op1_base_op0_bit.
    value = (cpu_decode_flag_op1_base_op0_0 * cpu_decode_flag_op1_base_op0_0
        - cpu_decode_flag_op1_base_op0_0)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[3] * value;

    // Constraint: cpu/decode/flag_res_op1_bit.
    value = (cpu_decode_flag_res_op1_0 * cpu_decode_flag_res_op1_0 - cpu_decode_flag_res_op1_0)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[4] * value;

    // Constraint: cpu/decode/flag_pc_update_regular_bit.
    value = (cpu_decode_flag_pc_update_regular_0 * cpu_decode_flag_pc_update_regular_0
        - cpu_decode_flag_pc_update_regular_0)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[5] * value;

    // Constraint: cpu/decode/fp_update_regular_bit.
    value = (cpu_decode_fp_update_regular_0 * cpu_decode_fp_update_regular_0
        - cpu_decode_fp_update_regular_0)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[6] * value;

    // Constraint: cpu/operands/mem_dst_addr.
    value = (column17_row8 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_0 * column19_row9
            + (Felt::ONE - cpu_decode_opcode_range_check_bit_0) * column19_row1
            + column19_row0))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[7] * value;

    // Constraint: cpu/operands/mem0_addr.
    value = (column17_row4 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_1 * column19_row9
            + (Felt::ONE - cpu_decode_opcode_range_check_bit_1) * column19_row1
            + column19_row8))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[8] * value;

    // Constraint: cpu/operands/mem1_addr.
    value = (column17_row12 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_2 * column17_row0
            + cpu_decode_opcode_range_check_bit_4 * column19_row1
            + cpu_decode_opcode_range_check_bit_3 * column19_row9
            + cpu_decode_flag_op1_base_op0_0 * column17_row5
            + column19_row4))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[9] * value;

    // Constraint: cpu/operands/ops_mul.
    value = (column19_row5 - column17_row5 * column17_row13)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[10] * value;

    // Constraint: cpu/operands/res.
    value = ((Felt::ONE - cpu_decode_opcode_range_check_bit_9) * column19_row13
        - (cpu_decode_opcode_range_check_bit_5 * (column17_row5 + column17_row13)
            + cpu_decode_opcode_range_check_bit_6 * column19_row5
            + cpu_decode_flag_res_op1_0 * column17_row13))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[11] * value;

    // Constraint: cpu/update_registers/update_pc/tmp0.
    value = (column19_row3 - cpu_decode_opcode_range_check_bit_9 * column17_row9)
        * domain19.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[12] * value;

    // Constraint: cpu/update_registers/update_pc/tmp1.
    value = (column19_row11 - column19_row3 * column19_row13)
        * domain19.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[13] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_negative.
    value = ((Felt::ONE - cpu_decode_opcode_range_check_bit_9) * column17_row16
        + column19_row3 * (column17_row16 - (column17_row0 + column17_row13))
        - (cpu_decode_flag_pc_update_regular_0 * npc_reg_0
            + cpu_decode_opcode_range_check_bit_7 * column19_row13
            + cpu_decode_opcode_range_check_bit_8 * (column17_row0 + column19_row13)))
        * domain19.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[14] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_positive.
    value = ((column19_row11 - cpu_decode_opcode_range_check_bit_9) * (column17_row16 - npc_reg_0))
        * domain19.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[15] * value;

    // Constraint: cpu/update_registers/update_ap/ap_update.
    value = (column19_row17
        - (column19_row1
            + cpu_decode_opcode_range_check_bit_10 * column19_row13
            + cpu_decode_opcode_range_check_bit_11
            + cpu_decode_opcode_range_check_bit_12 * Felt::TWO))
        * domain19.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[16] * value;

    // Constraint: cpu/update_registers/update_fp/fp_update.
    value = (column19_row25
        - (cpu_decode_fp_update_regular_0 * column19_row9
            + cpu_decode_opcode_range_check_bit_13 * column17_row9
            + cpu_decode_opcode_range_check_bit_12 * (column19_row1 + 2)))
        * domain19.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[17] * value;

    // Constraint: cpu/opcodes/call/push_fp.
    value = (cpu_decode_opcode_range_check_bit_12 * (column17_row9 - column19_row9))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[18] * value;

    // Constraint: cpu/opcodes/call/push_pc.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column17_row5 - (column17_row0 + cpu_decode_opcode_range_check_bit_2 + 1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[19] * value;

    // Constraint: cpu/opcodes/call/off0.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column19_row0 - global_values.half_offset_size))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[20] * value;

    // Constraint: cpu/opcodes/call/off1.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column19_row8 - (global_values.half_offset_size + 1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[21] * value;

    // Constraint: cpu/opcodes/call/flags.
    value = (cpu_decode_opcode_range_check_bit_12
        * (cpu_decode_opcode_range_check_bit_12 + cpu_decode_opcode_range_check_bit_12 + 1 + 1
            - (cpu_decode_opcode_range_check_bit_0 + cpu_decode_opcode_range_check_bit_1 + 4)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[22] * value;

    // Constraint: cpu/opcodes/ret/off0.
    value = (cpu_decode_opcode_range_check_bit_13
        * (column19_row0 + 2 - global_values.half_offset_size))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[23] * value;

    // Constraint: cpu/opcodes/ret/off2.
    value = (cpu_decode_opcode_range_check_bit_13
        * (column19_row4 + 1 - global_values.half_offset_size))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[24] * value;

    // Constraint: cpu/opcodes/ret/flags.
    value = (cpu_decode_opcode_range_check_bit_13
        * (cpu_decode_opcode_range_check_bit_7
            + cpu_decode_opcode_range_check_bit_0
            + cpu_decode_opcode_range_check_bit_3
            + cpu_decode_flag_res_op1_0
            - 4))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[25] * value;

    // Constraint: cpu/opcodes/assert_eq/assert_eq.
    value = (cpu_decode_opcode_range_check_bit_14 * (column17_row9 - column19_row13))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[26] * value;

    // Constraint: initial_ap.
    value = (column19_row1 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[27] * value;

    // Constraint: initial_fp.
    value = (column19_row9 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[28] * value;

    // Constraint: initial_pc.
    value = (column17_row0 - global_values.initial_pc)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[29] * value;

    // Constraint: final_ap.
    value = (column19_row1 - global_values.final_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[30] * value;

    // Constraint: final_fp.
    value = (column19_row9 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[31] * value;

    // Constraint: final_pc.
    value = (column17_row0 - global_values.final_pc)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[32] * value;

    // Constraint: memory/multi_column_perm/perm/init0.
    value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column18_row0
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column18_row1))
        * column21_inter1_row0
        + column17_row0
        + global_values.memory_multi_column_perm_hash_interaction_elm0 * column17_row1
        - global_values.memory_multi_column_perm_perm_interaction_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[33] * value;

    // Constraint: memory/multi_column_perm/perm/step0.
    value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column18_row2
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column18_row3))
        * column21_inter1_row2
        - (global_values.memory_multi_column_perm_perm_interaction_elm
            - (column17_row2
                + global_values.memory_multi_column_perm_hash_interaction_elm0 * column17_row3))
            * column21_inter1_row0)
        * domain21.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[34] * value;

    // Constraint: memory/multi_column_perm/perm/last.
    value = (column21_inter1_row0 - global_values.memory_multi_column_perm_perm_public_memory_prod)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain21));
    total_sum += constraint_coefficients[35] * value;

    // Constraint: memory/diff_is_bit.
    value = (memory_address_diff_0 * memory_address_diff_0 - memory_address_diff_0)
        * domain21.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[36] * value;

    // Constraint: memory/is_func.
    value = ((memory_address_diff_0 - 1) * (column18_row1 - column18_row3))
        * domain21.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[37] * value;

    // Constraint: memory/initial_addr.
    value = (column18_row0 - 1).field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[38] * value;

    // Constraint: public_memory_addr_zero.
    value = (column17_row2).field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[39] * value;

    // Constraint: public_memory_value_zero.
    value = (column17_row3).field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[40] * value;

    // Constraint: range_check16/perm/init0.
    value = ((global_values.range_check16_perm_interaction_elm - column19_row2)
        * column21_inter1_row1
        + column19_row0
        - global_values.range_check16_perm_interaction_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[41] * value;

    // Constraint: range_check16/perm/step0.
    value = ((global_values.range_check16_perm_interaction_elm - column19_row6)
        * column21_inter1_row5
        - (global_values.range_check16_perm_interaction_elm - column19_row4)
            * column21_inter1_row1)
        * domain22.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[42] * value;

    // Constraint: range_check16/perm/last.
    value = (column21_inter1_row1 - global_values.range_check16_perm_public_memory_prod)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain22));
    total_sum += constraint_coefficients[43] * value;

    // Constraint: range_check16/diff_is_bit.
    value = (range_check16_diff_0 * range_check16_diff_0 - range_check16_diff_0)
        * domain22.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[44] * value;

    // Constraint: range_check16/minimum.
    value = (column19_row2 - global_values.range_check_min)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[45] * value;

    // Constraint: range_check16/maximum.
    value = (column19_row2 - global_values.range_check_max)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain22));
    total_sum += constraint_coefficients[46] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/last_one_is_zero.
    value = (column14_row255 * (column3_row0 - (column3_row1 + column3_row1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[47] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    value = (column14_row255
        * (column3_row1
            - Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000")
                * column3_row192))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[48] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit192.
    value = (column14_row255
        - column13_row255 * (column3_row192 - (column3_row193 + column3_row193)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[49] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    value = (column13_row255 * (column3_row193 - Felt::from(8) * column3_row196))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[50] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit196.
    value = (column13_row255
        - (column3_row251 - (column3_row252 + column3_row252))
            * (column3_row196 - (column3_row197 + column3_row197)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[51] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    value = ((column3_row251 - (column3_row252 + column3_row252))
        * (column3_row197 - Felt::from_hex_unchecked("0x40000000000000") * column3_row251))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[52] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/booleanity_test.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (pedersen_hash0_ec_subset_sum_bit_0 - 1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[53] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_extraction_end.
    value = (column3_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain10));
    total_sum += constraint_coefficients[54] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/zeros_tail.
    value = (column3_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[55] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/slope.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (column2_row0 - global_values.pedersen_points_y)
        - column13_row0 * (column1_row0 - global_values.pedersen_points_x))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[56] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/x.
    value = (column13_row0 * column13_row0
        - pedersen_hash0_ec_subset_sum_bit_0
            * (column1_row0 + global_values.pedersen_points_x + column1_row1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[57] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/y.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (column2_row0 + column2_row1)
        - column13_row0 * (column1_row0 - column1_row1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[58] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/x.
    value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column1_row1 - column1_row0))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[59] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/y.
    value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column2_row1 - column2_row0))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[60] * value;

    // Constraint: pedersen/hash0/copy_point/x.
    value = (column1_row256 - column1_row255)
        * domain11.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[61] * value;

    // Constraint: pedersen/hash0/copy_point/y.
    value = (column2_row256 - column2_row255)
        * domain11.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[62] * value;

    // Constraint: pedersen/hash0/init/x.
    value = (column1_row0 - global_values.pedersen_shift_point.x)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[63] * value;

    // Constraint: pedersen/hash0/init/y.
    value = (column2_row0 - global_values.pedersen_shift_point.y)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[64] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/last_one_is_zero.
    value = (column16_row255 * (column6_row0 - (column6_row1 + column6_row1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[65] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    value = (column16_row255
        * (column6_row1
            - Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000")
                * column6_row192))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[66] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/cumulative_bit192.
    value = (column16_row255
        - column15_row255 * (column6_row192 - (column6_row193 + column6_row193)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[67] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    value = (column15_row255 * (column6_row193 - Felt::from(8) * column6_row196))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[68] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/cumulative_bit196.
    value = (column15_row255
        - (column6_row251 - (column6_row252 + column6_row252))
            * (column6_row196 - (column6_row197 + column6_row197)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[69] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    value = ((column6_row251 - (column6_row252 + column6_row252))
        * (column6_row197 - Felt::from_hex_unchecked("0x40000000000000") * column6_row251))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[70] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/booleanity_test.
    value = (pedersen_hash1_ec_subset_sum_bit_0 * (pedersen_hash1_ec_subset_sum_bit_0 - 1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[71] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/bit_extraction_end.
    value = (column6_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain10));
    total_sum += constraint_coefficients[72] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/zeros_tail.
    value = (column6_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[73] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/add_points/slope.
    value = (pedersen_hash1_ec_subset_sum_bit_0 * (column5_row0 - global_values.pedersen_points_y)
        - column14_row0 * (column4_row0 - global_values.pedersen_points_x))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[74] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/add_points/x.
    value = (column14_row0 * column14_row0
        - pedersen_hash1_ec_subset_sum_bit_0
            * (column4_row0 + global_values.pedersen_points_x + column4_row1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[75] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/add_points/y.
    value = (pedersen_hash1_ec_subset_sum_bit_0 * (column5_row0 + column5_row1)
        - column14_row0 * (column4_row0 - column4_row1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[76] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/copy_point/x.
    value = (pedersen_hash1_ec_subset_sum_bit_neg_0 * (column4_row1 - column4_row0))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[77] * value;

    // Constraint: pedersen/hash1/ec_subset_sum/copy_point/y.
    value = (pedersen_hash1_ec_subset_sum_bit_neg_0 * (column5_row1 - column5_row0))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[78] * value;

    // Constraint: pedersen/hash1/copy_point/x.
    value = (column4_row256 - column4_row255)
        * domain11.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[79] * value;

    // Constraint: pedersen/hash1/copy_point/y.
    value = (column5_row256 - column5_row255)
        * domain11.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[80] * value;

    // Constraint: pedersen/hash1/init/x.
    value = (column4_row0 - global_values.pedersen_shift_point.x)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[81] * value;

    // Constraint: pedersen/hash1/init/y.
    value = (column5_row0 - global_values.pedersen_shift_point.y)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[82] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/last_one_is_zero.
    value = (column20_row145 * (column9_row0 - (column9_row1 + column9_row1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[83] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    value = (column20_row145
        * (column9_row1
            - Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000")
                * column9_row192))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[84] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/cumulative_bit192.
    value = (column20_row145
        - column20_row17 * (column9_row192 - (column9_row193 + column9_row193)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[85] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    value = (column20_row17 * (column9_row193 - Felt::from(8) * column9_row196))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[86] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/cumulative_bit196.
    value = (column20_row17
        - (column9_row251 - (column9_row252 + column9_row252))
            * (column9_row196 - (column9_row197 + column9_row197)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[87] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    value = ((column9_row251 - (column9_row252 + column9_row252))
        * (column9_row197 - Felt::from_hex_unchecked("0x40000000000000") * column9_row251))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[88] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/booleanity_test.
    value = (pedersen_hash2_ec_subset_sum_bit_0 * (pedersen_hash2_ec_subset_sum_bit_0 - 1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[89] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/bit_extraction_end.
    value = (column9_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain10));
    total_sum += constraint_coefficients[90] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/zeros_tail.
    value = (column9_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[91] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/add_points/slope.
    value = (pedersen_hash2_ec_subset_sum_bit_0 * (column8_row0 - global_values.pedersen_points_y)
        - column15_row0 * (column7_row0 - global_values.pedersen_points_x))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[92] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/add_points/x.
    value = (column15_row0 * column15_row0
        - pedersen_hash2_ec_subset_sum_bit_0
            * (column7_row0 + global_values.pedersen_points_x + column7_row1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[93] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/add_points/y.
    value = (pedersen_hash2_ec_subset_sum_bit_0 * (column8_row0 + column8_row1)
        - column15_row0 * (column7_row0 - column7_row1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[94] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/copy_point/x.
    value = (pedersen_hash2_ec_subset_sum_bit_neg_0 * (column7_row1 - column7_row0))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[95] * value;

    // Constraint: pedersen/hash2/ec_subset_sum/copy_point/y.
    value = (pedersen_hash2_ec_subset_sum_bit_neg_0 * (column8_row1 - column8_row0))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[96] * value;

    // Constraint: pedersen/hash2/copy_point/x.
    value = (column7_row256 - column7_row255)
        * domain11.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[97] * value;

    // Constraint: pedersen/hash2/copy_point/y.
    value = (column8_row256 - column8_row255)
        * domain11.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[98] * value;

    // Constraint: pedersen/hash2/init/x.
    value = (column7_row0 - global_values.pedersen_shift_point.x)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[99] * value;

    // Constraint: pedersen/hash2/init/y.
    value = (column8_row0 - global_values.pedersen_shift_point.y)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[100] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/last_one_is_zero.
    value = (column20_row209 * (column12_row0 - (column12_row1 + column12_row1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[101] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    value = (column20_row209
        * (column12_row1
            - Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000")
                * column12_row192))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[102] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/cumulative_bit192.
    value = (column20_row209
        - column20_row81 * (column12_row192 - (column12_row193 + column12_row193)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[103] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    value = (column20_row81 * (column12_row193 - Felt::from(8) * column12_row196))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[104] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/cumulative_bit196.
    value = (column20_row81
        - (column12_row251 - (column12_row252 + column12_row252))
            * (column12_row196 - (column12_row197 + column12_row197)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[105] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    value = ((column12_row251 - (column12_row252 + column12_row252))
        * (column12_row197 - Felt::from_hex_unchecked("0x40000000000000") * column12_row251))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[106] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/booleanity_test.
    value = (pedersen_hash3_ec_subset_sum_bit_0 * (pedersen_hash3_ec_subset_sum_bit_0 - 1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[107] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/bit_extraction_end.
    value = (column12_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain10));
    total_sum += constraint_coefficients[108] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/zeros_tail.
    value = (column12_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[109] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/add_points/slope.
    value = (pedersen_hash3_ec_subset_sum_bit_0
        * (column11_row0 - global_values.pedersen_points_y)
        - column16_row0 * (column10_row0 - global_values.pedersen_points_x))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[110] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/add_points/x.
    value = (column16_row0 * column16_row0
        - pedersen_hash3_ec_subset_sum_bit_0
            * (column10_row0 + global_values.pedersen_points_x + column10_row1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[111] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/add_points/y.
    value = (pedersen_hash3_ec_subset_sum_bit_0 * (column11_row0 + column11_row1)
        - column16_row0 * (column10_row0 - column10_row1))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[112] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/copy_point/x.
    value = (pedersen_hash3_ec_subset_sum_bit_neg_0 * (column10_row1 - column10_row0))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[113] * value;

    // Constraint: pedersen/hash3/ec_subset_sum/copy_point/y.
    value = (pedersen_hash3_ec_subset_sum_bit_neg_0 * (column11_row1 - column11_row0))
        * domain9.field_div(&NonZeroFelt::from_felt_unchecked(domain0));
    total_sum += constraint_coefficients[114] * value;

    // Constraint: pedersen/hash3/copy_point/x.
    value = (column10_row256 - column10_row255)
        * domain11.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[115] * value;

    // Constraint: pedersen/hash3/copy_point/y.
    value = (column11_row256 - column11_row255)
        * domain11.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[116] * value;

    // Constraint: pedersen/hash3/init/x.
    value = (column10_row0 - global_values.pedersen_shift_point.x)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[117] * value;

    // Constraint: pedersen/hash3/init/y.
    value = (column11_row0 - global_values.pedersen_shift_point.y)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[118] * value;

    // Constraint: pedersen/input0_value0.
    value = (column17_row7 - column3_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[119] * value;

    // Constraint: pedersen/input0_value1.
    value = (column17_row135 - column6_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[120] * value;

    // Constraint: pedersen/input0_value2.
    value = (column17_row263 - column9_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[121] * value;

    // Constraint: pedersen/input0_value3.
    value =
        (column17_row391 - column12_row0).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[122] * value;

    // Constraint: pedersen/input0_addr.
    value = (column17_row134 - (column17_row38 + 1))
        * domain23.field_div(&NonZeroFelt::from_felt_unchecked(domain7));
    total_sum += constraint_coefficients[123] * value;

    // Constraint: pedersen/init_addr.
    value = (column17_row6 - global_values.initial_pedersen_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[124] * value;

    // Constraint: pedersen/input1_value0.
    value =
        (column17_row71 - column3_row256).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[125] * value;

    // Constraint: pedersen/input1_value1.
    value =
        (column17_row199 - column6_row256).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[126] * value;

    // Constraint: pedersen/input1_value2.
    value =
        (column17_row327 - column9_row256).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[127] * value;

    // Constraint: pedersen/input1_value3.
    value =
        (column17_row455 - column12_row256).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[128] * value;

    // Constraint: pedersen/input1_addr.
    value = (column17_row70 - (column17_row6 + 1))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain7));
    total_sum += constraint_coefficients[129] * value;

    // Constraint: pedersen/output_value0.
    value =
        (column17_row39 - column1_row511).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[130] * value;

    // Constraint: pedersen/output_value1.
    value =
        (column17_row167 - column4_row511).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[131] * value;

    // Constraint: pedersen/output_value2.
    value =
        (column17_row295 - column7_row511).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[132] * value;

    // Constraint: pedersen/output_value3.
    value =
        (column17_row423 - column10_row511).field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[133] * value;

    // Constraint: pedersen/output_addr.
    value = (column17_row38 - (column17_row70 + 1))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain7));
    total_sum += constraint_coefficients[134] * value;

    // Constraint: range_check_builtin/value.
    value = (range_check_builtin_value7_0 - column17_row103)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain7));
    total_sum += constraint_coefficients[135] * value;

    // Constraint: range_check_builtin/addr_step.
    value = (column17_row230 - (column17_row102 + 1))
        * domain23.field_div(&NonZeroFelt::from_felt_unchecked(domain7));
    total_sum += constraint_coefficients[136] * value;

    // Constraint: range_check_builtin/init_addr.
    value = (column17_row102 - global_values.initial_range_check_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[137] * value;

    // Constraint: ecdsa/signature0/doubling_key/slope.
    value = (ecdsa_signature0_doubling_key_x_squared
        + ecdsa_signature0_doubling_key_x_squared
        + ecdsa_signature0_doubling_key_x_squared
        + global_values.ecdsa_sig_config.alpha
        - (column19_row15 + column19_row15) * column20_row12)
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[138] * value;

    // Constraint: ecdsa/signature0/doubling_key/x.
    value = (column20_row12 * column20_row12 - (column19_row7 + column19_row7 + column19_row23))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[139] * value;

    // Constraint: ecdsa/signature0/doubling_key/y.
    value = (column19_row15 + column19_row31 - column20_row12 * (column19_row7 - column19_row23))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[140] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/booleanity_test.
    value = (ecdsa_signature0_exponentiate_generator_bit_0
        * (ecdsa_signature0_exponentiate_generator_bit_0 - 1))
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[141] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/bit_extraction_end.
    value = (column20_row14).field_div(&NonZeroFelt::from_felt_unchecked(domain17));
    total_sum += constraint_coefficients[142] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/zeros_tail.
    value = (column20_row14).field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[143] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/add_points/slope.
    value = (ecdsa_signature0_exponentiate_generator_bit_0
        * (column20_row22 - global_values.ecdsa_generator_points_y)
        - column20_row30 * (column20_row6 - global_values.ecdsa_generator_points_x))
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[144] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/add_points/x.
    value = (column20_row30 * column20_row30
        - ecdsa_signature0_exponentiate_generator_bit_0
            * (column20_row6 + global_values.ecdsa_generator_points_x + column20_row38))
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[145] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/add_points/y.
    value = (ecdsa_signature0_exponentiate_generator_bit_0 * (column20_row22 + column20_row54)
        - column20_row30 * (column20_row6 - column20_row38))
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[146] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/add_points/x_diff_inv.
    value = (column20_row1 * (column20_row6 - global_values.ecdsa_generator_points_x) - 1)
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[147] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/copy_point/x.
    value = (ecdsa_signature0_exponentiate_generator_bit_neg_0 * (column20_row38 - column20_row6))
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[148] * value;

    // Constraint: ecdsa/signature0/exponentiate_generator/copy_point/y.
    value = (ecdsa_signature0_exponentiate_generator_bit_neg_0 * (column20_row54 - column20_row22))
        * domain16.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[149] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/booleanity_test.
    value = (ecdsa_signature0_exponentiate_key_bit_0
        * (ecdsa_signature0_exponentiate_key_bit_0 - 1))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[150] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/bit_extraction_end.
    value = (column20_row4).field_div(&NonZeroFelt::from_felt_unchecked(domain14));
    total_sum += constraint_coefficients[151] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/zeros_tail.
    value = (column20_row4).field_div(&NonZeroFelt::from_felt_unchecked(domain13));
    total_sum += constraint_coefficients[152] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/add_points/slope.
    value = (ecdsa_signature0_exponentiate_key_bit_0 * (column20_row8 - column19_row15)
        - column20_row2 * (column20_row0 - column19_row7))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[153] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/add_points/x.
    value = (column20_row2 * column20_row2
        - ecdsa_signature0_exponentiate_key_bit_0
            * (column20_row0 + column19_row7 + column20_row16))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[154] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/add_points/y.
    value = (ecdsa_signature0_exponentiate_key_bit_0 * (column20_row8 + column20_row24)
        - column20_row2 * (column20_row0 - column20_row16))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[155] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/add_points/x_diff_inv.
    value = (column20_row10 * (column20_row0 - column19_row7) - 1)
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[156] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/copy_point/x.
    value = (ecdsa_signature0_exponentiate_key_bit_neg_0 * (column20_row16 - column20_row0))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[157] * value;

    // Constraint: ecdsa/signature0/exponentiate_key/copy_point/y.
    value = (ecdsa_signature0_exponentiate_key_bit_neg_0 * (column20_row24 - column20_row8))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[158] * value;

    // Constraint: ecdsa/signature0/init_gen/x.
    value = (column20_row6 - global_values.ecdsa_sig_config.shift_point.x)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[159] * value;

    // Constraint: ecdsa/signature0/init_gen/y.
    value = (column20_row22 + global_values.ecdsa_sig_config.shift_point.y)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[160] * value;

    // Constraint: ecdsa/signature0/init_key/x.
    value = (column20_row0 - global_values.ecdsa_sig_config.shift_point.x)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[161] * value;

    // Constraint: ecdsa/signature0/init_key/y.
    value = (column20_row8 - global_values.ecdsa_sig_config.shift_point.y)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[162] * value;

    // Constraint: ecdsa/signature0/add_results/slope.
    value = (column20_row8182
        - (column20_row4088 + column20_row8190 * (column20_row8166 - column20_row4080)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[163] * value;

    // Constraint: ecdsa/signature0/add_results/x.
    value = (column20_row8190 * column20_row8190
        - (column20_row8166 + column20_row4080 + column19_row4103))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[164] * value;

    // Constraint: ecdsa/signature0/add_results/y.
    value = (column20_row8182 + column19_row4111
        - column20_row8190 * (column20_row8166 - column19_row4103))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[165] * value;

    // Constraint: ecdsa/signature0/add_results/x_diff_inv.
    value = (column20_row8161 * (column20_row8166 - column20_row4080) - 1)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[166] * value;

    // Constraint: ecdsa/signature0/extract_r/slope.
    value = (column20_row8184 + global_values.ecdsa_sig_config.shift_point.y
        - column20_row4082 * (column20_row8176 - global_values.ecdsa_sig_config.shift_point.x))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[167] * value;

    // Constraint: ecdsa/signature0/extract_r/x.
    value = (column20_row4082 * column20_row4082
        - (column20_row8176 + global_values.ecdsa_sig_config.shift_point.x + column20_row4))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[168] * value;

    // Constraint: ecdsa/signature0/extract_r/x_diff_inv.
    value = (column20_row8178 * (column20_row8176 - global_values.ecdsa_sig_config.shift_point.x)
        - 1)
    .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[169] * value;

    // Constraint: ecdsa/signature0/z_nonzero.
    value = (column20_row14 * column20_row4090 - 1)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[170] * value;

    // Constraint: ecdsa/signature0/r_and_w_nonzero.
    value = (column20_row4 * column20_row4092 - 1)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain15));
    total_sum += constraint_coefficients[171] * value;

    // Constraint: ecdsa/signature0/q_on_curve/x_squared.
    value = (column20_row8186 - column19_row7 * column19_row7)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[172] * value;

    // Constraint: ecdsa/signature0/q_on_curve/on_curve.
    value = (column19_row15 * column19_row15
        - (column19_row7 * column20_row8186
            + global_values.ecdsa_sig_config.alpha * column19_row7
            + global_values.ecdsa_sig_config.beta))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[173] * value;

    // Constraint: ecdsa/init_addr.
    value = (column17_row22 - global_values.initial_ecdsa_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[174] * value;

    // Constraint: ecdsa/message_addr.
    value = (column17_row4118 - (column17_row22 + 1))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[175] * value;

    // Constraint: ecdsa/pubkey_addr.
    value = (column17_row8214 - (column17_row4118 + 1))
        * domain24.field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[176] * value;

    // Constraint: ecdsa/message_value0.
    value =
        (column17_row4119 - column20_row14).field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[177] * value;

    // Constraint: ecdsa/pubkey_value0.
    value = (column17_row23 - column19_row7).field_div(&NonZeroFelt::from_felt_unchecked(domain18));
    total_sum += constraint_coefficients[178] * value;

    total_sum
}

pub fn eval_oods_polynomial_inner(
    column_values: &[Felt],
    oods_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    oods_point: &Felt,
    trace_generator: &Felt,
) -> Felt {
    // Compute powers.
    let pow0 = trace_generator.pow(0_u128);
    let pow1 = trace_generator.pow(8161_u128);
    let pow2 = trace_generator.pow(4080_u128);
    let pow3 = trace_generator.pow(1_u128);
    let pow4 = pow3 * pow3; // pow(trace_generator, 2).
    let pow5 = pow2 * pow4; // pow(trace_generator, 4082).
    let pow6 = pow3 * pow4; // pow(trace_generator, 3).
    let pow7 = pow3 * pow6; // pow(trace_generator, 4).
    let pow8 = pow3 * pow7; // pow(trace_generator, 5).
    let pow9 = pow1 * pow8; // pow(trace_generator, 8166).
    let pow10 = pow3 * pow8; // pow(trace_generator, 6).
    let pow11 = pow3 * pow10; // pow(trace_generator, 7).
    let pow12 = pow3 * pow11; // pow(trace_generator, 8).
    let pow13 = pow2 * pow12; // pow(trace_generator, 4088).
    let pow14 = pow3 * pow12; // pow(trace_generator, 9).
    let pow15 = pow3 * pow14; // pow(trace_generator, 10).
    let pow16 = pow2 * pow15; // pow(trace_generator, 4090).
    let pow17 = pow3 * pow15; // pow(trace_generator, 11).
    let pow18 = pow3 * pow17; // pow(trace_generator, 12).
    let pow19 = pow3 * pow18; // pow(trace_generator, 13).
    let pow20 = pow3 * pow19; // pow(trace_generator, 14).
    let pow21 = pow3 * pow20; // pow(trace_generator, 15).
    let pow22 = pow3 * pow21; // pow(trace_generator, 16).
    let pow23 = pow3 * pow22; // pow(trace_generator, 17).
    let pow24 = pow6 * pow23; // pow(trace_generator, 20).
    let pow25 = pow4 * pow24; // pow(trace_generator, 22).
    let pow26 = pow3 * pow25; // pow(trace_generator, 23).
    let pow27 = pow3 * pow26; // pow(trace_generator, 24).
    let pow28 = pow3 * pow27; // pow(trace_generator, 25).
    let pow29 = pow6 * pow28; // pow(trace_generator, 28).
    let pow30 = pow4 * pow29; // pow(trace_generator, 30).
    let pow31 = pow3 * pow30; // pow(trace_generator, 31).
    let pow32 = pow1 * pow21; // pow(trace_generator, 8176).
    let pow33 = pow1 * pow23; // pow(trace_generator, 8178).
    let pow34 = pow11 * pow31; // pow(trace_generator, 38).
    let pow35 = pow3 * pow34; // pow(trace_generator, 39).
    let pow36 = pow8 * pow35; // pow(trace_generator, 44).
    let pow37 = pow4 * pow36; // pow(trace_generator, 46).
    let pow38 = pow12 * pow37; // pow(trace_generator, 54).
    let pow39 = pow10 * pow38; // pow(trace_generator, 60).
    let pow40 = pow15 * pow39; // pow(trace_generator, 70).
    let pow41 = pow3 * pow40; // pow(trace_generator, 71).
    let pow42 = pow8 * pow41; // pow(trace_generator, 76).
    let pow43 = pow8 * pow42; // pow(trace_generator, 81).
    let pow44 = pow17 * pow43; // pow(trace_generator, 92).
    let pow45 = pow15 * pow44; // pow(trace_generator, 102).
    let pow46 = pow3 * pow45; // pow(trace_generator, 103).
    let pow47 = pow8 * pow46; // pow(trace_generator, 108).
    let pow48 = pow22 * pow47; // pow(trace_generator, 124).
    let pow49 = pow15 * pow48; // pow(trace_generator, 134).
    let pow50 = pow3 * pow49; // pow(trace_generator, 135).
    let pow51 = pow15 * pow50; // pow(trace_generator, 145).
    let pow52 = pow25 * pow51; // pow(trace_generator, 167).
    let pow53 = pow28 * pow52; // pow(trace_generator, 192).
    let pow54 = pow3 * pow53; // pow(trace_generator, 193).
    let pow55 = pow6 * pow54; // pow(trace_generator, 196).
    let pow56 = pow3 * pow55; // pow(trace_generator, 197).
    let pow57 = pow38 * pow56; // pow(trace_generator, 251).
    let pow58 = pow4 * pow56; // pow(trace_generator, 199).
    let pow59 = pow31 * pow58; // pow(trace_generator, 230).
    let pow60 = pow3 * pow57; // pow(trace_generator, 252).
    let pow61 = pow2 * pow18; // pow(trace_generator, 4092).
    let pow62 = pow7 * pow33; // pow(trace_generator, 8182).
    let pow63 = pow1 * pow26; // pow(trace_generator, 8184).
    let pow64 = pow1 * pow28; // pow(trace_generator, 8186).
    let pow65 = pow7 * pow64; // pow(trace_generator, 8190).
    let pow66 = pow2 * pow26; // pow(trace_generator, 4103).
    let pow67 = pow2 * pow31; // pow(trace_generator, 4111).
    let pow68 = pow27 * pow65; // pow(trace_generator, 8214).
    let pow69 = pow2 * pow34; // pow(trace_generator, 4118).
    let pow70 = pow2 * pow35; // pow(trace_generator, 4119).
    let pow71 = pow15 * pow58; // pow(trace_generator, 209).
    let pow72 = pow6 * pow60; // pow(trace_generator, 255).
    let pow73 = pow3 * pow72; // pow(trace_generator, 256).
    let pow74 = pow72 * pow73; // pow(trace_generator, 511).
    let pow75 = pow52 * pow73; // pow(trace_generator, 423).
    let pow76 = pow50 * pow73; // pow(trace_generator, 391).
    let pow77 = pow41 * pow73; // pow(trace_generator, 327).
    let pow78 = pow35 * pow73; // pow(trace_generator, 295).
    let pow79 = pow11 * pow73; // pow(trace_generator, 263).
    let pow80 = pow53 * pow79; // pow(trace_generator, 455).

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
    let column10 = column_values[10];
    let column11 = column_values[11];
    let column12 = column_values[12];
    let column13 = column_values[13];
    let column14 = column_values[14];
    let column15 = column_values[15];
    let column16 = column_values[16];
    let column17 = column_values[17];
    let column18 = column_values[18];
    let column19 = column_values[19];
    let column20 = column_values[20];
    let column21 = column_values[21];

    // Sum the OODS constraints on the trace polynomials.
    let mut value: Felt;
    let mut total_sum = Felt::ZERO;

    value = (column0 - oods_values[0])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[0] * value;

    value = (column0 - oods_values[1])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[1] * value;

    value = (column0 - oods_values[2])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[2] * value;

    value = (column0 - oods_values[3])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[3] * value;

    value = (column0 - oods_values[4])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[4] * value;

    value = (column0 - oods_values[5])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[5] * value;

    value = (column0 - oods_values[6])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow10 * oods_point));
    total_sum += constraint_coefficients[6] * value;

    value = (column0 - oods_values[7])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[7] * value;

    value = (column0 - oods_values[8])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow12 * oods_point));
    total_sum += constraint_coefficients[8] * value;

    value = (column0 - oods_values[9])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[9] * value;

    value = (column0 - oods_values[10])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow15 * oods_point));
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
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[17] * value;

    value = (column1 - oods_values[18])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[18] * value;

    value = (column1 - oods_values[19])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[19] * value;

    value = (column1 - oods_values[20])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow74 * oods_point));
    total_sum += constraint_coefficients[20] * value;

    value = (column2 - oods_values[21])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[21] * value;

    value = (column2 - oods_values[22])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[22] * value;

    value = (column2 - oods_values[23])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[23] * value;

    value = (column2 - oods_values[24])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[24] * value;

    value = (column3 - oods_values[25])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[25] * value;

    value = (column3 - oods_values[26])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[26] * value;

    value = (column3 - oods_values[27])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow53 * oods_point));
    total_sum += constraint_coefficients[27] * value;

    value = (column3 - oods_values[28])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow54 * oods_point));
    total_sum += constraint_coefficients[28] * value;

    value = (column3 - oods_values[29])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow55 * oods_point));
    total_sum += constraint_coefficients[29] * value;

    value = (column3 - oods_values[30])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow56 * oods_point));
    total_sum += constraint_coefficients[30] * value;

    value = (column3 - oods_values[31])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow57 * oods_point));
    total_sum += constraint_coefficients[31] * value;

    value = (column3 - oods_values[32])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow60 * oods_point));
    total_sum += constraint_coefficients[32] * value;

    value = (column3 - oods_values[33])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[33] * value;

    value = (column4 - oods_values[34])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[34] * value;

    value = (column4 - oods_values[35])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[35] * value;

    value = (column4 - oods_values[36])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[36] * value;

    value = (column4 - oods_values[37])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[37] * value;

    value = (column4 - oods_values[38])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow74 * oods_point));
    total_sum += constraint_coefficients[38] * value;

    value = (column5 - oods_values[39])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[39] * value;

    value = (column5 - oods_values[40])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[40] * value;

    value = (column5 - oods_values[41])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[41] * value;

    value = (column5 - oods_values[42])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[42] * value;

    value = (column6 - oods_values[43])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[43] * value;

    value = (column6 - oods_values[44])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[44] * value;

    value = (column6 - oods_values[45])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow53 * oods_point));
    total_sum += constraint_coefficients[45] * value;

    value = (column6 - oods_values[46])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow54 * oods_point));
    total_sum += constraint_coefficients[46] * value;

    value = (column6 - oods_values[47])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow55 * oods_point));
    total_sum += constraint_coefficients[47] * value;

    value = (column6 - oods_values[48])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow56 * oods_point));
    total_sum += constraint_coefficients[48] * value;

    value = (column6 - oods_values[49])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow57 * oods_point));
    total_sum += constraint_coefficients[49] * value;

    value = (column6 - oods_values[50])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow60 * oods_point));
    total_sum += constraint_coefficients[50] * value;

    value = (column6 - oods_values[51])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[51] * value;

    value = (column7 - oods_values[52])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[52] * value;

    value = (column7 - oods_values[53])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[53] * value;

    value = (column7 - oods_values[54])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[54] * value;

    value = (column7 - oods_values[55])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[55] * value;

    value = (column7 - oods_values[56])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow74 * oods_point));
    total_sum += constraint_coefficients[56] * value;

    value = (column8 - oods_values[57])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[57] * value;

    value = (column8 - oods_values[58])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[58] * value;

    value = (column8 - oods_values[59])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[59] * value;

    value = (column8 - oods_values[60])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[60] * value;

    value = (column9 - oods_values[61])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[61] * value;

    value = (column9 - oods_values[62])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[62] * value;

    value = (column9 - oods_values[63])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow53 * oods_point));
    total_sum += constraint_coefficients[63] * value;

    value = (column9 - oods_values[64])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow54 * oods_point));
    total_sum += constraint_coefficients[64] * value;

    value = (column9 - oods_values[65])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow55 * oods_point));
    total_sum += constraint_coefficients[65] * value;

    value = (column9 - oods_values[66])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow56 * oods_point));
    total_sum += constraint_coefficients[66] * value;

    value = (column9 - oods_values[67])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow57 * oods_point));
    total_sum += constraint_coefficients[67] * value;

    value = (column9 - oods_values[68])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow60 * oods_point));
    total_sum += constraint_coefficients[68] * value;

    value = (column9 - oods_values[69])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[69] * value;

    value = (column10 - oods_values[70])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[70] * value;

    value = (column10 - oods_values[71])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[71] * value;

    value = (column10 - oods_values[72])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[72] * value;

    value = (column10 - oods_values[73])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[73] * value;

    value = (column10 - oods_values[74])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow74 * oods_point));
    total_sum += constraint_coefficients[74] * value;

    value = (column11 - oods_values[75])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[75] * value;

    value = (column11 - oods_values[76])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[76] * value;

    value = (column11 - oods_values[77])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[77] * value;

    value = (column11 - oods_values[78])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[78] * value;

    value = (column12 - oods_values[79])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[79] * value;

    value = (column12 - oods_values[80])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[80] * value;

    value = (column12 - oods_values[81])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow53 * oods_point));
    total_sum += constraint_coefficients[81] * value;

    value = (column12 - oods_values[82])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow54 * oods_point));
    total_sum += constraint_coefficients[82] * value;

    value = (column12 - oods_values[83])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow55 * oods_point));
    total_sum += constraint_coefficients[83] * value;

    value = (column12 - oods_values[84])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow56 * oods_point));
    total_sum += constraint_coefficients[84] * value;

    value = (column12 - oods_values[85])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow57 * oods_point));
    total_sum += constraint_coefficients[85] * value;

    value = (column12 - oods_values[86])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow60 * oods_point));
    total_sum += constraint_coefficients[86] * value;

    value = (column12 - oods_values[87])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[87] * value;

    value = (column13 - oods_values[88])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[88] * value;

    value = (column13 - oods_values[89])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[89] * value;

    value = (column14 - oods_values[90])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[90] * value;

    value = (column14 - oods_values[91])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[91] * value;

    value = (column15 - oods_values[92])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[92] * value;

    value = (column15 - oods_values[93])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[93] * value;

    value = (column16 - oods_values[94])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[94] * value;

    value = (column16 - oods_values[95])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[95] * value;

    value = (column17 - oods_values[96])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[96] * value;

    value = (column17 - oods_values[97])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[97] * value;

    value = (column17 - oods_values[98])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[98] * value;

    value = (column17 - oods_values[99])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[99] * value;

    value = (column17 - oods_values[100])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[100] * value;

    value = (column17 - oods_values[101])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[101] * value;

    value = (column17 - oods_values[102])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow10 * oods_point));
    total_sum += constraint_coefficients[102] * value;

    value = (column17 - oods_values[103])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[103] * value;

    value = (column17 - oods_values[104])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow12 * oods_point));
    total_sum += constraint_coefficients[104] * value;

    value = (column17 - oods_values[105])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[105] * value;

    value = (column17 - oods_values[106])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[106] * value;

    value = (column17 - oods_values[107])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow19 * oods_point));
    total_sum += constraint_coefficients[107] * value;

    value = (column17 - oods_values[108])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow22 * oods_point));
    total_sum += constraint_coefficients[108] * value;

    value = (column17 - oods_values[109])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow25 * oods_point));
    total_sum += constraint_coefficients[109] * value;

    value = (column17 - oods_values[110])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow26 * oods_point));
    total_sum += constraint_coefficients[110] * value;

    value = (column17 - oods_values[111])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow34 * oods_point));
    total_sum += constraint_coefficients[111] * value;

    value = (column17 - oods_values[112])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow35 * oods_point));
    total_sum += constraint_coefficients[112] * value;

    value = (column17 - oods_values[113])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow40 * oods_point));
    total_sum += constraint_coefficients[113] * value;

    value = (column17 - oods_values[114])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow41 * oods_point));
    total_sum += constraint_coefficients[114] * value;

    value = (column17 - oods_values[115])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow45 * oods_point));
    total_sum += constraint_coefficients[115] * value;

    value = (column17 - oods_values[116])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow46 * oods_point));
    total_sum += constraint_coefficients[116] * value;

    value = (column17 - oods_values[117])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow49 * oods_point));
    total_sum += constraint_coefficients[117] * value;

    value = (column17 - oods_values[118])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow50 * oods_point));
    total_sum += constraint_coefficients[118] * value;

    value = (column17 - oods_values[119])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow52 * oods_point));
    total_sum += constraint_coefficients[119] * value;

    value = (column17 - oods_values[120])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow58 * oods_point));
    total_sum += constraint_coefficients[120] * value;

    value = (column17 - oods_values[121])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow59 * oods_point));
    total_sum += constraint_coefficients[121] * value;

    value = (column17 - oods_values[122])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow79 * oods_point));
    total_sum += constraint_coefficients[122] * value;

    value = (column17 - oods_values[123])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow78 * oods_point));
    total_sum += constraint_coefficients[123] * value;

    value = (column17 - oods_values[124])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow77 * oods_point));
    total_sum += constraint_coefficients[124] * value;

    value = (column17 - oods_values[125])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow76 * oods_point));
    total_sum += constraint_coefficients[125] * value;

    value = (column17 - oods_values[126])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow75 * oods_point));
    total_sum += constraint_coefficients[126] * value;

    value = (column17 - oods_values[127])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow80 * oods_point));
    total_sum += constraint_coefficients[127] * value;

    value = (column17 - oods_values[128])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow69 * oods_point));
    total_sum += constraint_coefficients[128] * value;

    value = (column17 - oods_values[129])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow70 * oods_point));
    total_sum += constraint_coefficients[129] * value;

    value = (column17 - oods_values[130])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow68 * oods_point));
    total_sum += constraint_coefficients[130] * value;

    value = (column18 - oods_values[131])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[131] * value;

    value = (column18 - oods_values[132])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[132] * value;

    value = (column18 - oods_values[133])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[133] * value;

    value = (column18 - oods_values[134])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[134] * value;

    value = (column19 - oods_values[135])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[135] * value;

    value = (column19 - oods_values[136])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[136] * value;

    value = (column19 - oods_values[137])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[137] * value;

    value = (column19 - oods_values[138])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[138] * value;

    value = (column19 - oods_values[139])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[139] * value;

    value = (column19 - oods_values[140])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[140] * value;

    value = (column19 - oods_values[141])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow10 * oods_point));
    total_sum += constraint_coefficients[141] * value;

    value = (column19 - oods_values[142])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[142] * value;

    value = (column19 - oods_values[143])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow12 * oods_point));
    total_sum += constraint_coefficients[143] * value;

    value = (column19 - oods_values[144])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[144] * value;

    value = (column19 - oods_values[145])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow17 * oods_point));
    total_sum += constraint_coefficients[145] * value;

    value = (column19 - oods_values[146])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[146] * value;

    value = (column19 - oods_values[147])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow19 * oods_point));
    total_sum += constraint_coefficients[147] * value;

    value = (column19 - oods_values[148])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow21 * oods_point));
    total_sum += constraint_coefficients[148] * value;

    value = (column19 - oods_values[149])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow23 * oods_point));
    total_sum += constraint_coefficients[149] * value;

    value = (column19 - oods_values[150])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow26 * oods_point));
    total_sum += constraint_coefficients[150] * value;

    value = (column19 - oods_values[151])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow28 * oods_point));
    total_sum += constraint_coefficients[151] * value;

    value = (column19 - oods_values[152])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow29 * oods_point));
    total_sum += constraint_coefficients[152] * value;

    value = (column19 - oods_values[153])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow31 * oods_point));
    total_sum += constraint_coefficients[153] * value;

    value = (column19 - oods_values[154])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow36 * oods_point));
    total_sum += constraint_coefficients[154] * value;

    value = (column19 - oods_values[155])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow39 * oods_point));
    total_sum += constraint_coefficients[155] * value;

    value = (column19 - oods_values[156])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow42 * oods_point));
    total_sum += constraint_coefficients[156] * value;

    value = (column19 - oods_values[157])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow44 * oods_point));
    total_sum += constraint_coefficients[157] * value;

    value = (column19 - oods_values[158])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow47 * oods_point));
    total_sum += constraint_coefficients[158] * value;

    value = (column19 - oods_values[159])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow48 * oods_point));
    total_sum += constraint_coefficients[159] * value;

    value = (column19 - oods_values[160])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow66 * oods_point));
    total_sum += constraint_coefficients[160] * value;

    value = (column19 - oods_values[161])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow67 * oods_point));
    total_sum += constraint_coefficients[161] * value;

    value = (column20 - oods_values[162])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[162] * value;

    value = (column20 - oods_values[163])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[163] * value;

    value = (column20 - oods_values[164])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[164] * value;

    value = (column20 - oods_values[165])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[165] * value;

    value = (column20 - oods_values[166])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow10 * oods_point));
    total_sum += constraint_coefficients[166] * value;

    value = (column20 - oods_values[167])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow12 * oods_point));
    total_sum += constraint_coefficients[167] * value;

    value = (column20 - oods_values[168])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow15 * oods_point));
    total_sum += constraint_coefficients[168] * value;

    value = (column20 - oods_values[169])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[169] * value;

    value = (column20 - oods_values[170])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow20 * oods_point));
    total_sum += constraint_coefficients[170] * value;

    value = (column20 - oods_values[171])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow22 * oods_point));
    total_sum += constraint_coefficients[171] * value;

    value = (column20 - oods_values[172])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow23 * oods_point));
    total_sum += constraint_coefficients[172] * value;

    value = (column20 - oods_values[173])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow24 * oods_point));
    total_sum += constraint_coefficients[173] * value;

    value = (column20 - oods_values[174])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow25 * oods_point));
    total_sum += constraint_coefficients[174] * value;

    value = (column20 - oods_values[175])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow27 * oods_point));
    total_sum += constraint_coefficients[175] * value;

    value = (column20 - oods_values[176])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow30 * oods_point));
    total_sum += constraint_coefficients[176] * value;

    value = (column20 - oods_values[177])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow34 * oods_point));
    total_sum += constraint_coefficients[177] * value;

    value = (column20 - oods_values[178])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow37 * oods_point));
    total_sum += constraint_coefficients[178] * value;

    value = (column20 - oods_values[179])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow38 * oods_point));
    total_sum += constraint_coefficients[179] * value;

    value = (column20 - oods_values[180])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow43 * oods_point));
    total_sum += constraint_coefficients[180] * value;

    value = (column20 - oods_values[181])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow51 * oods_point));
    total_sum += constraint_coefficients[181] * value;

    value = (column20 - oods_values[182])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow71 * oods_point));
    total_sum += constraint_coefficients[182] * value;

    value = (column20 - oods_values[183])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow2 * oods_point));
    total_sum += constraint_coefficients[183] * value;

    value = (column20 - oods_values[184])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[184] * value;

    value = (column20 - oods_values[185])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow13 * oods_point));
    total_sum += constraint_coefficients[185] * value;

    value = (column20 - oods_values[186])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[186] * value;

    value = (column20 - oods_values[187])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow61 * oods_point));
    total_sum += constraint_coefficients[187] * value;

    value = (column20 - oods_values[188])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow1 * oods_point));
    total_sum += constraint_coefficients[188] * value;

    value = (column20 - oods_values[189])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow9 * oods_point));
    total_sum += constraint_coefficients[189] * value;

    value = (column20 - oods_values[190])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow32 * oods_point));
    total_sum += constraint_coefficients[190] * value;

    value = (column20 - oods_values[191])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow33 * oods_point));
    total_sum += constraint_coefficients[191] * value;

    value = (column20 - oods_values[192])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow62 * oods_point));
    total_sum += constraint_coefficients[192] * value;

    value = (column20 - oods_values[193])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow63 * oods_point));
    total_sum += constraint_coefficients[193] * value;

    value = (column20 - oods_values[194])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow64 * oods_point));
    total_sum += constraint_coefficients[194] * value;

    value = (column20 - oods_values[195])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow65 * oods_point));
    total_sum += constraint_coefficients[195] * value;

    value = (column21 - oods_values[196])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[196] * value;

    value = (column21 - oods_values[197])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[197] * value;

    value = (column21 - oods_values[198])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[198] * value;

    value = (column21 - oods_values[199])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[199] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow(CONSTRAINT_DEGREE);

    value = (column_values[(NUM_COLUMNS_FIRST + NUM_COLUMNS_SECOND) as usize] - oods_values[200])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - oods_point_to_deg));
    total_sum += constraint_coefficients[200] * value;

    value = (column_values[(NUM_COLUMNS_FIRST + NUM_COLUMNS_SECOND + 1) as usize]
        - oods_values[201])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - oods_point_to_deg));
    total_sum += constraint_coefficients[201] * value;

    total_sum
}
