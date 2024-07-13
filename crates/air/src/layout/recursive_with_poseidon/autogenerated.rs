use super::global_values::GlobalValues;
use crate::layout::recursive::{CONSTRAINT_DEGREE, NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
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
        &global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(4096))),
    );
    let pow1 = pow0 * pow0; // pow(point, (safe_div(global_values.trace_length, 2048))).
    let pow2 = pow1 * pow1; // pow(point, (safe_div(global_values.trace_length, 1024))).
    let pow3 = pow2 * pow2; // pow(point, (safe_div(global_values.trace_length, 512))).
    let pow4 = pow3 * pow3; // pow(point, (safe_div(global_values.trace_length, 256))).
    let pow5 = pow4 * pow4; // pow(point, (safe_div(global_values.trace_length, 128))).
    let pow6 = pow5 * pow5; // pow(point, (safe_div(global_values.trace_length, 64))).
    let pow7 = pow6 * pow6; // pow(point, (safe_div(global_values.trace_length, 32))).
    let pow8 = pow7 * pow7; // pow(point, (safe_div(global_values.trace_length, 16))).
    let pow9 = pow8 * pow8; // pow(point, (safe_div(global_values.trace_length, 8))).
    let pow10 = pow9 * pow9; // pow(point, (safe_div(global_values.trace_length, 4))).
    let pow11 = pow10 * pow10; // pow(point, (safe_div(global_values.trace_length, 2))).
    let pow12 = pow11 * pow11; // pow(point, global_values.trace_length).
    let pow13 = trace_generator.pow_felt(&(global_values.trace_length - 512));
    let pow14 = trace_generator.pow_felt(&(global_values.trace_length - 256));
    let pow15 = trace_generator.pow_felt(&(global_values.trace_length - 4096));
    let pow16 = trace_generator.pow_felt(&(global_values.trace_length - 4));
    let pow17 = trace_generator.pow_felt(&(global_values.trace_length - 2));
    let pow18 = trace_generator.pow_felt(&(global_values.trace_length - 16));
    let pow19 = trace_generator.pow_felt(
        &(global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(2)))),
    );
    let pow20 = trace_generator.pow_felt(
        &(Felt::from(255)
            * global_values
                .trace_length
                .floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(2056)))),
    );
    let pow21 = trace_generator.pow_felt(
        &(global_values.trace_length.floor_div(&NonZeroFelt::from_felt_unchecked(Felt::from(64)))),
    );
    let pow22 = pow21 * pow21; // pow(trace_generator, (safe_div(global_values.trace_length, 32))).
    let pow23 = pow21 * pow22; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 64))).
    let pow24 = pow21 * pow23; // pow(trace_generator, (safe_div(global_values.trace_length, 16))).
    let pow25 = pow21 * pow24; // pow(trace_generator, (safe_div((safe_mult(5, global_values.trace_length)), 64))).
    let pow26 = pow21 * pow25; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 32))).
    let pow27 = pow19 * pow26; // pow(trace_generator, (safe_div((safe_mult(19, global_values.trace_length)), 32))).
    let pow28 = pow21 * pow26; // pow(trace_generator, (safe_div((safe_mult(7, global_values.trace_length)), 64))).
    let pow29 = pow21 * pow28; // pow(trace_generator, (safe_div(global_values.trace_length, 8))).
    let pow30 = pow19 * pow29; // pow(trace_generator, (safe_div((safe_mult(5, global_values.trace_length)), 8))).
    let pow31 = pow21 * pow29; // pow(trace_generator, (safe_div((safe_mult(9, global_values.trace_length)), 64))).
    let pow32 = pow21 * pow31; // pow(trace_generator, (safe_div((safe_mult(5, global_values.trace_length)), 32))).
    let pow33 = pow19 * pow32; // pow(trace_generator, (safe_div((safe_mult(21, global_values.trace_length)), 32))).
    let pow34 = pow21 * pow32; // pow(trace_generator, (safe_div((safe_mult(11, global_values.trace_length)), 64))).
    let pow35 = pow21 * pow34; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 16))).
    let pow36 = pow19 * pow35; // pow(trace_generator, (safe_div((safe_mult(11, global_values.trace_length)), 16))).
    let pow37 = pow21 * pow35; // pow(trace_generator, (safe_div((safe_mult(13, global_values.trace_length)), 64))).
    let pow38 = pow21 * pow37; // pow(trace_generator, (safe_div((safe_mult(7, global_values.trace_length)), 32))).
    let pow39 = pow19 * pow38; // pow(trace_generator, (safe_div((safe_mult(23, global_values.trace_length)), 32))).
    let pow40 = pow21 * pow38; // pow(trace_generator, (safe_div((safe_mult(15, global_values.trace_length)), 64))).
    let pow41 = pow22 * pow39; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 4))).
    let pow42 = pow22 * pow41; // pow(trace_generator, (safe_div((safe_mult(25, global_values.trace_length)), 32))).
    let pow43 = pow22 * pow42; // pow(trace_generator, (safe_div((safe_mult(13, global_values.trace_length)), 16))).
    let pow44 = pow22 * pow43; // pow(trace_generator, (safe_div((safe_mult(27, global_values.trace_length)), 32))).
    let pow45 = pow22 * pow44; // pow(trace_generator, (safe_div((safe_mult(7, global_values.trace_length)), 8))).
    let pow46 = pow22 * pow45; // pow(trace_generator, (safe_div((safe_mult(29, global_values.trace_length)), 32))).
    let pow47 = pow22 * pow46; // pow(trace_generator, (safe_div((safe_mult(15, global_values.trace_length)), 16))).
    let pow48 = pow21 * pow47; // pow(trace_generator, (safe_div((safe_mult(61, global_values.trace_length)), 64))).
    let pow49 = pow21 * pow48; // pow(trace_generator, (safe_div((safe_mult(31, global_values.trace_length)), 32))).
    let pow50 = pow21 * pow49; // pow(trace_generator, (safe_div((safe_mult(63, global_values.trace_length)), 64))).

    // Compute domains.
    let domain0 = pow12 - 1;
    let domain1 = pow11 - 1;
    let domain2 = pow10 - 1;
    let domain3 = pow9 - 1;
    let domain4 = pow8 - pow47;
    let domain5 = pow8 - 1;
    let domain6 = pow7 - 1;
    let domain7 = pow6 - 1;
    let domain8 = pow5 - 1;
    let domain9 = pow4 - 1;
    let domain10 = pow4 - pow41;
    let temp = pow4 - pow21;
    let temp = temp * (pow4 - pow22);
    let temp = temp * (pow4 - pow23);
    let temp = temp * (pow4 - pow24);
    let temp = temp * (pow4 - pow25);
    let temp = temp * (pow4 - pow26);
    let temp = temp * (pow4 - pow28);
    let temp = temp * (pow4 - pow29);
    let temp = temp * (pow4 - pow31);
    let temp = temp * (pow4 - pow32);
    let temp = temp * (pow4 - pow34);
    let temp = temp * (pow4 - pow35);
    let temp = temp * (pow4 - pow37);
    let temp = temp * (pow4 - pow38);
    let temp = temp * (pow4 - pow40);
    let domain11 = temp * (domain9);
    let domain12 = pow3 - 1;
    let domain13 = pow3 - pow41;
    let domain14 = pow2 - pow49;
    let temp = pow2 - pow36;
    let temp = temp * (pow2 - pow39);
    let temp = temp * (pow2 - pow41);
    let temp = temp * (pow2 - pow42);
    let temp = temp * (pow2 - pow43);
    let temp = temp * (pow2 - pow44);
    let temp = temp * (pow2 - pow45);
    let temp = temp * (pow2 - pow46);
    let temp = temp * (pow2 - pow47);
    let domain15 = temp * (domain14);
    let domain16 = pow2 - 1;
    let temp = pow2 - pow48;
    let temp = temp * (pow2 - pow50);
    let domain17 = temp * (domain14);
    let temp = pow2 - pow27;
    let temp = temp * (pow2 - pow30);
    let temp = temp * (pow2 - pow33);
    let domain18 = temp * (domain15);
    let domain19 = pow1 - 1;
    let domain20 = pow1 - pow20;
    let domain21 = pow1 - pow50;
    let domain22 = pow0 - pow19;
    let domain23 = pow0 - 1;
    let domain24 = point - pow18;
    let domain25 = point - 1;
    let domain26 = point - pow17;
    let domain27 = point - pow16;
    let domain28 = point - pow15;
    let domain29 = point - pow14;
    let domain30 = point - pow13;

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
    let column1_row3 = mask_values[19];
    let column1_row4 = mask_values[20];
    let column1_row5 = mask_values[21];
    let column1_row8 = mask_values[22];
    let column1_row9 = mask_values[23];
    let column1_row10 = mask_values[24];
    let column1_row11 = mask_values[25];
    let column1_row12 = mask_values[26];
    let column1_row13 = mask_values[27];
    let column1_row16 = mask_values[28];
    let column1_row42 = mask_values[29];
    let column1_row43 = mask_values[30];
    let column1_row74 = mask_values[31];
    let column1_row75 = mask_values[32];
    let column1_row106 = mask_values[33];
    let column1_row138 = mask_values[34];
    let column1_row139 = mask_values[35];
    let column1_row171 = mask_values[36];
    let column1_row202 = mask_values[37];
    let column1_row203 = mask_values[38];
    let column1_row234 = mask_values[39];
    let column1_row235 = mask_values[40];
    let column1_row266 = mask_values[41];
    let column1_row267 = mask_values[42];
    let column1_row298 = mask_values[43];
    let column1_row394 = mask_values[44];
    let column1_row458 = mask_values[45];
    let column1_row459 = mask_values[46];
    let column1_row714 = mask_values[47];
    let column1_row715 = mask_values[48];
    let column1_row778 = mask_values[49];
    let column1_row779 = mask_values[50];
    let column1_row970 = mask_values[51];
    let column1_row971 = mask_values[52];
    let column1_row1034 = mask_values[53];
    let column1_row1035 = mask_values[54];
    let column1_row2058 = mask_values[55];
    let column1_row2059 = mask_values[56];
    let column1_row4106 = mask_values[57];
    let column2_row0 = mask_values[58];
    let column2_row1 = mask_values[59];
    let column2_row2 = mask_values[60];
    let column2_row3 = mask_values[61];
    let column3_row0 = mask_values[62];
    let column3_row1 = mask_values[63];
    let column3_row2 = mask_values[64];
    let column3_row3 = mask_values[65];
    let column3_row4 = mask_values[66];
    let column3_row8 = mask_values[67];
    let column3_row12 = mask_values[68];
    let column3_row16 = mask_values[69];
    let column3_row20 = mask_values[70];
    let column3_row24 = mask_values[71];
    let column3_row28 = mask_values[72];
    let column3_row32 = mask_values[73];
    let column3_row36 = mask_values[74];
    let column3_row40 = mask_values[75];
    let column3_row44 = mask_values[76];
    let column3_row48 = mask_values[77];
    let column3_row52 = mask_values[78];
    let column3_row56 = mask_values[79];
    let column3_row60 = mask_values[80];
    let column3_row64 = mask_values[81];
    let column3_row66 = mask_values[82];
    let column3_row128 = mask_values[83];
    let column3_row130 = mask_values[84];
    let column3_row176 = mask_values[85];
    let column3_row180 = mask_values[86];
    let column3_row184 = mask_values[87];
    let column3_row188 = mask_values[88];
    let column3_row192 = mask_values[89];
    let column3_row194 = mask_values[90];
    let column3_row240 = mask_values[91];
    let column3_row244 = mask_values[92];
    let column3_row248 = mask_values[93];
    let column3_row252 = mask_values[94];
    let column4_row0 = mask_values[95];
    let column4_row1 = mask_values[96];
    let column4_row2 = mask_values[97];
    let column4_row3 = mask_values[98];
    let column4_row4 = mask_values[99];
    let column4_row5 = mask_values[100];
    let column4_row6 = mask_values[101];
    let column4_row7 = mask_values[102];
    let column4_row8 = mask_values[103];
    let column4_row9 = mask_values[104];
    let column4_row11 = mask_values[105];
    let column4_row12 = mask_values[106];
    let column4_row13 = mask_values[107];
    let column4_row44 = mask_values[108];
    let column4_row76 = mask_values[109];
    let column4_row108 = mask_values[110];
    let column4_row140 = mask_values[111];
    let column4_row172 = mask_values[112];
    let column4_row204 = mask_values[113];
    let column4_row236 = mask_values[114];
    let column4_row1539 = mask_values[115];
    let column4_row1547 = mask_values[116];
    let column4_row1571 = mask_values[117];
    let column4_row1579 = mask_values[118];
    let column4_row2011 = mask_values[119];
    let column4_row2019 = mask_values[120];
    let column4_row2041 = mask_values[121];
    let column4_row2045 = mask_values[122];
    let column4_row2047 = mask_values[123];
    let column4_row2049 = mask_values[124];
    let column4_row2051 = mask_values[125];
    let column4_row2053 = mask_values[126];
    let column4_row4089 = mask_values[127];
    let column5_row0 = mask_values[128];
    let column5_row1 = mask_values[129];
    let column5_row2 = mask_values[130];
    let column5_row4 = mask_values[131];
    let column5_row6 = mask_values[132];
    let column5_row8 = mask_values[133];
    let column5_row9 = mask_values[134];
    let column5_row10 = mask_values[135];
    let column5_row12 = mask_values[136];
    let column5_row14 = mask_values[137];
    let column5_row16 = mask_values[138];
    let column5_row17 = mask_values[139];
    let column5_row22 = mask_values[140];
    let column5_row24 = mask_values[141];
    let column5_row25 = mask_values[142];
    let column5_row30 = mask_values[143];
    let column5_row33 = mask_values[144];
    let column5_row38 = mask_values[145];
    let column5_row41 = mask_values[146];
    let column5_row46 = mask_values[147];
    let column5_row49 = mask_values[148];
    let column5_row54 = mask_values[149];
    let column5_row57 = mask_values[150];
    let column5_row65 = mask_values[151];
    let column5_row73 = mask_values[152];
    let column5_row81 = mask_values[153];
    let column5_row89 = mask_values[154];
    let column5_row97 = mask_values[155];
    let column5_row105 = mask_values[156];
    let column5_row137 = mask_values[157];
    let column5_row169 = mask_values[158];
    let column5_row201 = mask_values[159];
    let column5_row393 = mask_values[160];
    let column5_row409 = mask_values[161];
    let column5_row425 = mask_values[162];
    let column5_row457 = mask_values[163];
    let column5_row473 = mask_values[164];
    let column5_row489 = mask_values[165];
    let column5_row521 = mask_values[166];
    let column5_row553 = mask_values[167];
    let column5_row585 = mask_values[168];
    let column5_row609 = mask_values[169];
    let column5_row625 = mask_values[170];
    let column5_row641 = mask_values[171];
    let column5_row657 = mask_values[172];
    let column5_row673 = mask_values[173];
    let column5_row689 = mask_values[174];
    let column5_row905 = mask_values[175];
    let column5_row921 = mask_values[176];
    let column5_row937 = mask_values[177];
    let column5_row969 = mask_values[178];
    let column5_row982 = mask_values[179];
    let column5_row985 = mask_values[180];
    let column5_row998 = mask_values[181];
    let column5_row1001 = mask_values[182];
    let column5_row1014 = mask_values[183];
    let column6_inter1_row0 = mask_values[184];
    let column6_inter1_row1 = mask_values[185];
    let column6_inter1_row2 = mask_values[186];
    let column6_inter1_row3 = mask_values[187];
    let column7_inter1_row0 = mask_values[188];
    let column7_inter1_row1 = mask_values[189];
    let column7_inter1_row2 = mask_values[190];
    let column7_inter1_row5 = mask_values[191];

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
    let npc_reg_0 = column1_row0 + cpu_decode_opcode_range_check_bit_2 + 1;
    let cpu_decode_opcode_range_check_bit_10 = column0_row10 - (column0_row11 + column0_row11);
    let cpu_decode_opcode_range_check_bit_11 = column0_row11 - (column0_row12 + column0_row12);
    let cpu_decode_opcode_range_check_bit_14 = column0_row14 - (column0_row15 + column0_row15);
    let memory_address_diff_0 = column2_row2 - column2_row0;
    let range_check16_diff_0 = column4_row6 - column4_row2;
    let pedersen_hash0_ec_subset_sum_bit_0 = column4_row3 - (column4_row11 + column4_row11);
    let pedersen_hash0_ec_subset_sum_bit_neg_0 = Felt::ONE - pedersen_hash0_ec_subset_sum_bit_0;
    let range_check_builtin_value0_0 = column4_row12;
    let range_check_builtin_value1_0 =
        range_check_builtin_value0_0 * global_values.offset_size + column4_row44;
    let range_check_builtin_value2_0 =
        range_check_builtin_value1_0 * global_values.offset_size + column4_row76;
    let range_check_builtin_value3_0 =
        range_check_builtin_value2_0 * global_values.offset_size + column4_row108;
    let range_check_builtin_value4_0 =
        range_check_builtin_value3_0 * global_values.offset_size + column4_row140;
    let range_check_builtin_value5_0 =
        range_check_builtin_value4_0 * global_values.offset_size + column4_row172;
    let range_check_builtin_value6_0 =
        range_check_builtin_value5_0 * global_values.offset_size + column4_row204;
    let range_check_builtin_value7_0 =
        range_check_builtin_value6_0 * global_values.offset_size + column4_row236;
    let bitwise_sum_var_0_0 = column3_row0
        + column3_row4 * Felt::from_hex_unchecked("0x2")
        + column3_row8 * Felt::from_hex_unchecked("0x4")
        + column3_row12 * Felt::from_hex_unchecked("0x8")
        + column3_row16 * Felt::from_hex_unchecked("0x10000000000000000")
        + column3_row20 * Felt::from_hex_unchecked("0x20000000000000000")
        + column3_row24 * Felt::from_hex_unchecked("0x40000000000000000")
        + column3_row28 * Felt::from_hex_unchecked("0x80000000000000000");
    let bitwise_sum_var_8_0 = column3_row32
        * Felt::from_hex_unchecked("0x100000000000000000000000000000000")
        + column3_row36 * Felt::from_hex_unchecked("0x200000000000000000000000000000000")
        + column3_row40 * Felt::from_hex_unchecked("0x400000000000000000000000000000000")
        + column3_row44 * Felt::from_hex_unchecked("0x800000000000000000000000000000000")
        + column3_row48
            * Felt::from_hex_unchecked("0x1000000000000000000000000000000000000000000000000")
        + column3_row52
            * Felt::from_hex_unchecked("0x2000000000000000000000000000000000000000000000000")
        + column3_row56
            * Felt::from_hex_unchecked("0x4000000000000000000000000000000000000000000000000")
        + column3_row60
            * Felt::from_hex_unchecked("0x8000000000000000000000000000000000000000000000000");
    let poseidon_poseidon_full_rounds_state0_cubed_0 = column5_row9 * column5_row105;
    let poseidon_poseidon_full_rounds_state1_cubed_0 = column5_row73 * column5_row25;
    let poseidon_poseidon_full_rounds_state2_cubed_0 = column5_row41 * column5_row89;
    let poseidon_poseidon_full_rounds_state0_cubed_7 = column5_row905 * column5_row1001;
    let poseidon_poseidon_full_rounds_state1_cubed_7 = column5_row969 * column5_row921;
    let poseidon_poseidon_full_rounds_state2_cubed_7 = column5_row937 * column5_row985;
    let poseidon_poseidon_full_rounds_state0_cubed_3 = column5_row393 * column5_row489;
    let poseidon_poseidon_full_rounds_state1_cubed_3 = column5_row457 * column5_row409;
    let poseidon_poseidon_full_rounds_state2_cubed_3 = column5_row425 * column5_row473;
    let poseidon_poseidon_partial_rounds_state0_cubed_0 = column5_row6 * column5_row14;
    let poseidon_poseidon_partial_rounds_state0_cubed_1 = column5_row22 * column5_row30;
    let poseidon_poseidon_partial_rounds_state0_cubed_2 = column5_row38 * column5_row46;
    let poseidon_poseidon_partial_rounds_state1_cubed_0 = column5_row1 * column5_row17;
    let poseidon_poseidon_partial_rounds_state1_cubed_1 = column5_row33 * column5_row49;
    let poseidon_poseidon_partial_rounds_state1_cubed_2 = column5_row65 * column5_row81;
    let poseidon_poseidon_partial_rounds_state1_cubed_19 = column5_row609 * column5_row625;
    let poseidon_poseidon_partial_rounds_state1_cubed_20 = column5_row641 * column5_row657;
    let poseidon_poseidon_partial_rounds_state1_cubed_21 = column5_row673 * column5_row689;

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
    value = (column1_row1
        - (((column0_row0 * global_values.offset_size + column4_row4)
            * global_values.offset_size
            + column4_row8)
            * global_values.offset_size
            + column4_row0))
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
    value = (column1_row8 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_0 * column5_row8
            + (Felt::ONE - cpu_decode_opcode_range_check_bit_0) * column5_row0
            + column4_row0))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[7] * value;

    // Constraint: cpu/operands/mem0_addr.
    value = (column1_row4 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_1 * column5_row8
            + (Felt::ONE - cpu_decode_opcode_range_check_bit_1) * column5_row0
            + column4_row8))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[8] * value;

    // Constraint: cpu/operands/mem1_addr.
    value = (column1_row12 + global_values.half_offset_size
        - (cpu_decode_opcode_range_check_bit_2 * column1_row0
            + cpu_decode_opcode_range_check_bit_4 * column5_row0
            + cpu_decode_opcode_range_check_bit_3 * column5_row8
            + cpu_decode_flag_op1_base_op0_0 * column1_row5
            + column4_row4))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[9] * value;

    // Constraint: cpu/operands/ops_mul.
    value = (column5_row4 - column1_row5 * column1_row13)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[10] * value;

    // Constraint: cpu/operands/res.
    value = ((Felt::ONE - cpu_decode_opcode_range_check_bit_9) * column5_row12
        - (cpu_decode_opcode_range_check_bit_5 * (column1_row5 + column1_row13)
            + cpu_decode_opcode_range_check_bit_6 * column5_row4
            + cpu_decode_flag_res_op1_0 * column1_row13))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[11] * value;

    // Constraint: cpu/update_registers/update_pc/tmp0.
    value = (column5_row2 - cpu_decode_opcode_range_check_bit_9 * column1_row9)
        * domain24.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[12] * value;

    // Constraint: cpu/update_registers/update_pc/tmp1.
    value = (column5_row10 - column5_row2 * column5_row12)
        * domain24.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[13] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_negative.
    value = ((Felt::ONE - cpu_decode_opcode_range_check_bit_9) * column1_row16
        + column5_row2 * (column1_row16 - (column1_row0 + column1_row13))
        - (cpu_decode_flag_pc_update_regular_0 * npc_reg_0
            + cpu_decode_opcode_range_check_bit_7 * column5_row12
            + cpu_decode_opcode_range_check_bit_8 * (column1_row0 + column5_row12)))
        * domain24.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[14] * value;

    // Constraint: cpu/update_registers/update_pc/pc_cond_positive.
    value = ((column5_row10 - cpu_decode_opcode_range_check_bit_9) * (column1_row16 - npc_reg_0))
        * domain24.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[15] * value;

    // Constraint: cpu/update_registers/update_ap/ap_update.
    value = (column5_row16
        - (column5_row0
            + cpu_decode_opcode_range_check_bit_10 * column5_row12
            + cpu_decode_opcode_range_check_bit_11
            + cpu_decode_opcode_range_check_bit_12 * Felt::TWO))
        * domain24.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[16] * value;

    // Constraint: cpu/update_registers/update_fp/fp_update.
    value = (column5_row24
        - (cpu_decode_fp_update_regular_0 * column5_row8
            + cpu_decode_opcode_range_check_bit_13 * column1_row9
            + cpu_decode_opcode_range_check_bit_12 * (column5_row0 + 2)))
        * domain24.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[17] * value;

    // Constraint: cpu/opcodes/call/push_fp.
    value = (cpu_decode_opcode_range_check_bit_12 * (column1_row9 - column5_row8))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[18] * value;

    // Constraint: cpu/opcodes/call/push_pc.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column1_row5 - (column1_row0 + cpu_decode_opcode_range_check_bit_2 + 1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[19] * value;

    // Constraint: cpu/opcodes/call/off0.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column4_row0 - global_values.half_offset_size))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[20] * value;

    // Constraint: cpu/opcodes/call/off1.
    value = (cpu_decode_opcode_range_check_bit_12
        * (column4_row8 - (global_values.half_offset_size + 1)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[21] * value;

    // Constraint: cpu/opcodes/call/flags.
    value = (cpu_decode_opcode_range_check_bit_12
        * (cpu_decode_opcode_range_check_bit_12
            + cpu_decode_opcode_range_check_bit_12
            + 1
            + Felt::ONE
            - (cpu_decode_opcode_range_check_bit_0 + cpu_decode_opcode_range_check_bit_1 + 4)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[22] * value;

    // Constraint: cpu/opcodes/ret/off0.
    value = (cpu_decode_opcode_range_check_bit_13
        * (column4_row0 + 2 - global_values.half_offset_size))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[23] * value;

    // Constraint: cpu/opcodes/ret/off2.
    value = (cpu_decode_opcode_range_check_bit_13
        * (column4_row4 + Felt::ONE - global_values.half_offset_size))
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
    value = (cpu_decode_opcode_range_check_bit_14 * (column1_row9 - column5_row12))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[26] * value;

    // Constraint: initial_ap.
    value = (column5_row0 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[27] * value;

    // Constraint: initial_fp.
    value = (column5_row8 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[28] * value;

    // Constraint: initial_pc.
    value = (column1_row0 - global_values.initial_pc)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[29] * value;

    // Constraint: final_ap.
    value = (column5_row0 - global_values.final_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain24));
    total_sum += constraint_coefficients[30] * value;

    // Constraint: final_fp.
    value = (column5_row8 - global_values.initial_ap)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain24));
    total_sum += constraint_coefficients[31] * value;

    // Constraint: final_pc.
    value = (column1_row0 - global_values.final_pc)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain24));
    total_sum += constraint_coefficients[32] * value;

    // Constraint: memory/multi_column_perm/perm/init0.
    value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column2_row0
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column2_row1))
        * column6_inter1_row0
        + column1_row0
        + global_values.memory_multi_column_perm_hash_interaction_elm0 * column1_row1
        - global_values.memory_multi_column_perm_perm_interaction_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[33] * value;

    // Constraint: memory/multi_column_perm/perm/step0.
    value = ((global_values.memory_multi_column_perm_perm_interaction_elm
        - (column2_row2
            + global_values.memory_multi_column_perm_hash_interaction_elm0 * column2_row3))
        * column6_inter1_row2
        - (global_values.memory_multi_column_perm_perm_interaction_elm
            - (column1_row2
                + global_values.memory_multi_column_perm_hash_interaction_elm0 * column1_row3))
            * column6_inter1_row0)
        * domain26.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[34] * value;

    // Constraint: memory/multi_column_perm/perm/last.
    value = (column6_inter1_row0 - global_values.memory_multi_column_perm_perm_public_memory_prod)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain26));
    total_sum += constraint_coefficients[35] * value;

    // Constraint: memory/diff_is_bit.
    value = (memory_address_diff_0 * memory_address_diff_0 - memory_address_diff_0)
        * domain26.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[36] * value;

    // Constraint: memory/is_func.
    value = ((memory_address_diff_0 - 1) * (column2_row1 - column2_row3))
        * domain26.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[37] * value;

    // Constraint: memory/initial_addr.
    value = (column2_row0 - 1).field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[38] * value;

    // Constraint: public_memory_addr_zero.
    value = (column1_row2).field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[39] * value;

    // Constraint: public_memory_value_zero.
    value = (column1_row3).field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[40] * value;

    // Constraint: range_check16/perm/init0.
    value = ((global_values.range_check16_perm_interaction_elm - column4_row2)
        * column7_inter1_row1
        + column4_row0
        - global_values.range_check16_perm_interaction_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[41] * value;

    // Constraint: range_check16/perm/step0.
    value = ((global_values.range_check16_perm_interaction_elm - column4_row6)
        * column7_inter1_row5
        - (global_values.range_check16_perm_interaction_elm - column4_row4) * column7_inter1_row1)
        * domain27.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[42] * value;

    // Constraint: range_check16/perm/last.
    value = (column7_inter1_row1 - global_values.range_check16_perm_public_memory_prod)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain27));
    total_sum += constraint_coefficients[43] * value;

    // Constraint: range_check16/diff_is_bit.
    value = (range_check16_diff_0 * range_check16_diff_0 - range_check16_diff_0)
        * domain27.field_div(&NonZeroFelt::from_felt_unchecked(domain2));
    total_sum += constraint_coefficients[44] * value;

    // Constraint: range_check16/minimum.
    value = (column4_row2 - global_values.range_check_min)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[45] * value;

    // Constraint: range_check16/maximum.
    value = (column4_row2 - global_values.range_check_max)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain27));
    total_sum += constraint_coefficients[46] * value;

    // Constraint: diluted_check/permutation/init0.
    value = ((global_values.diluted_check_permutation_interaction_elm - column3_row1)
        * column7_inter1_row0
        + column3_row0
        - global_values.diluted_check_permutation_interaction_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[47] * value;

    // Constraint: diluted_check/permutation/step0.
    value = ((global_values.diluted_check_permutation_interaction_elm - column3_row3)
        * column7_inter1_row2
        - (global_values.diluted_check_permutation_interaction_elm - column3_row2)
            * column7_inter1_row0)
        * domain26.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[48] * value;

    // Constraint: diluted_check/permutation/last.
    value = (column7_inter1_row0 - global_values.diluted_check_permutation_public_memory_prod)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain26));
    total_sum += constraint_coefficients[49] * value;

    // Constraint: diluted_check/init.
    value = (column6_inter1_row1 - 1).field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[50] * value;

    // Constraint: diluted_check/first_element.
    value = (column3_row1 - global_values.diluted_check_first_elm)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[51] * value;

    // Constraint: diluted_check/step.
    value = (column6_inter1_row3
        - (column6_inter1_row1
            * (Felt::ONE
                + global_values.diluted_check_interaction_z * (column3_row3 - column3_row1))
            + global_values.diluted_check_interaction_alpha
                * (column3_row3 - column3_row1)
                * (column3_row3 - column3_row1)))
        * domain26.field_div(&NonZeroFelt::from_felt_unchecked(domain1));
    total_sum += constraint_coefficients[52] * value;

    // Constraint: diluted_check/last.
    value = (column6_inter1_row1 - global_values.diluted_check_final_cum_val)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain26));
    total_sum += constraint_coefficients[53] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/last_one_is_zero.
    value = (column5_row57 * (column4_row3 - (column4_row11 + column4_row11)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[54] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
    value = (column5_row57
        * (column4_row11
            - Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000")
                * column4_row1539))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[55] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit192.
    value = (column5_row57
        - column4_row2047 * (column4_row1539 - (column4_row1547 + column4_row1547)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[56] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
    value = (column4_row2047 * (column4_row1547 - Felt::from(8) * column4_row1571))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[57] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit196.
    value = (column4_row2047
        - (column4_row2011 - (column4_row2019 + column4_row2019))
            * (column4_row1571 - (column4_row1579 + column4_row1579)))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[58] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
    value = ((column4_row2011 - (column4_row2019 + column4_row2019))
        * (column4_row1579 - Felt::from_hex_unchecked("0x40000000000000") * column4_row2011))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[59] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/booleanity_test.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (pedersen_hash0_ec_subset_sum_bit_0 - 1))
        * domain20.field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[60] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/bit_extraction_end.
    value = (column4_row3).field_div(&NonZeroFelt::from_felt_unchecked(domain21));
    total_sum += constraint_coefficients[61] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/zeros_tail.
    value = (column4_row3).field_div(&NonZeroFelt::from_felt_unchecked(domain20));
    total_sum += constraint_coefficients[62] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/slope.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (column4_row5 - global_values.pedersen_points_y)
        - column4_row7 * (column4_row1 - global_values.pedersen_points_x))
        * domain20.field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[63] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/x.
    value = (column4_row7 * column4_row7
        - pedersen_hash0_ec_subset_sum_bit_0
            * (column4_row1 + global_values.pedersen_points_x + column4_row9))
        * domain20.field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[64] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/add_points/y.
    value = (pedersen_hash0_ec_subset_sum_bit_0 * (column4_row5 + column4_row13)
        - column4_row7 * (column4_row1 - column4_row9))
        * domain20.field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[65] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/x.
    value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column4_row9 - column4_row1))
        * domain20.field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[66] * value;

    // Constraint: pedersen/hash0/ec_subset_sum/copy_point/y.
    value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column4_row13 - column4_row5))
        * domain20.field_div(&NonZeroFelt::from_felt_unchecked(domain3));
    total_sum += constraint_coefficients[67] * value;

    // Constraint: pedersen/hash0/copy_point/x.
    value = (column4_row2049 - column4_row2041)
        * domain22.field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[68] * value;

    // Constraint: pedersen/hash0/copy_point/y.
    value = (column4_row2053 - column4_row2045)
        * domain22.field_div(&NonZeroFelt::from_felt_unchecked(domain19));
    total_sum += constraint_coefficients[69] * value;

    // Constraint: pedersen/hash0/init/x.
    value = (column4_row1 - global_values.pedersen_shift_point.x)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain23));
    total_sum += constraint_coefficients[70] * value;

    // Constraint: pedersen/hash0/init/y.
    value = (column4_row5 - global_values.pedersen_shift_point.y)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain23));
    total_sum += constraint_coefficients[71] * value;

    // Constraint: pedersen/input0_value0.
    value = (column1_row11 - column4_row3).field_div(&NonZeroFelt::from_felt_unchecked(domain23));
    total_sum += constraint_coefficients[72] * value;

    // Constraint: pedersen/input0_addr.
    value = (column1_row4106 - (column1_row1034 + 1))
        * domain28.field_div(&NonZeroFelt::from_felt_unchecked(domain23));
    total_sum += constraint_coefficients[73] * value;

    // Constraint: pedersen/init_addr.
    value = (column1_row10 - global_values.initial_pedersen_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[74] * value;

    // Constraint: pedersen/input1_value0.
    value =
        (column1_row2059 - column4_row2051).field_div(&NonZeroFelt::from_felt_unchecked(domain23));
    total_sum += constraint_coefficients[75] * value;

    // Constraint: pedersen/input1_addr.
    value = (column1_row2058 - (column1_row10 + 1))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain23));
    total_sum += constraint_coefficients[76] * value;

    // Constraint: pedersen/output_value0.
    value =
        (column1_row1035 - column4_row4089).field_div(&NonZeroFelt::from_felt_unchecked(domain23));
    total_sum += constraint_coefficients[77] * value;

    // Constraint: pedersen/output_addr.
    value = (column1_row1034 - (column1_row2058 + 1))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain23));
    total_sum += constraint_coefficients[78] * value;

    // Constraint: range_check_builtin/value.
    value = (range_check_builtin_value7_0 - column1_row139)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[79] * value;

    // Constraint: range_check_builtin/addr_step.
    value = (column1_row394 - (column1_row138 + 1))
        * domain29.field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[80] * value;

    // Constraint: range_check_builtin/init_addr.
    value = (column1_row138 - global_values.initial_range_check_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[81] * value;

    // Constraint: bitwise/init_var_pool_addr.
    value = (column1_row42 - global_values.initial_bitwise_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[82] * value;

    // Constraint: bitwise/step_var_pool_addr.
    value = (column1_row106 - (column1_row42 + 1))
        * domain10.field_div(&NonZeroFelt::from_felt_unchecked(domain7));
    total_sum += constraint_coefficients[83] * value;

    // Constraint: bitwise/x_or_y_addr.
    value = (column1_row74 - (column1_row234 + 1))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[84] * value;

    // Constraint: bitwise/next_var_pool_addr.
    value = (column1_row298 - (column1_row74 + 1))
        * domain29.field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[85] * value;

    // Constraint: bitwise/partition.
    value = (bitwise_sum_var_0_0 + bitwise_sum_var_8_0 - column1_row43)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain7));
    total_sum += constraint_coefficients[86] * value;

    // Constraint: bitwise/or_is_and_plus_xor.
    value = (column1_row75 - (column1_row171 + column1_row235))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[87] * value;

    // Constraint: bitwise/addition_is_xor_with_and.
    value = (column3_row0 + column3_row64 - (column3_row192 + column3_row128 + column3_row128))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain11));
    total_sum += constraint_coefficients[88] * value;

    // Constraint: bitwise/unique_unpacking192.
    value = ((column3_row176 + column3_row240) * Felt::from(16) - column3_row2)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[89] * value;

    // Constraint: bitwise/unique_unpacking193.
    value = ((column3_row180 + column3_row244) * Felt::from(16) - column3_row130)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[90] * value;

    // Constraint: bitwise/unique_unpacking194.
    value = ((column3_row184 + column3_row248) * Felt::from(16) - column3_row66)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[91] * value;

    // Constraint: bitwise/unique_unpacking195.
    value = ((column3_row188 + column3_row252) * Felt::from(256) - column3_row194)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain9));
    total_sum += constraint_coefficients[92] * value;

    // Constraint: poseidon/param_0/init_input_output_addr.
    value = (column1_row266 - global_values.initial_poseidon_addr)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[93] * value;

    // Constraint: poseidon/param_0/addr_input_output_step.
    value = (column1_row778 - (column1_row266 + 3))
        * domain30.field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[94] * value;

    // Constraint: poseidon/param_1/init_input_output_addr.
    value = (column1_row202 - (global_values.initial_poseidon_addr + 1))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[95] * value;

    // Constraint: poseidon/param_1/addr_input_output_step.
    value = (column1_row714 - (column1_row202 + 3))
        * domain30.field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[96] * value;

    // Constraint: poseidon/param_2/init_input_output_addr.
    value = (column1_row458 - (global_values.initial_poseidon_addr + 2))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain25));
    total_sum += constraint_coefficients[97] * value;

    // Constraint: poseidon/param_2/addr_input_output_step.
    value = (column1_row970 - (column1_row458 + 3))
        * domain30.field_div(&NonZeroFelt::from_felt_unchecked(domain12));
    total_sum += constraint_coefficients[98] * value;

    // Constraint: poseidon/poseidon/full_rounds_state0_squaring.
    value = (column5_row9 * column5_row9 - column5_row105)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[99] * value;

    // Constraint: poseidon/poseidon/full_rounds_state1_squaring.
    value = (column5_row73 * column5_row73 - column5_row25)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[100] * value;

    // Constraint: poseidon/poseidon/full_rounds_state2_squaring.
    value = (column5_row41 * column5_row41 - column5_row89)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[101] * value;

    // Constraint: poseidon/poseidon/partial_rounds_state0_squaring.
    value = (column5_row6 * column5_row6 - column5_row14)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[102] * value;

    // Constraint: poseidon/poseidon/partial_rounds_state1_squaring.
    value = (column5_row1 * column5_row1 - column5_row17)
        * domain15.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[103] * value;

    // Constraint: poseidon/poseidon/add_first_round_key0.
    value = (column1_row267
        + Felt::from_hex_unchecked(
            "0x6861759EA556A2339DD92F9562A30B9E58E2AD98109AE4780B7FD8EAC77FE6F",
        )
        - column5_row9)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[104] * value;

    // Constraint: poseidon/poseidon/add_first_round_key1.
    value = (column1_row203
        + Felt::from_hex_unchecked(
            "0x3827681995D5AF9FFC8397A3D00425A3DA43F76ABF28A64E4AB1A22F27508C4",
        )
        - column5_row73)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[105] * value;

    // Constraint: poseidon/poseidon/add_first_round_key2.
    value = (column1_row459
        + Felt::from_hex_unchecked(
            "0x3A3956D2FAD44D0E7F760A2277DC7CB2CAC75DC279B2D687A0DBE17704A8309",
        )
        - column5_row41)
        .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[106] * value;

    // Constraint: poseidon/poseidon/full_round0.
    value = (column5_row137
        - (poseidon_poseidon_full_rounds_state0_cubed_0
            + poseidon_poseidon_full_rounds_state0_cubed_0
            + poseidon_poseidon_full_rounds_state0_cubed_0
            + poseidon_poseidon_full_rounds_state1_cubed_0
            + poseidon_poseidon_full_rounds_state2_cubed_0
            + global_values.poseidon_poseidon_full_round_key0))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[107] * value;

    // Constraint: poseidon/poseidon/full_round1.
    value = (column5_row201 + poseidon_poseidon_full_rounds_state1_cubed_0
        - (poseidon_poseidon_full_rounds_state0_cubed_0
            + poseidon_poseidon_full_rounds_state2_cubed_0
            + global_values.poseidon_poseidon_full_round_key1))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[108] * value;

    // Constraint: poseidon/poseidon/full_round2.
    value = (column5_row169
        + poseidon_poseidon_full_rounds_state2_cubed_0
        + poseidon_poseidon_full_rounds_state2_cubed_0
        - (poseidon_poseidon_full_rounds_state0_cubed_0
            + poseidon_poseidon_full_rounds_state1_cubed_0
            + global_values.poseidon_poseidon_full_round_key2))
        * domain13.field_div(&NonZeroFelt::from_felt_unchecked(domain8));
    total_sum += constraint_coefficients[109] * value;

    // Constraint: poseidon/poseidon/last_full_round0.
    value = (column1_row779
        - (poseidon_poseidon_full_rounds_state0_cubed_7
            + poseidon_poseidon_full_rounds_state0_cubed_7
            + poseidon_poseidon_full_rounds_state0_cubed_7
            + poseidon_poseidon_full_rounds_state1_cubed_7
            + poseidon_poseidon_full_rounds_state2_cubed_7))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[110] * value;

    // Constraint: poseidon/poseidon/last_full_round1.
    value = (column1_row715 + poseidon_poseidon_full_rounds_state1_cubed_7
        - (poseidon_poseidon_full_rounds_state0_cubed_7
            + poseidon_poseidon_full_rounds_state2_cubed_7))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[111] * value;

    // Constraint: poseidon/poseidon/last_full_round2.
    value = (column1_row971
        + poseidon_poseidon_full_rounds_state2_cubed_7
        + poseidon_poseidon_full_rounds_state2_cubed_7
        - (poseidon_poseidon_full_rounds_state0_cubed_7
            + poseidon_poseidon_full_rounds_state1_cubed_7))
        .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[112] * value;

    // Constraint: poseidon/poseidon/copy_partial_rounds0_i0.
    value = (column5_row982 - column5_row1).field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[113] * value;

    // Constraint: poseidon/poseidon/copy_partial_rounds0_i1.
    value = (column5_row998 - column5_row33).field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[114] * value;

    // Constraint: poseidon/poseidon/copy_partial_rounds0_i2.
    value =
        (column5_row1014 - column5_row65).field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[115] * value;

    // Constraint: poseidon/poseidon/margin_full_to_partial0.
    value = (column5_row6
        + poseidon_poseidon_full_rounds_state2_cubed_3
        + poseidon_poseidon_full_rounds_state2_cubed_3
        - (poseidon_poseidon_full_rounds_state0_cubed_3
            + poseidon_poseidon_full_rounds_state1_cubed_3
            + Felt::from_hex_unchecked(
                "0x4B085EB1DF4258C3453CC97445954BF3433B6AB9DD5A99592864C00F54A3F9A",
            )))
    .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[116] * value;

    // Constraint: poseidon/poseidon/margin_full_to_partial1.
    value = (column5_row22
        - (Felt::from_hex_unchecked(
            "0x800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD",
        ) * poseidon_poseidon_full_rounds_state1_cubed_3
            + Felt::from(10) * poseidon_poseidon_full_rounds_state2_cubed_3
            + Felt::from(4) * column5_row6
            + Felt::from_hex_unchecked(
                "0x800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ) * poseidon_poseidon_partial_rounds_state0_cubed_0
            + Felt::from_hex_unchecked(
                "0x46FB825257FEC76C50FE043684D4E6D2D2F2FDFE9B7C8D7128CA7ACC0F66F30",
            )))
    .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[117] * value;

    // Constraint: poseidon/poseidon/margin_full_to_partial2.
    value = (column5_row38
        - (Felt::from(8) * poseidon_poseidon_full_rounds_state2_cubed_3
            + Felt::from(4) * column5_row6
            + Felt::from(6) * poseidon_poseidon_partial_rounds_state0_cubed_0
            + column5_row22
            + column5_row22
            + Felt::from_hex_unchecked(
                "0x800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ) * poseidon_poseidon_partial_rounds_state0_cubed_1
            + Felt::from_hex_unchecked(
                "0xF2193BA0C7EA33CE6222D9446C1E166202AE5461005292F4A2BCB93420151A",
            )))
    .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[118] * value;

    // Constraint: poseidon/poseidon/partial_round0.
    value = (column5_row54
        - (Felt::from(8) * poseidon_poseidon_partial_rounds_state0_cubed_0
            + Felt::from(4) * column5_row22
            + Felt::from(6) * poseidon_poseidon_partial_rounds_state0_cubed_1
            + column5_row38
            + column5_row38
            + Felt::from_hex_unchecked(
                "0x800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ) * poseidon_poseidon_partial_rounds_state0_cubed_2
            + global_values.poseidon_poseidon_partial_round_key0))
        * domain17.field_div(&NonZeroFelt::from_felt_unchecked(domain5));
    total_sum += constraint_coefficients[119] * value;

    // Constraint: poseidon/poseidon/partial_round1.
    value = (column5_row97
        - (Felt::from(8) * poseidon_poseidon_partial_rounds_state1_cubed_0
            + Felt::from(4) * column5_row33
            + Felt::from(6) * poseidon_poseidon_partial_rounds_state1_cubed_1
            + column5_row65
            + column5_row65
            + Felt::from_hex_unchecked(
                "0x800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ) * poseidon_poseidon_partial_rounds_state1_cubed_2
            + global_values.poseidon_poseidon_partial_round_key1))
        * domain18.field_div(&NonZeroFelt::from_felt_unchecked(domain6));
    total_sum += constraint_coefficients[120] * value;

    // Constraint: poseidon/poseidon/margin_partial_to_full0.
    value = (column5_row521
        - (Felt::from(16) * poseidon_poseidon_partial_rounds_state1_cubed_19
            + Felt::from(8) * column5_row641
            + Felt::from(16) * poseidon_poseidon_partial_rounds_state1_cubed_20
            + Felt::from(6) * column5_row673
            + poseidon_poseidon_partial_rounds_state1_cubed_21
            + Felt::from_hex_unchecked(
                "0x13D1B5CFD87693224F0AC561AB2C15CA53365D768311AF59CEFAF701BC53B37",
            )))
    .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[121] * value;

    // Constraint: poseidon/poseidon/margin_partial_to_full1.
    value = (column5_row585
        - (Felt::from(4) * poseidon_poseidon_partial_rounds_state1_cubed_20
            + column5_row673
            + column5_row673
            + poseidon_poseidon_partial_rounds_state1_cubed_21
            + Felt::from_hex_unchecked(
                "0x3195D6B2D930E71CEDE286D5B8B41D49296DDF222BCD3BF3717A12A9A6947FF",
            )))
    .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[122] * value;

    // Constraint: poseidon/poseidon/margin_partial_to_full2.
    value = (column5_row553
        - (Felt::from(8) * poseidon_poseidon_partial_rounds_state1_cubed_19
            + Felt::from(4) * column5_row641
            + Felt::from(6) * poseidon_poseidon_partial_rounds_state1_cubed_20
            + column5_row673
            + column5_row673
            + Felt::from_hex_unchecked(
                "0x800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ) * poseidon_poseidon_partial_rounds_state1_cubed_21
            + Felt::from_hex_unchecked(
                "0x2C14FCCABC26929170CC7AC9989C823608B9008BEF3B8E16B6089A5D33CD72E",
            )))
    .field_div(&NonZeroFelt::from_felt_unchecked(domain16));
    total_sum += constraint_coefficients[123] * value;

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
    let pow1 = trace_generator.pow(4089_u128);
    let pow2 = trace_generator.pow(2011_u128);
    let pow3 = trace_generator.pow(1539_u128);
    let pow4 = trace_generator.pow(1_u128);
    let pow5 = pow4 * pow4; // pow(trace_generator, 2).
    let pow6 = pow4 * pow5; // pow(trace_generator, 3).
    let pow7 = pow4 * pow6; // pow(trace_generator, 4).
    let pow8 = pow4 * pow7; // pow(trace_generator, 5).
    let pow9 = pow4 * pow8; // pow(trace_generator, 6).
    let pow10 = pow4 * pow9; // pow(trace_generator, 7).
    let pow11 = pow4 * pow10; // pow(trace_generator, 8).
    let pow12 = pow3 * pow11; // pow(trace_generator, 1547).
    let pow13 = pow4 * pow11; // pow(trace_generator, 9).
    let pow14 = pow4 * pow13; // pow(trace_generator, 10).
    let pow15 = pow4 * pow14; // pow(trace_generator, 11).
    let pow16 = pow4 * pow15; // pow(trace_generator, 12).
    let pow17 = pow4 * pow16; // pow(trace_generator, 13).
    let pow18 = pow4 * pow17; // pow(trace_generator, 14).
    let pow19 = pow4 * pow18; // pow(trace_generator, 15).
    let pow20 = pow4 * pow19; // pow(trace_generator, 16).
    let pow21 = pow4 * pow20; // pow(trace_generator, 17).
    let pow22 = pow6 * pow21; // pow(trace_generator, 20).
    let pow23 = pow5 * pow22; // pow(trace_generator, 22).
    let pow24 = pow5 * pow23; // pow(trace_generator, 24).
    let pow25 = pow4 * pow24; // pow(trace_generator, 25).
    let pow26 = pow6 * pow25; // pow(trace_generator, 28).
    let pow27 = pow5 * pow26; // pow(trace_generator, 30).
    let pow28 = pow5 * pow27; // pow(trace_generator, 32).
    let pow29 = pow4 * pow28; // pow(trace_generator, 33).
    let pow30 = pow3 * pow28; // pow(trace_generator, 1571).
    let pow31 = pow6 * pow29; // pow(trace_generator, 36).
    let pow32 = pow5 * pow31; // pow(trace_generator, 38).
    let pow33 = pow5 * pow32; // pow(trace_generator, 40).
    let pow34 = pow4 * pow33; // pow(trace_generator, 41).
    let pow35 = pow4 * pow34; // pow(trace_generator, 42).
    let pow36 = pow4 * pow35; // pow(trace_generator, 43).
    let pow37 = pow4 * pow36; // pow(trace_generator, 44).
    let pow38 = pow5 * pow37; // pow(trace_generator, 46).
    let pow39 = pow5 * pow38; // pow(trace_generator, 48).
    let pow40 = pow4 * pow39; // pow(trace_generator, 49).
    let pow41 = pow6 * pow40; // pow(trace_generator, 52).
    let pow42 = pow5 * pow41; // pow(trace_generator, 54).
    let pow43 = pow5 * pow42; // pow(trace_generator, 56).
    let pow44 = pow4 * pow43; // pow(trace_generator, 57).
    let pow45 = pow6 * pow44; // pow(trace_generator, 60).
    let pow46 = pow7 * pow45; // pow(trace_generator, 64).
    let pow47 = pow4 * pow46; // pow(trace_generator, 65).
    let pow48 = pow4 * pow47; // pow(trace_generator, 66).
    let pow49 = pow10 * pow48; // pow(trace_generator, 73).
    let pow50 = pow4 * pow49; // pow(trace_generator, 74).
    let pow51 = pow4 * pow50; // pow(trace_generator, 75).
    let pow52 = pow4 * pow51; // pow(trace_generator, 76).
    let pow53 = pow8 * pow52; // pow(trace_generator, 81).
    let pow54 = pow11 * pow53; // pow(trace_generator, 89).
    let pow55 = pow11 * pow54; // pow(trace_generator, 97).
    let pow56 = pow11 * pow55; // pow(trace_generator, 105).
    let pow57 = pow4 * pow56; // pow(trace_generator, 106).
    let pow58 = pow5 * pow57; // pow(trace_generator, 108).
    let pow59 = pow22 * pow58; // pow(trace_generator, 128).
    let pow60 = pow5 * pow59; // pow(trace_generator, 130).
    let pow61 = pow10 * pow60; // pow(trace_generator, 137).
    let pow62 = pow4 * pow61; // pow(trace_generator, 138).
    let pow63 = pow4 * pow62; // pow(trace_generator, 139).
    let pow64 = pow27 * pow63; // pow(trace_generator, 169).
    let pow65 = pow5 * pow64; // pow(trace_generator, 171).
    let pow66 = pow4 * pow63; // pow(trace_generator, 140).
    let pow67 = pow4 * pow65; // pow(trace_generator, 172).
    let pow68 = pow7 * pow67; // pow(trace_generator, 176).
    let pow69 = pow7 * pow68; // pow(trace_generator, 180).
    let pow70 = pow7 * pow69; // pow(trace_generator, 184).
    let pow71 = pow7 * pow70; // pow(trace_generator, 188).
    let pow72 = pow7 * pow71; // pow(trace_generator, 192).
    let pow73 = pow5 * pow72; // pow(trace_generator, 194).
    let pow74 = pow10 * pow73; // pow(trace_generator, 201).
    let pow75 = pow4 * pow74; // pow(trace_generator, 202).
    let pow76 = pow4 * pow75; // pow(trace_generator, 203).
    let pow77 = pow72 * pow74; // pow(trace_generator, 393).
    let pow78 = pow4 * pow76; // pow(trace_generator, 204).
    let pow79 = pow27 * pow78; // pow(trace_generator, 234).
    let pow80 = pow4 * pow79; // pow(trace_generator, 235).
    let pow81 = pow4 * pow80; // pow(trace_generator, 236).
    let pow82 = pow7 * pow81; // pow(trace_generator, 240).
    let pow83 = pow7 * pow82; // pow(trace_generator, 244).
    let pow84 = pow7 * pow83; // pow(trace_generator, 248).
    let pow85 = pow7 * pow84; // pow(trace_generator, 252).
    let pow86 = pow18 * pow85; // pow(trace_generator, 266).
    let pow87 = pow4 * pow86; // pow(trace_generator, 267).
    let pow88 = pow4 * pow77; // pow(trace_generator, 394).
    let pow89 = pow19 * pow88; // pow(trace_generator, 409).
    let pow90 = pow20 * pow89; // pow(trace_generator, 425).
    let pow91 = pow28 * pow90; // pow(trace_generator, 457).
    let pow92 = pow4 * pow91; // pow(trace_generator, 458).
    let pow93 = pow4 * pow92; // pow(trace_generator, 459).
    let pow94 = pow18 * pow93; // pow(trace_generator, 473).
    let pow95 = pow20 * pow94; // pow(trace_generator, 489).
    let pow96 = pow28 * pow95; // pow(trace_generator, 521).
    let pow97 = pow28 * pow96; // pow(trace_generator, 553).
    let pow98 = pow28 * pow97; // pow(trace_generator, 585).
    let pow99 = pow24 * pow98; // pow(trace_generator, 609).
    let pow100 = pow20 * pow99; // pow(trace_generator, 625).
    let pow101 = pow20 * pow100; // pow(trace_generator, 641).
    let pow102 = pow20 * pow101; // pow(trace_generator, 657).
    let pow103 = pow84 * pow102; // pow(trace_generator, 905).
    let pow104 = pow20 * pow102; // pow(trace_generator, 673).
    let pow105 = pow20 * pow103; // pow(trace_generator, 921).
    let pow106 = pow20 * pow104; // pow(trace_generator, 689).
    let pow107 = pow20 * pow105; // pow(trace_generator, 937).
    let pow108 = pow28 * pow107; // pow(trace_generator, 969).
    let pow109 = pow25 * pow106; // pow(trace_generator, 714).
    let pow110 = pow46 * pow109; // pow(trace_generator, 778).
    let pow111 = pow4 * pow108; // pow(trace_generator, 970).
    let pow112 = pow3 * pow33; // pow(trace_generator, 1579).
    let pow113 = pow4 * pow109; // pow(trace_generator, 715).
    let pow114 = pow4 * pow110; // pow(trace_generator, 779).
    let pow115 = pow28 * pow86; // pow(trace_generator, 298).
    let pow116 = pow4 * pow111; // pow(trace_generator, 971).
    let pow117 = pow15 * pow116; // pow(trace_generator, 982).
    let pow118 = pow6 * pow117; // pow(trace_generator, 985).
    let pow119 = pow17 * pow118; // pow(trace_generator, 998).
    let pow120 = pow6 * pow119; // pow(trace_generator, 1001).
    let pow121 = pow17 * pow120; // pow(trace_generator, 1014).
    let pow122 = pow22 * pow121; // pow(trace_generator, 1034).
    let pow123 = pow2 * pow11; // pow(trace_generator, 2019).
    let pow124 = pow2 * pow27; // pow(trace_generator, 2041).
    let pow125 = pow7 * pow124; // pow(trace_generator, 2045).
    let pow126 = pow2 * pow31; // pow(trace_generator, 2047).
    let pow127 = pow4 * pow122; // pow(trace_generator, 1035).
    let pow128 = pow2 * pow32; // pow(trace_generator, 2049).
    let pow129 = pow2 * pow33; // pow(trace_generator, 2051).
    let pow130 = pow2 * pow35; // pow(trace_generator, 2053).
    let pow131 = pow8 * pow130; // pow(trace_generator, 2058).
    let pow132 = pow2 * pow39; // pow(trace_generator, 2059).
    let pow133 = pow1 * pow21; // pow(trace_generator, 4106).

    // Fetch columns.
    let column0 = column_values[0];
    let column1 = column_values[1];
    let column2 = column_values[2];
    let column3 = column_values[3];
    let column4 = column_values[4];
    let column5 = column_values[5];
    let column6 = column_values[6];
    let column7 = column_values[7];

    // Sum constraints.
    let mut total_sum = Felt::ZERO;

    let mut value = (column0 - oods_values[0])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[0] * value;

    value = (column0 - oods_values[1])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[1] * value;

    value = (column0 - oods_values[2])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
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
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow9 * oods_point));
    total_sum += constraint_coefficients[6] * value;

    value = (column0 - oods_values[7])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow10 * oods_point));
    total_sum += constraint_coefficients[7] * value;

    value = (column0 - oods_values[8])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[8] * value;

    value = (column0 - oods_values[9])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow13 * oods_point));
    total_sum += constraint_coefficients[9] * value;

    value = (column0 - oods_values[10])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[10] * value;

    value = (column0 - oods_values[11])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow15 * oods_point));
    total_sum += constraint_coefficients[11] * value;

    value = (column0 - oods_values[12])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[12] * value;

    value = (column0 - oods_values[13])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow17 * oods_point));
    total_sum += constraint_coefficients[13] * value;

    value = (column0 - oods_values[14])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[14] * value;

    value = (column0 - oods_values[15])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow19 * oods_point));
    total_sum += constraint_coefficients[15] * value;

    value = (column1 - oods_values[16])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[16] * value;

    value = (column1 - oods_values[17])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[17] * value;

    value = (column1 - oods_values[18])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[18] * value;

    value = (column1 - oods_values[19])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[19] * value;

    value = (column1 - oods_values[20])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[20] * value;

    value = (column1 - oods_values[21])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[21] * value;

    value = (column1 - oods_values[22])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[22] * value;

    value = (column1 - oods_values[23])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow13 * oods_point));
    total_sum += constraint_coefficients[23] * value;

    value = (column1 - oods_values[24])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[24] * value;

    value = (column1 - oods_values[25])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow15 * oods_point));
    total_sum += constraint_coefficients[25] * value;

    value = (column1 - oods_values[26])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[26] * value;

    value = (column1 - oods_values[27])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow17 * oods_point));
    total_sum += constraint_coefficients[27] * value;

    value = (column1 - oods_values[28])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow20 * oods_point));
    total_sum += constraint_coefficients[28] * value;

    value = (column1 - oods_values[29])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow35 * oods_point));
    total_sum += constraint_coefficients[29] * value;

    value = (column1 - oods_values[30])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow36 * oods_point));
    total_sum += constraint_coefficients[30] * value;

    value = (column1 - oods_values[31])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow50 * oods_point));
    total_sum += constraint_coefficients[31] * value;

    value = (column1 - oods_values[32])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow51 * oods_point));
    total_sum += constraint_coefficients[32] * value;

    value = (column1 - oods_values[33])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow57 * oods_point));
    total_sum += constraint_coefficients[33] * value;

    value = (column1 - oods_values[34])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow62 * oods_point));
    total_sum += constraint_coefficients[34] * value;

    value = (column1 - oods_values[35])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow63 * oods_point));
    total_sum += constraint_coefficients[35] * value;

    value = (column1 - oods_values[36])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow65 * oods_point));
    total_sum += constraint_coefficients[36] * value;

    value = (column1 - oods_values[37])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow75 * oods_point));
    total_sum += constraint_coefficients[37] * value;

    value = (column1 - oods_values[38])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow76 * oods_point));
    total_sum += constraint_coefficients[38] * value;

    value = (column1 - oods_values[39])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow79 * oods_point));
    total_sum += constraint_coefficients[39] * value;

    value = (column1 - oods_values[40])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow80 * oods_point));
    total_sum += constraint_coefficients[40] * value;

    value = (column1 - oods_values[41])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow86 * oods_point));
    total_sum += constraint_coefficients[41] * value;

    value = (column1 - oods_values[42])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow87 * oods_point));
    total_sum += constraint_coefficients[42] * value;

    value = (column1 - oods_values[43])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow115 * oods_point));
    total_sum += constraint_coefficients[43] * value;

    value = (column1 - oods_values[44])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow88 * oods_point));
    total_sum += constraint_coefficients[44] * value;

    value = (column1 - oods_values[45])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow92 * oods_point));
    total_sum += constraint_coefficients[45] * value;

    value = (column1 - oods_values[46])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow93 * oods_point));
    total_sum += constraint_coefficients[46] * value;

    value = (column1 - oods_values[47])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow109 * oods_point));
    total_sum += constraint_coefficients[47] * value;

    value = (column1 - oods_values[48])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow113 * oods_point));
    total_sum += constraint_coefficients[48] * value;

    value = (column1 - oods_values[49])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow110 * oods_point));
    total_sum += constraint_coefficients[49] * value;

    value = (column1 - oods_values[50])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow114 * oods_point));
    total_sum += constraint_coefficients[50] * value;

    value = (column1 - oods_values[51])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow111 * oods_point));
    total_sum += constraint_coefficients[51] * value;

    value = (column1 - oods_values[52])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow116 * oods_point));
    total_sum += constraint_coefficients[52] * value;

    value = (column1 - oods_values[53])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow122 * oods_point));
    total_sum += constraint_coefficients[53] * value;

    value = (column1 - oods_values[54])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow127 * oods_point));
    total_sum += constraint_coefficients[54] * value;

    value = (column1 - oods_values[55])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow131 * oods_point));
    total_sum += constraint_coefficients[55] * value;

    value = (column1 - oods_values[56])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow132 * oods_point));
    total_sum += constraint_coefficients[56] * value;

    value = (column1 - oods_values[57])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow133 * oods_point));
    total_sum += constraint_coefficients[57] * value;

    value = (column2 - oods_values[58])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[58] * value;

    value = (column2 - oods_values[59])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[59] * value;

    value = (column2 - oods_values[60])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[60] * value;

    value = (column2 - oods_values[61])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[61] * value;

    value = (column3 - oods_values[62])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[62] * value;

    value = (column3 - oods_values[63])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[63] * value;

    value = (column3 - oods_values[64])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[64] * value;

    value = (column3 - oods_values[65])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[65] * value;

    value = (column3 - oods_values[66])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[66] * value;

    value = (column3 - oods_values[67])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[67] * value;

    value = (column3 - oods_values[68])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[68] * value;

    value = (column3 - oods_values[69])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow20 * oods_point));
    total_sum += constraint_coefficients[69] * value;

    value = (column3 - oods_values[70])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow22 * oods_point));
    total_sum += constraint_coefficients[70] * value;

    value = (column3 - oods_values[71])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow24 * oods_point));
    total_sum += constraint_coefficients[71] * value;

    value = (column3 - oods_values[72])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow26 * oods_point));
    total_sum += constraint_coefficients[72] * value;

    value = (column3 - oods_values[73])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow28 * oods_point));
    total_sum += constraint_coefficients[73] * value;

    value = (column3 - oods_values[74])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow31 * oods_point));
    total_sum += constraint_coefficients[74] * value;

    value = (column3 - oods_values[75])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow33 * oods_point));
    total_sum += constraint_coefficients[75] * value;

    value = (column3 - oods_values[76])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow37 * oods_point));
    total_sum += constraint_coefficients[76] * value;

    value = (column3 - oods_values[77])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow39 * oods_point));
    total_sum += constraint_coefficients[77] * value;

    value = (column3 - oods_values[78])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow41 * oods_point));
    total_sum += constraint_coefficients[78] * value;

    value = (column3 - oods_values[79])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow43 * oods_point));
    total_sum += constraint_coefficients[79] * value;

    value = (column3 - oods_values[80])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow45 * oods_point));
    total_sum += constraint_coefficients[80] * value;

    value = (column3 - oods_values[81])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow46 * oods_point));
    total_sum += constraint_coefficients[81] * value;

    value = (column3 - oods_values[82])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow48 * oods_point));
    total_sum += constraint_coefficients[82] * value;

    value = (column3 - oods_values[83])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow59 * oods_point));
    total_sum += constraint_coefficients[83] * value;

    value = (column3 - oods_values[84])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow60 * oods_point));
    total_sum += constraint_coefficients[84] * value;

    value = (column3 - oods_values[85])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow68 * oods_point));
    total_sum += constraint_coefficients[85] * value;

    value = (column3 - oods_values[86])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow69 * oods_point));
    total_sum += constraint_coefficients[86] * value;

    value = (column3 - oods_values[87])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow70 * oods_point));
    total_sum += constraint_coefficients[87] * value;

    value = (column3 - oods_values[88])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow71 * oods_point));
    total_sum += constraint_coefficients[88] * value;

    value = (column3 - oods_values[89])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow72 * oods_point));
    total_sum += constraint_coefficients[89] * value;

    value = (column3 - oods_values[90])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow73 * oods_point));
    total_sum += constraint_coefficients[90] * value;

    value = (column3 - oods_values[91])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow82 * oods_point));
    total_sum += constraint_coefficients[91] * value;

    value = (column3 - oods_values[92])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow83 * oods_point));
    total_sum += constraint_coefficients[92] * value;

    value = (column3 - oods_values[93])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow84 * oods_point));
    total_sum += constraint_coefficients[93] * value;

    value = (column3 - oods_values[94])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow85 * oods_point));
    total_sum += constraint_coefficients[94] * value;

    value = (column4 - oods_values[95])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[95] * value;

    value = (column4 - oods_values[96])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[96] * value;

    value = (column4 - oods_values[97])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[97] * value;

    value = (column4 - oods_values[98])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[98] * value;

    value = (column4 - oods_values[99])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[99] * value;

    value = (column4 - oods_values[100])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[100] * value;

    value = (column4 - oods_values[101])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow9 * oods_point));
    total_sum += constraint_coefficients[101] * value;

    value = (column4 - oods_values[102])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow10 * oods_point));
    total_sum += constraint_coefficients[102] * value;

    value = (column4 - oods_values[103])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[103] * value;

    value = (column4 - oods_values[104])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow13 * oods_point));
    total_sum += constraint_coefficients[104] * value;

    value = (column4 - oods_values[105])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow15 * oods_point));
    total_sum += constraint_coefficients[105] * value;

    value = (column4 - oods_values[106])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[106] * value;

    value = (column4 - oods_values[107])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow17 * oods_point));
    total_sum += constraint_coefficients[107] * value;

    value = (column4 - oods_values[108])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow37 * oods_point));
    total_sum += constraint_coefficients[108] * value;

    value = (column4 - oods_values[109])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow52 * oods_point));
    total_sum += constraint_coefficients[109] * value;

    value = (column4 - oods_values[110])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow58 * oods_point));
    total_sum += constraint_coefficients[110] * value;

    value = (column4 - oods_values[111])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow66 * oods_point));
    total_sum += constraint_coefficients[111] * value;

    value = (column4 - oods_values[112])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow67 * oods_point));
    total_sum += constraint_coefficients[112] * value;

    value = (column4 - oods_values[113])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow78 * oods_point));
    total_sum += constraint_coefficients[113] * value;

    value = (column4 - oods_values[114])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow81 * oods_point));
    total_sum += constraint_coefficients[114] * value;

    value = (column4 - oods_values[115])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow3 * oods_point));
    total_sum += constraint_coefficients[115] * value;

    value = (column4 - oods_values[116])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow12 * oods_point));
    total_sum += constraint_coefficients[116] * value;

    value = (column4 - oods_values[117])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow30 * oods_point));
    total_sum += constraint_coefficients[117] * value;

    value = (column4 - oods_values[118])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow112 * oods_point));
    total_sum += constraint_coefficients[118] * value;

    value = (column4 - oods_values[119])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow2 * oods_point));
    total_sum += constraint_coefficients[119] * value;

    value = (column4 - oods_values[120])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow123 * oods_point));
    total_sum += constraint_coefficients[120] * value;

    value = (column4 - oods_values[121])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow124 * oods_point));
    total_sum += constraint_coefficients[121] * value;

    value = (column4 - oods_values[122])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow125 * oods_point));
    total_sum += constraint_coefficients[122] * value;

    value = (column4 - oods_values[123])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow126 * oods_point));
    total_sum += constraint_coefficients[123] * value;

    value = (column4 - oods_values[124])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow128 * oods_point));
    total_sum += constraint_coefficients[124] * value;

    value = (column4 - oods_values[125])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow129 * oods_point));
    total_sum += constraint_coefficients[125] * value;

    value = (column4 - oods_values[126])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow130 * oods_point));
    total_sum += constraint_coefficients[126] * value;

    value = (column4 - oods_values[127])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow1 * oods_point));
    total_sum += constraint_coefficients[127] * value;

    value = (column5 - oods_values[128])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[128] * value;

    value = (column5 - oods_values[129])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[129] * value;

    value = (column5 - oods_values[130])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[130] * value;

    value = (column5 - oods_values[131])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow7 * oods_point));
    total_sum += constraint_coefficients[131] * value;

    value = (column5 - oods_values[132])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow9 * oods_point));
    total_sum += constraint_coefficients[132] * value;

    value = (column5 - oods_values[133])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow11 * oods_point));
    total_sum += constraint_coefficients[133] * value;

    value = (column5 - oods_values[134])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow13 * oods_point));
    total_sum += constraint_coefficients[134] * value;

    value = (column5 - oods_values[135])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow14 * oods_point));
    total_sum += constraint_coefficients[135] * value;

    value = (column5 - oods_values[136])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow16 * oods_point));
    total_sum += constraint_coefficients[136] * value;

    value = (column5 - oods_values[137])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow18 * oods_point));
    total_sum += constraint_coefficients[137] * value;

    value = (column5 - oods_values[138])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow20 * oods_point));
    total_sum += constraint_coefficients[138] * value;

    value = (column5 - oods_values[139])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow21 * oods_point));
    total_sum += constraint_coefficients[139] * value;

    value = (column5 - oods_values[140])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow23 * oods_point));
    total_sum += constraint_coefficients[140] * value;

    value = (column5 - oods_values[141])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow24 * oods_point));
    total_sum += constraint_coefficients[141] * value;

    value = (column5 - oods_values[142])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow25 * oods_point));
    total_sum += constraint_coefficients[142] * value;

    value = (column5 - oods_values[143])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow27 * oods_point));
    total_sum += constraint_coefficients[143] * value;

    value = (column5 - oods_values[144])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow29 * oods_point));
    total_sum += constraint_coefficients[144] * value;

    value = (column5 - oods_values[145])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow32 * oods_point));
    total_sum += constraint_coefficients[145] * value;

    value = (column5 - oods_values[146])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow34 * oods_point));
    total_sum += constraint_coefficients[146] * value;

    value = (column5 - oods_values[147])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow38 * oods_point));
    total_sum += constraint_coefficients[147] * value;

    value = (column5 - oods_values[148])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow40 * oods_point));
    total_sum += constraint_coefficients[148] * value;

    value = (column5 - oods_values[149])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow42 * oods_point));
    total_sum += constraint_coefficients[149] * value;

    value = (column5 - oods_values[150])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow44 * oods_point));
    total_sum += constraint_coefficients[150] * value;

    value = (column5 - oods_values[151])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow47 * oods_point));
    total_sum += constraint_coefficients[151] * value;

    value = (column5 - oods_values[152])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow49 * oods_point));
    total_sum += constraint_coefficients[152] * value;

    value = (column5 - oods_values[153])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow53 * oods_point));
    total_sum += constraint_coefficients[153] * value;

    value = (column5 - oods_values[154])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow54 * oods_point));
    total_sum += constraint_coefficients[154] * value;

    value = (column5 - oods_values[155])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow55 * oods_point));
    total_sum += constraint_coefficients[155] * value;

    value = (column5 - oods_values[156])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow56 * oods_point));
    total_sum += constraint_coefficients[156] * value;

    value = (column5 - oods_values[157])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow61 * oods_point));
    total_sum += constraint_coefficients[157] * value;

    value = (column5 - oods_values[158])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow64 * oods_point));
    total_sum += constraint_coefficients[158] * value;

    value = (column5 - oods_values[159])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow74 * oods_point));
    total_sum += constraint_coefficients[159] * value;

    value = (column5 - oods_values[160])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow77 * oods_point));
    total_sum += constraint_coefficients[160] * value;

    value = (column5 - oods_values[161])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow89 * oods_point));
    total_sum += constraint_coefficients[161] * value;

    value = (column5 - oods_values[162])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow90 * oods_point));
    total_sum += constraint_coefficients[162] * value;

    value = (column5 - oods_values[163])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow91 * oods_point));
    total_sum += constraint_coefficients[163] * value;

    value = (column5 - oods_values[164])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow94 * oods_point));
    total_sum += constraint_coefficients[164] * value;

    value = (column5 - oods_values[165])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow95 * oods_point));
    total_sum += constraint_coefficients[165] * value;

    value = (column5 - oods_values[166])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow96 * oods_point));
    total_sum += constraint_coefficients[166] * value;

    value = (column5 - oods_values[167])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow97 * oods_point));
    total_sum += constraint_coefficients[167] * value;

    value = (column5 - oods_values[168])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow98 * oods_point));
    total_sum += constraint_coefficients[168] * value;

    value = (column5 - oods_values[169])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow99 * oods_point));
    total_sum += constraint_coefficients[169] * value;

    value = (column5 - oods_values[170])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow100 * oods_point));
    total_sum += constraint_coefficients[170] * value;

    value = (column5 - oods_values[171])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow101 * oods_point));
    total_sum += constraint_coefficients[171] * value;

    value = (column5 - oods_values[172])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow102 * oods_point));
    total_sum += constraint_coefficients[172] * value;

    value = (column5 - oods_values[173])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow104 * oods_point));
    total_sum += constraint_coefficients[173] * value;

    value = (column5 - oods_values[174])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow106 * oods_point));
    total_sum += constraint_coefficients[174] * value;

    value = (column5 - oods_values[175])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow103 * oods_point));
    total_sum += constraint_coefficients[175] * value;

    value = (column5 - oods_values[176])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow105 * oods_point));
    total_sum += constraint_coefficients[176] * value;

    value = (column5 - oods_values[177])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow107 * oods_point));
    total_sum += constraint_coefficients[177] * value;

    value = (column5 - oods_values[178])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow108 * oods_point));
    total_sum += constraint_coefficients[178] * value;

    value = (column5 - oods_values[179])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow117 * oods_point));
    total_sum += constraint_coefficients[179] * value;

    value = (column5 - oods_values[180])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow118 * oods_point));
    total_sum += constraint_coefficients[180] * value;

    value = (column5 - oods_values[181])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow119 * oods_point));
    total_sum += constraint_coefficients[181] * value;

    value = (column5 - oods_values[182])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow120 * oods_point));
    total_sum += constraint_coefficients[182] * value;

    value = (column5 - oods_values[183])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow121 * oods_point));
    total_sum += constraint_coefficients[183] * value;

    value = (column6 - oods_values[184])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[184] * value;

    value = (column6 - oods_values[185])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[185] * value;

    value = (column6 - oods_values[186])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[186] * value;

    value = (column6 - oods_values[187])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow6 * oods_point));
    total_sum += constraint_coefficients[187] * value;

    value = (column7 - oods_values[188])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow0 * oods_point));
    total_sum += constraint_coefficients[188] * value;

    value = (column7 - oods_values[189])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow4 * oods_point));
    total_sum += constraint_coefficients[189] * value;

    value = (column7 - oods_values[190])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow5 * oods_point));
    total_sum += constraint_coefficients[190] * value;

    value = (column7 - oods_values[191])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - pow8 * oods_point));
    total_sum += constraint_coefficients[191] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow(CONSTRAINT_DEGREE);

    value = (column_values[(NUM_COLUMNS_FIRST + NUM_COLUMNS_SECOND) as usize] - oods_values[192])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - oods_point_to_deg));
    total_sum += constraint_coefficients[192] * value;

    value = (column_values[(NUM_COLUMNS_FIRST + NUM_COLUMNS_SECOND + 1) as usize]
        - oods_values[193])
        .field_div(&NonZeroFelt::from_felt_unchecked(point - oods_point_to_deg));
    total_sum += constraint_coefficients[193] * value;

    total_sum
}
