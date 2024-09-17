use crate::{consts::*, felt_nonzero, layout::starknet::GlobalValues};
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
	let pow0 = point.pow_felt(&((global_values.trace_length.floor_div(&felt_nonzero!(FELT_32768)))));
	let pow1 = pow0 * pow0; // pow(point, (safe_div(global_values.trace_length, 16384))).
	let pow2 = point.pow_felt(&((global_values.trace_length.floor_div(&felt_nonzero!(FELT_1024)))));
	let pow3 = pow2 * pow2; // pow(point, (safe_div(global_values.trace_length, 512))).
	let pow4 = pow3 * pow3; // pow(point, (safe_div(global_values.trace_length, 256))).
	let pow5 = pow4 * pow4; // pow(point, (safe_div(global_values.trace_length, 128))).
	let pow6 = pow5 * pow5; // pow(point, (safe_div(global_values.trace_length, 64))).
	let pow7 = point.pow_felt(&((global_values.trace_length.floor_div(&felt_nonzero!(FELT_16)))));
	let pow8 = pow7 * pow7; // pow(point, (safe_div(global_values.trace_length, 8))).
	let pow9 = pow8 * pow8; // pow(point, (safe_div(global_values.trace_length, 4))).
	let pow10 = pow9 * pow9; // pow(point, (safe_div(global_values.trace_length, 2))).
	let pow11 = pow10 * pow10; // pow(point, global_values.trace_length).
	let pow12 = trace_generator.pow_felt(&(global_values.trace_length - FELT_16384));
	let pow13 = trace_generator.pow_felt(&(global_values.trace_length - FELT_1024));
	let pow14 = trace_generator.pow_felt(&(global_values.trace_length - FELT_32768));
	let pow15 = trace_generator.pow_felt(&(global_values.trace_length - FELT_256));
	let pow16 = trace_generator.pow_felt(&(global_values.trace_length - FELT_512));
	let pow17 = trace_generator.pow_felt(&(global_values.trace_length - FELT_8));
	let pow18 = trace_generator.pow_felt(&(global_values.trace_length - FELT_4));
	let pow19 = trace_generator.pow_felt(&(global_values.trace_length - FELT_2));
	let pow20 = trace_generator.pow_felt(&(global_values.trace_length - FELT_16));
	let pow21 = trace_generator.pow_felt(&(((FELT_251 * global_values.trace_length).floor_div(&felt_nonzero!(FELT_256)))));
	let pow22 = trace_generator.pow_felt(&((global_values.trace_length.floor_div(&felt_nonzero!(FELT_64)))));
	let pow23 = pow22 * pow22; // pow(trace_generator, (safe_div(global_values.trace_length, 32))).
	let pow24 = pow22 * pow23; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 64))).
	let pow25 = pow22 * pow24; // pow(trace_generator, (safe_div(global_values.trace_length, 16))).
	let pow26 = pow22 * pow25; // pow(trace_generator, (safe_div((safe_mult(5, global_values.trace_length)), 64))).
	let pow27 = pow22 * pow26; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 32))).
	let pow28 = pow22 * pow27; // pow(trace_generator, (safe_div((safe_mult(7, global_values.trace_length)), 64))).
	let pow29 = pow22 * pow28; // pow(trace_generator, (safe_div(global_values.trace_length, 8))).
	let pow30 = pow22 * pow29; // pow(trace_generator, (safe_div((safe_mult(9, global_values.trace_length)), 64))).
	let pow31 = pow22 * pow30; // pow(trace_generator, (safe_div((safe_mult(5, global_values.trace_length)), 32))).
	let pow32 = pow22 * pow31; // pow(trace_generator, (safe_div((safe_mult(11, global_values.trace_length)), 64))).
	let pow33 = pow22 * pow32; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 16))).
	let pow34 = pow22 * pow33; // pow(trace_generator, (safe_div((safe_mult(13, global_values.trace_length)), 64))).
	let pow35 = pow22 * pow34; // pow(trace_generator, (safe_div((safe_mult(7, global_values.trace_length)), 32))).
	let pow36 = pow22 * pow35; // pow(trace_generator, (safe_div((safe_mult(15, global_values.trace_length)), 64))).
	let pow37 = trace_generator.pow_felt(&((global_values.trace_length.floor_div(&felt_nonzero!(FELT_2)))));
	let pow38 = pow27 * pow37; // pow(trace_generator, (safe_div((safe_mult(19, global_values.trace_length)), 32))).
	let pow39 = pow23 * pow38; // pow(trace_generator, (safe_div((safe_mult(5, global_values.trace_length)), 8))).
	let pow40 = pow23 * pow39; // pow(trace_generator, (safe_div((safe_mult(21, global_values.trace_length)), 32))).
	let pow41 = pow23 * pow40; // pow(trace_generator, (safe_div((safe_mult(11, global_values.trace_length)), 16))).
	let pow42 = pow23 * pow41; // pow(trace_generator, (safe_div((safe_mult(23, global_values.trace_length)), 32))).
	let pow43 = pow23 * pow42; // pow(trace_generator, (safe_div((safe_mult(3, global_values.trace_length)), 4))).
	let pow44 = pow23 * pow43; // pow(trace_generator, (safe_div((safe_mult(25, global_values.trace_length)), 32))).
	let pow45 = pow23 * pow44; // pow(trace_generator, (safe_div((safe_mult(13, global_values.trace_length)), 16))).
	let pow46 = pow23 * pow45; // pow(trace_generator, (safe_div((safe_mult(27, global_values.trace_length)), 32))).
	let pow47 = pow23 * pow46; // pow(trace_generator, (safe_div((safe_mult(7, global_values.trace_length)), 8))).
	let pow48 = pow23 * pow47; // pow(trace_generator, (safe_div((safe_mult(29, global_values.trace_length)), 32))).
	let pow49 = pow21 * pow22; // pow(trace_generator, (safe_div((safe_mult(255, global_values.trace_length)), 256))).
	let pow50 = pow23 * pow48; // pow(trace_generator, (safe_div((safe_mult(15, global_values.trace_length)), 16))).
	let pow51 = pow22 * pow50; // pow(trace_generator, (safe_div((safe_mult(61, global_values.trace_length)), 64))).
	let pow52 = pow22 * pow51; // pow(trace_generator, (safe_div((safe_mult(31, global_values.trace_length)), 32))).
	let pow53 = pow22 * pow52; // pow(trace_generator, (safe_div((safe_mult(63, global_values.trace_length)), 64))).
	
	// Compute domains.
	let domain0 = pow11 - FELT_1;
	let domain1 = pow10 - FELT_1;
	let domain2 = pow9 - FELT_1;
	let domain3 = pow8 - FELT_1;
	let domain4 = pow7 - pow50;
	let domain5 = pow7 - FELT_1;
	let domain6 = pow6 - FELT_1;
	let domain7 = pow5 - FELT_1;
	let domain8 = pow4 - FELT_1;
	let domain9 = pow4 - pow49;
	let domain10 = pow4 - pow53;
	let domain11 = pow4 - pow43;
	let domain12 = pow3 - pow37;
	let domain13 = pow3 - FELT_1;
	let domain14 = pow3 - pow52;
	let temp = pow3 - pow41;
	let temp = temp * (pow3 - pow42);
	let temp = temp * (pow3 - pow43);
	let temp = temp * (pow3 - pow44);
	let temp = temp * (pow3 - pow45);
	let temp = temp * (pow3 - pow46);
	let temp = temp * (pow3 - pow47);
	let temp = temp * (pow3 - pow48);
	let temp = temp * (pow3 - pow50);
	let domain15 = temp * (domain14);
	let temp = pow3 - pow51;
	let temp = temp * (pow3 - pow53);
	let domain16 = temp * (domain14);
	let temp = pow3 - pow38;
	let temp = temp * (pow3 - pow39);
	let temp = temp * (pow3 - pow40);
	let domain17 = temp * (domain15);
	let domain18 = pow2 - pow43;
	let domain19 = pow2 - FELT_1;
	let temp = pow2 - pow22;
	let temp = temp * (pow2 - pow23);
	let temp = temp * (pow2 - pow24);
	let temp = temp * (pow2 - pow25);
	let temp = temp * (pow2 - pow26);
	let temp = temp * (pow2 - pow27);
	let temp = temp * (pow2 - pow28);
	let temp = temp * (pow2 - pow29);
	let temp = temp * (pow2 - pow30);
	let temp = temp * (pow2 - pow31);
	let temp = temp * (pow2 - pow32);
	let temp = temp * (pow2 - pow33);
	let temp = temp * (pow2 - pow34);
	let temp = temp * (pow2 - pow35);
	let temp = temp * (pow2 - pow36);
	let domain20 = temp * (domain19);
	let domain21 = pow1 - pow49;
	let domain22 = pow1 - pow21;
	let domain23 = pow1 - FELT_1;
	let domain24 = pow1 - pow53;
	let domain25 = pow0 - pow49;
	let domain26 = pow0 - pow21;
	let domain27 = pow0 - FELT_1;
	let domain28 = point - pow20;
	let domain29 = point - FELT_1;
	let domain30 = point - pow19;
	let domain31 = point - pow18;
	let domain32 = point - pow17;
	let domain33 = point - pow16;
	let domain34 = point - pow15;
	let domain35 = point - pow14;
	let domain36 = point - pow13;
	let domain37 = point - pow12;
	
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
	let column4_row255 = mask_values[35];
	let column5_row0 = mask_values[36];
	let column5_row1 = mask_values[37];
	let column5_row2 = mask_values[38];
	let column5_row3 = mask_values[39];
	let column5_row4 = mask_values[40];
	let column5_row5 = mask_values[41];
	let column5_row6 = mask_values[42];
	let column5_row7 = mask_values[43];
	let column5_row8 = mask_values[44];
	let column5_row9 = mask_values[45];
	let column5_row12 = mask_values[46];
	let column5_row13 = mask_values[47];
	let column5_row16 = mask_values[48];
	let column5_row38 = mask_values[49];
	let column5_row39 = mask_values[50];
	let column5_row70 = mask_values[51];
	let column5_row71 = mask_values[52];
	let column5_row102 = mask_values[53];
	let column5_row103 = mask_values[54];
	let column5_row134 = mask_values[55];
	let column5_row135 = mask_values[56];
	let column5_row166 = mask_values[57];
	let column5_row167 = mask_values[58];
	let column5_row198 = mask_values[59];
	let column5_row199 = mask_values[60];
	let column5_row262 = mask_values[61];
	let column5_row263 = mask_values[62];
	let column5_row294 = mask_values[63];
	let column5_row295 = mask_values[64];
	let column5_row326 = mask_values[65];
	let column5_row358 = mask_values[66];
	let column5_row359 = mask_values[67];
	let column5_row390 = mask_values[68];
	let column5_row391 = mask_values[69];
	let column5_row422 = mask_values[70];
	let column5_row423 = mask_values[71];
	let column5_row454 = mask_values[72];
	let column5_row518 = mask_values[73];
	let column5_row711 = mask_values[74];
	let column5_row902 = mask_values[75];
	let column5_row903 = mask_values[76];
	let column5_row966 = mask_values[77];
	let column5_row967 = mask_values[78];
	let column5_row1222 = mask_values[79];
	let column5_row2438 = mask_values[80];
	let column5_row2439 = mask_values[81];
	let column5_row4486 = mask_values[82];
	let column5_row4487 = mask_values[83];
	let column5_row6534 = mask_values[84];
	let column5_row6535 = mask_values[85];
	let column5_row8582 = mask_values[86];
	let column5_row8583 = mask_values[87];
	let column5_row10630 = mask_values[88];
	let column5_row10631 = mask_values[89];
	let column5_row12678 = mask_values[90];
	let column5_row12679 = mask_values[91];
	let column5_row14726 = mask_values[92];
	let column5_row14727 = mask_values[93];
	let column5_row16774 = mask_values[94];
	let column5_row16775 = mask_values[95];
	let column5_row24966 = mask_values[96];
	let column5_row33158 = mask_values[97];
	let column6_row0 = mask_values[98];
	let column6_row1 = mask_values[99];
	let column6_row2 = mask_values[100];
	let column6_row3 = mask_values[101];
	let column7_row0 = mask_values[102];
	let column7_row1 = mask_values[103];
	let column7_row2 = mask_values[104];
	let column7_row3 = mask_values[105];
	let column7_row4 = mask_values[106];
	let column7_row5 = mask_values[107];
	let column7_row6 = mask_values[108];
	let column7_row7 = mask_values[109];
	let column7_row8 = mask_values[110];
	let column7_row9 = mask_values[111];
	let column7_row11 = mask_values[112];
	let column7_row12 = mask_values[113];
	let column7_row13 = mask_values[114];
	let column7_row15 = mask_values[115];
	let column7_row17 = mask_values[116];
	let column7_row19 = mask_values[117];
	let column7_row23 = mask_values[118];
	let column7_row27 = mask_values[119];
	let column7_row33 = mask_values[120];
	let column7_row44 = mask_values[121];
	let column7_row49 = mask_values[122];
	let column7_row65 = mask_values[123];
	let column7_row76 = mask_values[124];
	let column7_row81 = mask_values[125];
	let column7_row97 = mask_values[126];
	let column7_row108 = mask_values[127];
	let column7_row113 = mask_values[128];
	let column7_row129 = mask_values[129];
	let column7_row140 = mask_values[130];
	let column7_row145 = mask_values[131];
	let column7_row161 = mask_values[132];
	let column7_row172 = mask_values[133];
	let column7_row177 = mask_values[134];
	let column7_row193 = mask_values[135];
	let column7_row204 = mask_values[136];
	let column7_row209 = mask_values[137];
	let column7_row225 = mask_values[138];
	let column7_row236 = mask_values[139];
	let column7_row241 = mask_values[140];
	let column7_row257 = mask_values[141];
	let column7_row265 = mask_values[142];
	let column7_row491 = mask_values[143];
	let column7_row499 = mask_values[144];
	let column7_row507 = mask_values[145];
	let column7_row513 = mask_values[146];
	let column7_row521 = mask_values[147];
	let column7_row705 = mask_values[148];
	let column7_row721 = mask_values[149];
	let column7_row737 = mask_values[150];
	let column7_row753 = mask_values[151];
	let column7_row769 = mask_values[152];
	let column7_row777 = mask_values[153];
	let column7_row961 = mask_values[154];
	let column7_row977 = mask_values[155];
	let column7_row993 = mask_values[156];
	let column7_row1009 = mask_values[157];
	let column8_row0 = mask_values[158];
	let column8_row1 = mask_values[159];
	let column8_row2 = mask_values[160];
	let column8_row3 = mask_values[161];
	let column8_row4 = mask_values[162];
	let column8_row5 = mask_values[163];
	let column8_row6 = mask_values[164];
	let column8_row7 = mask_values[165];
	let column8_row8 = mask_values[166];
	let column8_row9 = mask_values[167];
	let column8_row10 = mask_values[168];
	let column8_row11 = mask_values[169];
	let column8_row12 = mask_values[170];
	let column8_row13 = mask_values[171];
	let column8_row14 = mask_values[172];
	let column8_row16 = mask_values[173];
	let column8_row17 = mask_values[174];
	let column8_row19 = mask_values[175];
	let column8_row21 = mask_values[176];
	let column8_row22 = mask_values[177];
	let column8_row24 = mask_values[178];
	let column8_row25 = mask_values[179];
	let column8_row27 = mask_values[180];
	let column8_row29 = mask_values[181];
	let column8_row30 = mask_values[182];
	let column8_row33 = mask_values[183];
	let column8_row35 = mask_values[184];
	let column8_row37 = mask_values[185];
	let column8_row38 = mask_values[186];
	let column8_row41 = mask_values[187];
	let column8_row43 = mask_values[188];
	let column8_row45 = mask_values[189];
	let column8_row46 = mask_values[190];
	let column8_row49 = mask_values[191];
	let column8_row51 = mask_values[192];
	let column8_row53 = mask_values[193];
	let column8_row54 = mask_values[194];
	let column8_row57 = mask_values[195];
	let column8_row59 = mask_values[196];
	let column8_row61 = mask_values[197];
	let column8_row65 = mask_values[198];
	let column8_row69 = mask_values[199];
	let column8_row71 = mask_values[200];
	let column8_row73 = mask_values[201];
	let column8_row77 = mask_values[202];
	let column8_row81 = mask_values[203];
	let column8_row85 = mask_values[204];
	let column8_row89 = mask_values[205];
	let column8_row91 = mask_values[206];
	let column8_row97 = mask_values[207];
	let column8_row101 = mask_values[208];
	let column8_row105 = mask_values[209];
	let column8_row109 = mask_values[210];
	let column8_row113 = mask_values[211];
	let column8_row117 = mask_values[212];
	let column8_row123 = mask_values[213];
	let column8_row155 = mask_values[214];
	let column8_row187 = mask_values[215];
	let column8_row195 = mask_values[216];
	let column8_row205 = mask_values[217];
	let column8_row219 = mask_values[218];
	let column8_row221 = mask_values[219];
	let column8_row237 = mask_values[220];
	let column8_row245 = mask_values[221];
	let column8_row253 = mask_values[222];
	let column8_row269 = mask_values[223];
	let column8_row301 = mask_values[224];
	let column8_row309 = mask_values[225];
	let column8_row310 = mask_values[226];
	let column8_row318 = mask_values[227];
	let column8_row326 = mask_values[228];
	let column8_row334 = mask_values[229];
	let column8_row342 = mask_values[230];
	let column8_row350 = mask_values[231];
	let column8_row451 = mask_values[232];
	let column8_row461 = mask_values[233];
	let column8_row477 = mask_values[234];
	let column8_row493 = mask_values[235];
	let column8_row501 = mask_values[236];
	let column8_row509 = mask_values[237];
	let column8_row12309 = mask_values[238];
	let column8_row12373 = mask_values[239];
	let column8_row12565 = mask_values[240];
	let column8_row12629 = mask_values[241];
	let column8_row16085 = mask_values[242];
	let column8_row16149 = mask_values[243];
	let column8_row16325 = mask_values[244];
	let column8_row16331 = mask_values[245];
	let column8_row16337 = mask_values[246];
	let column8_row16339 = mask_values[247];
	let column8_row16355 = mask_values[248];
	let column8_row16357 = mask_values[249];
	let column8_row16363 = mask_values[250];
	let column8_row16369 = mask_values[251];
	let column8_row16371 = mask_values[252];
	let column8_row16385 = mask_values[253];
	let column8_row16417 = mask_values[254];
	let column8_row32647 = mask_values[255];
	let column8_row32667 = mask_values[256];
	let column8_row32715 = mask_values[257];
	let column8_row32721 = mask_values[258];
	let column8_row32731 = mask_values[259];
	let column8_row32747 = mask_values[260];
	let column8_row32753 = mask_values[261];
	let column8_row32763 = mask_values[262];
	let column9_inter1_row0 = mask_values[263];
	let column9_inter1_row1 = mask_values[264];
	let column9_inter1_row2 = mask_values[265];
	let column9_inter1_row3 = mask_values[266];
	let column9_inter1_row5 = mask_values[267];
	let column9_inter1_row7 = mask_values[268];
	let column9_inter1_row11 = mask_values[269];
	let column9_inter1_row15 = mask_values[270];
	
	// Compute intermediate values.
	let cpu_decode_opcode_range_check_bit_0 = column0_row0 - (column0_row1 + column0_row1);
	let cpu_decode_opcode_range_check_bit_2 = column0_row2 - (column0_row3 + column0_row3);
	let cpu_decode_opcode_range_check_bit_4 = column0_row4 - (column0_row5 + column0_row5);
	let cpu_decode_opcode_range_check_bit_3 = column0_row3 - (column0_row4 + column0_row4);
	let cpu_decode_flag_op1_base_op0_0 = FELT_1 - (cpu_decode_opcode_range_check_bit_2 + cpu_decode_opcode_range_check_bit_4 + cpu_decode_opcode_range_check_bit_3);
	let cpu_decode_opcode_range_check_bit_5 = column0_row5 - (column0_row6 + column0_row6);
	let cpu_decode_opcode_range_check_bit_6 = column0_row6 - (column0_row7 + column0_row7);
	let cpu_decode_opcode_range_check_bit_9 = column0_row9 - (column0_row10 + column0_row10);
	let cpu_decode_flag_res_op1_0 = FELT_1 - (cpu_decode_opcode_range_check_bit_5 + cpu_decode_opcode_range_check_bit_6 + cpu_decode_opcode_range_check_bit_9);
	let cpu_decode_opcode_range_check_bit_7 = column0_row7 - (column0_row8 + column0_row8);
	let cpu_decode_opcode_range_check_bit_8 = column0_row8 - (column0_row9 + column0_row9);
	let cpu_decode_flag_pc_update_regular_0 = FELT_1 - (cpu_decode_opcode_range_check_bit_7 + cpu_decode_opcode_range_check_bit_8 + cpu_decode_opcode_range_check_bit_9);
	let cpu_decode_opcode_range_check_bit_12 = column0_row12 - (column0_row13 + column0_row13);
	let cpu_decode_opcode_range_check_bit_13 = column0_row13 - (column0_row14 + column0_row14);
	let cpu_decode_fp_update_regular_0 = FELT_1 - (cpu_decode_opcode_range_check_bit_12 + cpu_decode_opcode_range_check_bit_13);
	let cpu_decode_opcode_range_check_bit_1 = column0_row1 - (column0_row2 + column0_row2);
	let npc_reg_0 = column5_row0 + cpu_decode_opcode_range_check_bit_2 + FELT_1;
	let cpu_decode_opcode_range_check_bit_10 = column0_row10 - (column0_row11 + column0_row11);
	let cpu_decode_opcode_range_check_bit_11 = column0_row11 - (column0_row12 + column0_row12);
	let cpu_decode_opcode_range_check_bit_14 = column0_row14 - (column0_row15 + column0_row15);
	let memory_address_diff_0 = column6_row2 - column6_row0;
	let range_check16_diff_0 = column7_row6 - column7_row2;
	let pedersen_hash0_ec_subset_sum_bit_0 = column3_row0 - (column3_row1 + column3_row1);
	let pedersen_hash0_ec_subset_sum_bit_neg_0 = FELT_1 - pedersen_hash0_ec_subset_sum_bit_0;
	let range_check_builtin_value0_0 = column7_row12;
	let range_check_builtin_value1_0 = range_check_builtin_value0_0 * global_values.offset_size + column7_row44;
	let range_check_builtin_value2_0 = range_check_builtin_value1_0 * global_values.offset_size + column7_row76;
	let range_check_builtin_value3_0 = range_check_builtin_value2_0 * global_values.offset_size + column7_row108;
	let range_check_builtin_value4_0 = range_check_builtin_value3_0 * global_values.offset_size + column7_row140;
	let range_check_builtin_value5_0 = range_check_builtin_value4_0 * global_values.offset_size + column7_row172;
	let range_check_builtin_value6_0 = range_check_builtin_value5_0 * global_values.offset_size + column7_row204;
	let range_check_builtin_value7_0 = range_check_builtin_value6_0 * global_values.offset_size + column7_row236;
	let ecdsa_signature0_doubling_key_x_squared = column8_row1 * column8_row1;
	let ecdsa_signature0_exponentiate_generator_bit_0 = column8_row59 - (column8_row187 + column8_row187);
	let ecdsa_signature0_exponentiate_generator_bit_neg_0 = FELT_1 - ecdsa_signature0_exponentiate_generator_bit_0;
	let ecdsa_signature0_exponentiate_key_bit_0 = column8_row9 - (column8_row73 + column8_row73);
	let ecdsa_signature0_exponentiate_key_bit_neg_0 = FELT_1 - ecdsa_signature0_exponentiate_key_bit_0;
	let bitwise_sum_var_0_0 = column7_row1 + column7_row17 * FELT_2 + column7_row33 * FELT_4 + column7_row49 * FELT_8 + column7_row65 * FELT_18446744073709551616 + column7_row81 * FELT_36893488147419103232 + column7_row97 * FELT_73786976294838206464 + column7_row113 * FELT_147573952589676412928;
	let bitwise_sum_var_8_0 = column7_row129 * FELT_340282366920938463463374607431768211456 + column7_row145 * FELT_680564733841876926926749214863536422912 + column7_row161 * FELT_1361129467683753853853498429727072845824 + column7_row177 * FELT_2722258935367507707706996859454145691648 + column7_row193 * FELT_6277101735386680763835789423207666416102355444464034512896 + column7_row209 * FELT_12554203470773361527671578846415332832204710888928069025792 + column7_row225 * FELT_25108406941546723055343157692830665664409421777856138051584 + column7_row241 * FELT_50216813883093446110686315385661331328818843555712276103168;
	let ec_op_doubling_q_x_squared_0 = column8_row41 * column8_row41;
	let ec_op_ec_subset_sum_bit_0 = column8_row21 - (column8_row85 + column8_row85);
	let ec_op_ec_subset_sum_bit_neg_0 = FELT_1 - ec_op_ec_subset_sum_bit_0;
	let poseidon_poseidon_full_rounds_state0_cubed_0 = column8_row53 * column8_row29;
	let poseidon_poseidon_full_rounds_state1_cubed_0 = column8_row13 * column8_row61;
	let poseidon_poseidon_full_rounds_state2_cubed_0 = column8_row45 * column8_row3;
	let poseidon_poseidon_full_rounds_state0_cubed_7 = column8_row501 * column8_row477;
	let poseidon_poseidon_full_rounds_state1_cubed_7 = column8_row461 * column8_row509;
	let poseidon_poseidon_full_rounds_state2_cubed_7 = column8_row493 * column8_row451;
	let poseidon_poseidon_full_rounds_state0_cubed_3 = column8_row245 * column8_row221;
	let poseidon_poseidon_full_rounds_state1_cubed_3 = column8_row205 * column8_row253;
	let poseidon_poseidon_full_rounds_state2_cubed_3 = column8_row237 * column8_row195;
	let poseidon_poseidon_partial_rounds_state0_cubed_0 = column7_row3 * column7_row7;
	let poseidon_poseidon_partial_rounds_state0_cubed_1 = column7_row11 * column7_row15;
	let poseidon_poseidon_partial_rounds_state0_cubed_2 = column7_row19 * column7_row23;
	let poseidon_poseidon_partial_rounds_state1_cubed_0 = column8_row6 * column8_row14;
	let poseidon_poseidon_partial_rounds_state1_cubed_1 = column8_row22 * column8_row30;
	let poseidon_poseidon_partial_rounds_state1_cubed_2 = column8_row38 * column8_row46;
	let poseidon_poseidon_partial_rounds_state1_cubed_19 = column8_row310 * column8_row318;
	let poseidon_poseidon_partial_rounds_state1_cubed_20 = column8_row326 * column8_row334;
	let poseidon_poseidon_partial_rounds_state1_cubed_21 = column8_row342 * column8_row350;
	
	// Sum constraints.
	let total_sum = FELT_0;
	
	// Constraint: cpu/decode/opcode_range_check/bit.
	let value = (cpu_decode_opcode_range_check_bit_0 * cpu_decode_opcode_range_check_bit_0 - cpu_decode_opcode_range_check_bit_0) * domain4.field_div(&felt_nonzero!(domain0));
	let total_sum = total_sum + constraint_coefficients[0] * value;
	
	// Constraint: cpu/decode/opcode_range_check/zero.
	let value = (column0_row0).field_div(&felt_nonzero!(domain4));
	let total_sum = total_sum + constraint_coefficients[1] * value;
	
	// Constraint: cpu/decode/opcode_range_check_input.
	let value = (column5_row1 - (((column0_row0 * global_values.offset_size + column7_row4) * global_values.offset_size + column7_row8) * global_values.offset_size + column7_row0)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[2] * value;
	
	// Constraint: cpu/decode/flag_op1_base_op0_bit.
	let value = (cpu_decode_flag_op1_base_op0_0 * cpu_decode_flag_op1_base_op0_0 - cpu_decode_flag_op1_base_op0_0).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[3] * value;
	
	// Constraint: cpu/decode/flag_res_op1_bit.
	let value = (cpu_decode_flag_res_op1_0 * cpu_decode_flag_res_op1_0 - cpu_decode_flag_res_op1_0).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[4] * value;
	
	// Constraint: cpu/decode/flag_pc_update_regular_bit.
	let value = (cpu_decode_flag_pc_update_regular_0 * cpu_decode_flag_pc_update_regular_0 - cpu_decode_flag_pc_update_regular_0).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[5] * value;
	
	// Constraint: cpu/decode/fp_update_regular_bit.
	let value = (cpu_decode_fp_update_regular_0 * cpu_decode_fp_update_regular_0 - cpu_decode_fp_update_regular_0).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[6] * value;
	
	// Constraint: cpu/operands/mem_dst_addr.
	let value = (column5_row8 + global_values.half_offset_size - (cpu_decode_opcode_range_check_bit_0 * column8_row8 + (FELT_1 - cpu_decode_opcode_range_check_bit_0) * column8_row0 + column7_row0)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[7] * value;
	
	// Constraint: cpu/operands/mem0_addr.
	let value = (column5_row4 + global_values.half_offset_size - (cpu_decode_opcode_range_check_bit_1 * column8_row8 + (FELT_1 - cpu_decode_opcode_range_check_bit_1) * column8_row0 + column7_row8)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[8] * value;
	
	// Constraint: cpu/operands/mem1_addr.
	let value = (column5_row12 + global_values.half_offset_size - (cpu_decode_opcode_range_check_bit_2 * column5_row0 + cpu_decode_opcode_range_check_bit_4 * column8_row0 + cpu_decode_opcode_range_check_bit_3 * column8_row8 + cpu_decode_flag_op1_base_op0_0 * column5_row5 + column7_row4)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[9] * value;
	
	// Constraint: cpu/operands/ops_mul.
	let value = (column8_row4 - column5_row5 * column5_row13).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[10] * value;
	
	// Constraint: cpu/operands/res.
	let value = ((FELT_1 - cpu_decode_opcode_range_check_bit_9) * column8_row12 - (cpu_decode_opcode_range_check_bit_5 * (column5_row5 + column5_row13) + cpu_decode_opcode_range_check_bit_6 * column8_row4 + cpu_decode_flag_res_op1_0 * column5_row13)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[11] * value;
	
	// Constraint: cpu/update_registers/update_pc/tmp0.
	let value = (column8_row2 - cpu_decode_opcode_range_check_bit_9 * column5_row9) * domain28.field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[12] * value;
	
	// Constraint: cpu/update_registers/update_pc/tmp1.
	let value = (column8_row10 - column8_row2 * column8_row12) * domain28.field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[13] * value;
	
	// Constraint: cpu/update_registers/update_pc/pc_cond_negative.
	let value = ((FELT_1 - cpu_decode_opcode_range_check_bit_9) * column5_row16 + column8_row2 * (column5_row16 - (column5_row0 + column5_row13)) - (cpu_decode_flag_pc_update_regular_0 * npc_reg_0 + cpu_decode_opcode_range_check_bit_7 * column8_row12 + cpu_decode_opcode_range_check_bit_8 * (column5_row0 + column8_row12))) * domain28.field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[14] * value;
	
	// Constraint: cpu/update_registers/update_pc/pc_cond_positive.
	let value = ((column8_row10 - cpu_decode_opcode_range_check_bit_9) * (column5_row16 - npc_reg_0)) * domain28.field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[15] * value;
	
	// Constraint: cpu/update_registers/update_ap/ap_update.
	let value = (column8_row16 - (column8_row0 + cpu_decode_opcode_range_check_bit_10 * column8_row12 + cpu_decode_opcode_range_check_bit_11 + cpu_decode_opcode_range_check_bit_12 * FELT_2)) * domain28.field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[16] * value;
	
	// Constraint: cpu/update_registers/update_fp/fp_update.
	let value = (column8_row24 - (cpu_decode_fp_update_regular_0 * column8_row8 + cpu_decode_opcode_range_check_bit_13 * column5_row9 + cpu_decode_opcode_range_check_bit_12 * (column8_row0 + FELT_2))) * domain28.field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[17] * value;
	
	// Constraint: cpu/opcodes/call/push_fp.
	let value = (cpu_decode_opcode_range_check_bit_12 * (column5_row9 - column8_row8)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[18] * value;
	
	// Constraint: cpu/opcodes/call/push_pc.
	let value = (cpu_decode_opcode_range_check_bit_12 * (column5_row5 - (column5_row0 + cpu_decode_opcode_range_check_bit_2 + FELT_1))).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[19] * value;
	
	// Constraint: cpu/opcodes/call/off0.
	let value = (cpu_decode_opcode_range_check_bit_12 * (column7_row0 - global_values.half_offset_size)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[20] * value;
	
	// Constraint: cpu/opcodes/call/off1.
	let value = (cpu_decode_opcode_range_check_bit_12 * (column7_row8 - (global_values.half_offset_size + FELT_1))).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[21] * value;
	
	// Constraint: cpu/opcodes/call/flags.
	let value = (cpu_decode_opcode_range_check_bit_12 * (cpu_decode_opcode_range_check_bit_12 + cpu_decode_opcode_range_check_bit_12 + FELT_1 + FELT_1 - (cpu_decode_opcode_range_check_bit_0 + cpu_decode_opcode_range_check_bit_1 + FELT_4))).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[22] * value;
	
	// Constraint: cpu/opcodes/ret/off0.
	let value = (cpu_decode_opcode_range_check_bit_13 * (column7_row0 + FELT_2 - global_values.half_offset_size)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[23] * value;
	
	// Constraint: cpu/opcodes/ret/off2.
	let value = (cpu_decode_opcode_range_check_bit_13 * (column7_row4 + FELT_1 - global_values.half_offset_size)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[24] * value;
	
	// Constraint: cpu/opcodes/ret/flags.
	let value = (cpu_decode_opcode_range_check_bit_13 * (cpu_decode_opcode_range_check_bit_7 + cpu_decode_opcode_range_check_bit_0 + cpu_decode_opcode_range_check_bit_3 + cpu_decode_flag_res_op1_0 - FELT_4)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[25] * value;
	
	// Constraint: cpu/opcodes/assert_eq/assert_eq.
	let value = (cpu_decode_opcode_range_check_bit_14 * (column5_row9 - column8_row12)).field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[26] * value;
	
	// Constraint: initial_ap.
	let value = (column8_row0 - global_values.initial_ap).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[27] * value;
	
	// Constraint: initial_fp.
	let value = (column8_row8 - global_values.initial_ap).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[28] * value;
	
	// Constraint: initial_pc.
	let value = (column5_row0 - global_values.initial_pc).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[29] * value;
	
	// Constraint: final_ap.
	let value = (column8_row0 - global_values.final_ap).field_div(&felt_nonzero!(domain28));
	let total_sum = total_sum + constraint_coefficients[30] * value;
	
	// Constraint: final_fp.
	let value = (column8_row8 - global_values.initial_ap).field_div(&felt_nonzero!(domain28));
	let total_sum = total_sum + constraint_coefficients[31] * value;
	
	// Constraint: final_pc.
	let value = (column5_row0 - global_values.final_pc).field_div(&felt_nonzero!(domain28));
	let total_sum = total_sum + constraint_coefficients[32] * value;
	
	// Constraint: memory/multi_column_perm/perm/init0.
	let value = ((global_values.memory_multi_column_perm_perm_interaction_elm - (column6_row0 + global_values.memory_multi_column_perm_hash_interaction_elm0 * column6_row1)) * column9_inter1_row0 + column5_row0 + global_values.memory_multi_column_perm_hash_interaction_elm0 * column5_row1 - global_values.memory_multi_column_perm_perm_interaction_elm).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[33] * value;
	
	// Constraint: memory/multi_column_perm/perm/step0.
	let value = ((global_values.memory_multi_column_perm_perm_interaction_elm - (column6_row2 + global_values.memory_multi_column_perm_hash_interaction_elm0 * column6_row3)) * column9_inter1_row2 - (global_values.memory_multi_column_perm_perm_interaction_elm - (column5_row2 + global_values.memory_multi_column_perm_hash_interaction_elm0 * column5_row3)) * column9_inter1_row0) * domain30.field_div(&felt_nonzero!(domain1));
	let total_sum = total_sum + constraint_coefficients[34] * value;
	
	// Constraint: memory/multi_column_perm/perm/last.
	let value = (column9_inter1_row0 - global_values.memory_multi_column_perm_perm_public_memory_prod).field_div(&felt_nonzero!(domain30));
	let total_sum = total_sum + constraint_coefficients[35] * value;
	
	// Constraint: memory/diff_is_bit.
	let value = (memory_address_diff_0 * memory_address_diff_0 - memory_address_diff_0) * domain30.field_div(&felt_nonzero!(domain1));
	let total_sum = total_sum + constraint_coefficients[36] * value;
	
	// Constraint: memory/is_func.
	let value = ((memory_address_diff_0 - FELT_1) * (column6_row1 - column6_row3)) * domain30.field_div(&felt_nonzero!(domain1));
	let total_sum = total_sum + constraint_coefficients[37] * value;
	
	// Constraint: memory/initial_addr.
	let value = (column6_row0 - FELT_1).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[38] * value;
	
	// Constraint: public_memory_addr_zero.
	let value = (column5_row2).field_div(&felt_nonzero!(domain3));
	let total_sum = total_sum + constraint_coefficients[39] * value;
	
	// Constraint: public_memory_value_zero.
	let value = (column5_row3).field_div(&felt_nonzero!(domain3));
	let total_sum = total_sum + constraint_coefficients[40] * value;
	
	// Constraint: range_check16/perm/init0.
	let value = ((global_values.range_check16_perm_interaction_elm - column7_row2) * column9_inter1_row1 + column7_row0 - global_values.range_check16_perm_interaction_elm).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[41] * value;
	
	// Constraint: range_check16/perm/step0.
	let value = ((global_values.range_check16_perm_interaction_elm - column7_row6) * column9_inter1_row5 - (global_values.range_check16_perm_interaction_elm - column7_row4) * column9_inter1_row1) * domain31.field_div(&felt_nonzero!(domain2));
	let total_sum = total_sum + constraint_coefficients[42] * value;
	
	// Constraint: range_check16/perm/last.
	let value = (column9_inter1_row1 - global_values.range_check16_perm_public_memory_prod).field_div(&felt_nonzero!(domain31));
	let total_sum = total_sum + constraint_coefficients[43] * value;
	
	// Constraint: range_check16/diff_is_bit.
	let value = (range_check16_diff_0 * range_check16_diff_0 - range_check16_diff_0) * domain31.field_div(&felt_nonzero!(domain2));
	let total_sum = total_sum + constraint_coefficients[44] * value;
	
	// Constraint: range_check16/minimum.
	let value = (column7_row2 - global_values.range_check_min).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[45] * value;
	
	// Constraint: range_check16/maximum.
	let value = (column7_row2 - global_values.range_check_max).field_div(&felt_nonzero!(domain31));
	let total_sum = total_sum + constraint_coefficients[46] * value;
	
	// Constraint: diluted_check/permutation/init0.
	let value = ((global_values.diluted_check_permutation_interaction_elm - column7_row5) * column9_inter1_row7 + column7_row1 - global_values.diluted_check_permutation_interaction_elm).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[47] * value;
	
	// Constraint: diluted_check/permutation/step0.
	let value = ((global_values.diluted_check_permutation_interaction_elm - column7_row13) * column9_inter1_row15 - (global_values.diluted_check_permutation_interaction_elm - column7_row9) * column9_inter1_row7) * domain32.field_div(&felt_nonzero!(domain3));
	let total_sum = total_sum + constraint_coefficients[48] * value;
	
	// Constraint: diluted_check/permutation/last.
	let value = (column9_inter1_row7 - global_values.diluted_check_permutation_public_memory_prod).field_div(&felt_nonzero!(domain32));
	let total_sum = total_sum + constraint_coefficients[49] * value;
	
	// Constraint: diluted_check/init.
	let value = (column9_inter1_row3 - FELT_1).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[50] * value;
	
	// Constraint: diluted_check/first_element.
	let value = (column7_row5 - global_values.diluted_check_first_elm).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[51] * value;
	
	// Constraint: diluted_check/step.
	let value = (column9_inter1_row11 - (column9_inter1_row3 * (FELT_1 + global_values.diluted_check_interaction_z * (column7_row13 - column7_row5)) + global_values.diluted_check_interaction_alpha * (column7_row13 - column7_row5) * (column7_row13 - column7_row5))) * domain32.field_div(&felt_nonzero!(domain3));
	let total_sum = total_sum + constraint_coefficients[52] * value;
	
	// Constraint: diluted_check/last.
	let value = (column9_inter1_row3 - global_values.diluted_check_final_cum_val).field_div(&felt_nonzero!(domain32));
	let total_sum = total_sum + constraint_coefficients[53] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/last_one_is_zero.
	let value = (column8_row71 * (column3_row0 - (column3_row1 + column3_row1))).field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[54] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
	let value = (column8_row71 * (column3_row1 - FELT_3138550867693340381917894711603833208051177722232017256448 * column3_row192)).field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[55] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit192.
	let value = (column8_row71 - column4_row255 * (column3_row192 - (column3_row193 + column3_row193))).field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[56] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
	let value = (column4_row255 * (column3_row193 - FELT_8 * column3_row196)).field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[57] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit196.
	let value = (column4_row255 - (column3_row251 - (column3_row252 + column3_row252)) * (column3_row196 - (column3_row197 + column3_row197))).field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[58] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
	let value = ((column3_row251 - (column3_row252 + column3_row252)) * (column3_row197 - FELT_18014398509481984 * column3_row251)).field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[59] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/booleanity_test.
	let value = (pedersen_hash0_ec_subset_sum_bit_0 * (pedersen_hash0_ec_subset_sum_bit_0 - FELT_1)) * domain9.field_div(&felt_nonzero!(domain0));
	let total_sum = total_sum + constraint_coefficients[60] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/bit_extraction_end.
	let value = (column3_row0).field_div(&felt_nonzero!(domain10));
	let total_sum = total_sum + constraint_coefficients[61] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/zeros_tail.
	let value = (column3_row0).field_div(&felt_nonzero!(domain9));
	let total_sum = total_sum + constraint_coefficients[62] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/add_points/slope.
	let value = (pedersen_hash0_ec_subset_sum_bit_0 * (column2_row0 - global_values.pedersen_points_y) - column4_row0 * (column1_row0 - global_values.pedersen_points_x)) * domain9.field_div(&felt_nonzero!(domain0));
	let total_sum = total_sum + constraint_coefficients[63] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/add_points/x.
	let value = (column4_row0 * column4_row0 - pedersen_hash0_ec_subset_sum_bit_0 * (column1_row0 + global_values.pedersen_points_x + column1_row1)) * domain9.field_div(&felt_nonzero!(domain0));
	let total_sum = total_sum + constraint_coefficients[64] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/add_points/y.
	let value = (pedersen_hash0_ec_subset_sum_bit_0 * (column2_row0 + column2_row1) - column4_row0 * (column1_row0 - column1_row1)) * domain9.field_div(&felt_nonzero!(domain0));
	let total_sum = total_sum + constraint_coefficients[65] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/copy_point/x.
	let value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column1_row1 - column1_row0)) * domain9.field_div(&felt_nonzero!(domain0));
	let total_sum = total_sum + constraint_coefficients[66] * value;
	
	// Constraint: pedersen/hash0/ec_subset_sum/copy_point/y.
	let value = (pedersen_hash0_ec_subset_sum_bit_neg_0 * (column2_row1 - column2_row0)) * domain9.field_div(&felt_nonzero!(domain0));
	let total_sum = total_sum + constraint_coefficients[67] * value;
	
	// Constraint: pedersen/hash0/copy_point/x.
	let value = (column1_row256 - column1_row255) * domain12.field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[68] * value;
	
	// Constraint: pedersen/hash0/copy_point/y.
	let value = (column2_row256 - column2_row255) * domain12.field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[69] * value;
	
	// Constraint: pedersen/hash0/init/x.
	let value = (column1_row0 - global_values.pedersen_shift_point.x).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[70] * value;
	
	// Constraint: pedersen/hash0/init/y.
	let value = (column2_row0 - global_values.pedersen_shift_point.y).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[71] * value;
	
	// Constraint: pedersen/input0_value0.
	let value = (column5_row7 - column3_row0).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[72] * value;
	
	// Constraint: pedersen/input0_addr.
	let value = (column5_row518 - (column5_row134 + FELT_1)) * domain33.field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[73] * value;
	
	// Constraint: pedersen/init_addr.
	let value = (column5_row6 - global_values.initial_pedersen_addr).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[74] * value;
	
	// Constraint: pedersen/input1_value0.
	let value = (column5_row263 - column3_row256).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[75] * value;
	
	// Constraint: pedersen/input1_addr.
	let value = (column5_row262 - (column5_row6 + FELT_1)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[76] * value;
	
	// Constraint: pedersen/output_value0.
	let value = (column5_row135 - column1_row511).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[77] * value;
	
	// Constraint: pedersen/output_addr.
	let value = (column5_row134 - (column5_row262 + FELT_1)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[78] * value;
	
	// Constraint: range_check_builtin/value.
	let value = (range_check_builtin_value7_0 - column5_row71).field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[79] * value;
	
	// Constraint: range_check_builtin/addr_step.
	let value = (column5_row326 - (column5_row70 + FELT_1)) * domain34.field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[80] * value;
	
	// Constraint: range_check_builtin/init_addr.
	let value = (column5_row70 - global_values.initial_range_check_addr).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[81] * value;
	
	// Constraint: ecdsa/signature0/doubling_key/slope.
	let value = (ecdsa_signature0_doubling_key_x_squared + ecdsa_signature0_doubling_key_x_squared + ecdsa_signature0_doubling_key_x_squared + global_values.ecdsa_sig_config.alpha - (column8_row33 + column8_row33) * column8_row35) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[82] * value;
	
	// Constraint: ecdsa/signature0/doubling_key/x.
	let value = (column8_row35 * column8_row35 - (column8_row1 + column8_row1 + column8_row65)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[83] * value;
	
	// Constraint: ecdsa/signature0/doubling_key/y.
	let value = (column8_row33 + column8_row97 - column8_row35 * (column8_row1 - column8_row65)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[84] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/booleanity_test.
	let value = (ecdsa_signature0_exponentiate_generator_bit_0 * (ecdsa_signature0_exponentiate_generator_bit_0 - FELT_1)) * domain25.field_div(&felt_nonzero!(domain7));
	let total_sum = total_sum + constraint_coefficients[85] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/bit_extraction_end.
	let value = (column8_row59).field_div(&felt_nonzero!(domain26));
	let total_sum = total_sum + constraint_coefficients[86] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/zeros_tail.
	let value = (column8_row59).field_div(&felt_nonzero!(domain25));
	let total_sum = total_sum + constraint_coefficients[87] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/add_points/slope.
	let value = (ecdsa_signature0_exponentiate_generator_bit_0 * (column8_row91 - global_values.ecdsa_generator_points_y) - column8_row123 * (column8_row27 - global_values.ecdsa_generator_points_x)) * domain25.field_div(&felt_nonzero!(domain7));
	let total_sum = total_sum + constraint_coefficients[88] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/add_points/x.
	let value = (column8_row123 * column8_row123 - ecdsa_signature0_exponentiate_generator_bit_0 * (column8_row27 + global_values.ecdsa_generator_points_x + column8_row155)) * domain25.field_div(&felt_nonzero!(domain7));
	let total_sum = total_sum + constraint_coefficients[89] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/add_points/y.
	let value = (ecdsa_signature0_exponentiate_generator_bit_0 * (column8_row91 + column8_row219) - column8_row123 * (column8_row27 - column8_row155)) * domain25.field_div(&felt_nonzero!(domain7));
	let total_sum = total_sum + constraint_coefficients[90] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/add_points/x_diff_inv.
	let value = (column8_row7 * (column8_row27 - global_values.ecdsa_generator_points_x) - FELT_1) * domain25.field_div(&felt_nonzero!(domain7));
	let total_sum = total_sum + constraint_coefficients[91] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/copy_point/x.
	let value = (ecdsa_signature0_exponentiate_generator_bit_neg_0 * (column8_row155 - column8_row27)) * domain25.field_div(&felt_nonzero!(domain7));
	let total_sum = total_sum + constraint_coefficients[92] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_generator/copy_point/y.
	let value = (ecdsa_signature0_exponentiate_generator_bit_neg_0 * (column8_row219 - column8_row91)) * domain25.field_div(&felt_nonzero!(domain7));
	let total_sum = total_sum + constraint_coefficients[93] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/booleanity_test.
	let value = (ecdsa_signature0_exponentiate_key_bit_0 * (ecdsa_signature0_exponentiate_key_bit_0 - FELT_1)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[94] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/bit_extraction_end.
	let value = (column8_row9).field_div(&felt_nonzero!(domain22));
	let total_sum = total_sum + constraint_coefficients[95] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/zeros_tail.
	let value = (column8_row9).field_div(&felt_nonzero!(domain21));
	let total_sum = total_sum + constraint_coefficients[96] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/add_points/slope.
	let value = (ecdsa_signature0_exponentiate_key_bit_0 * (column8_row49 - column8_row33) - column8_row19 * (column8_row17 - column8_row1)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[97] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/add_points/x.
	let value = (column8_row19 * column8_row19 - ecdsa_signature0_exponentiate_key_bit_0 * (column8_row17 + column8_row1 + column8_row81)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[98] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/add_points/y.
	let value = (ecdsa_signature0_exponentiate_key_bit_0 * (column8_row49 + column8_row113) - column8_row19 * (column8_row17 - column8_row81)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[99] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/add_points/x_diff_inv.
	let value = (column8_row51 * (column8_row17 - column8_row1) - FELT_1) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[100] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/copy_point/x.
	let value = (ecdsa_signature0_exponentiate_key_bit_neg_0 * (column8_row81 - column8_row17)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[101] * value;
	
	// Constraint: ecdsa/signature0/exponentiate_key/copy_point/y.
	let value = (ecdsa_signature0_exponentiate_key_bit_neg_0 * (column8_row113 - column8_row49)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[102] * value;
	
	// Constraint: ecdsa/signature0/init_gen/x.
	let value = (column8_row27 - global_values.ecdsa_sig_config.shift_point.x).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[103] * value;
	
	// Constraint: ecdsa/signature0/init_gen/y.
	let value = (column8_row91 + global_values.ecdsa_sig_config.shift_point.y).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[104] * value;
	
	// Constraint: ecdsa/signature0/init_key/x.
	let value = (column8_row17 - global_values.ecdsa_sig_config.shift_point.x).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[105] * value;
	
	// Constraint: ecdsa/signature0/init_key/y.
	let value = (column8_row49 - global_values.ecdsa_sig_config.shift_point.y).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[106] * value;
	
	// Constraint: ecdsa/signature0/add_results/slope.
	let value = (column8_row32731 - (column8_row16369 + column8_row32763 * (column8_row32667 - column8_row16337))).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[107] * value;
	
	// Constraint: ecdsa/signature0/add_results/x.
	let value = (column8_row32763 * column8_row32763 - (column8_row32667 + column8_row16337 + column8_row16385)).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[108] * value;
	
	// Constraint: ecdsa/signature0/add_results/y.
	let value = (column8_row32731 + column8_row16417 - column8_row32763 * (column8_row32667 - column8_row16385)).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[109] * value;
	
	// Constraint: ecdsa/signature0/add_results/x_diff_inv.
	let value = (column8_row32647 * (column8_row32667 - column8_row16337) - FELT_1).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[110] * value;
	
	// Constraint: ecdsa/signature0/extract_r/slope.
	let value = (column8_row32753 + global_values.ecdsa_sig_config.shift_point.y - column8_row16331 * (column8_row32721 - global_values.ecdsa_sig_config.shift_point.x)).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[111] * value;
	
	// Constraint: ecdsa/signature0/extract_r/x.
	let value = (column8_row16331 * column8_row16331 - (column8_row32721 + global_values.ecdsa_sig_config.shift_point.x + column8_row9)).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[112] * value;
	
	// Constraint: ecdsa/signature0/extract_r/x_diff_inv.
	let value = (column8_row32715 * (column8_row32721 - global_values.ecdsa_sig_config.shift_point.x) - FELT_1).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[113] * value;
	
	// Constraint: ecdsa/signature0/z_nonzero.
	let value = (column8_row59 * column8_row16363 - FELT_1).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[114] * value;
	
	// Constraint: ecdsa/signature0/r_and_w_nonzero.
	let value = (column8_row9 * column8_row16355 - FELT_1).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[115] * value;
	
	// Constraint: ecdsa/signature0/q_on_curve/x_squared.
	let value = (column8_row32747 - column8_row1 * column8_row1).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[116] * value;
	
	// Constraint: ecdsa/signature0/q_on_curve/on_curve.
	let value = (column8_row33 * column8_row33 - (column8_row1 * column8_row32747 + global_values.ecdsa_sig_config.alpha * column8_row1 + global_values.ecdsa_sig_config.beta)).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[117] * value;
	
	// Constraint: ecdsa/init_addr.
	let value = (column5_row390 - global_values.initial_ecdsa_addr).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[118] * value;
	
	// Constraint: ecdsa/message_addr.
	let value = (column5_row16774 - (column5_row390 + FELT_1)).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[119] * value;
	
	// Constraint: ecdsa/pubkey_addr.
	let value = (column5_row33158 - (column5_row16774 + FELT_1)) * domain35.field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[120] * value;
	
	// Constraint: ecdsa/message_value0.
	let value = (column5_row16775 - column8_row59).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[121] * value;
	
	// Constraint: ecdsa/pubkey_value0.
	let value = (column5_row391 - column8_row1).field_div(&felt_nonzero!(domain27));
	let total_sum = total_sum + constraint_coefficients[122] * value;
	
	// Constraint: bitwise/init_var_pool_addr.
	let value = (column5_row198 - global_values.initial_bitwise_addr).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[123] * value;
	
	// Constraint: bitwise/step_var_pool_addr.
	let value = (column5_row454 - (column5_row198 + FELT_1)) * domain18.field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[124] * value;
	
	// Constraint: bitwise/x_or_y_addr.
	let value = (column5_row902 - (column5_row966 + FELT_1)).field_div(&felt_nonzero!(domain19));
	let total_sum = total_sum + constraint_coefficients[125] * value;
	
	// Constraint: bitwise/next_var_pool_addr.
	let value = (column5_row1222 - (column5_row902 + FELT_1)) * domain36.field_div(&felt_nonzero!(domain19));
	let total_sum = total_sum + constraint_coefficients[126] * value;
	
	// Constraint: bitwise/partition.
	let value = (bitwise_sum_var_0_0 + bitwise_sum_var_8_0 - column5_row199).field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[127] * value;
	
	// Constraint: bitwise/or_is_and_plus_xor.
	let value = (column5_row903 - (column5_row711 + column5_row967)).field_div(&felt_nonzero!(domain19));
	let total_sum = total_sum + constraint_coefficients[128] * value;
	
	// Constraint: bitwise/addition_is_xor_with_and.
	let value = (column7_row1 + column7_row257 - (column7_row769 + column7_row513 + column7_row513)).field_div(&felt_nonzero!(domain20));
	let total_sum = total_sum + constraint_coefficients[129] * value;
	
	// Constraint: bitwise/unique_unpacking192.
	let value = ((column7_row705 + column7_row961) * FELT_16 - column7_row9).field_div(&felt_nonzero!(domain19));
	let total_sum = total_sum + constraint_coefficients[130] * value;
	
	// Constraint: bitwise/unique_unpacking193.
	let value = ((column7_row721 + column7_row977) * FELT_16 - column7_row521).field_div(&felt_nonzero!(domain19));
	let total_sum = total_sum + constraint_coefficients[131] * value;
	
	// Constraint: bitwise/unique_unpacking194.
	let value = ((column7_row737 + column7_row993) * FELT_16 - column7_row265).field_div(&felt_nonzero!(domain19));
	let total_sum = total_sum + constraint_coefficients[132] * value;
	
	// Constraint: bitwise/unique_unpacking195.
	let value = ((column7_row753 + column7_row1009) * FELT_256 - column7_row777).field_div(&felt_nonzero!(domain19));
	let total_sum = total_sum + constraint_coefficients[133] * value;
	
	// Constraint: ec_op/init_addr.
	let value = (column5_row8582 - global_values.initial_ec_op_addr).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[134] * value;
	
	// Constraint: ec_op/p_x_addr.
	let value = (column5_row24966 - (column5_row8582 + FELT_7)) * domain37.field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[135] * value;
	
	// Constraint: ec_op/p_y_addr.
	let value = (column5_row4486 - (column5_row8582 + FELT_1)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[136] * value;
	
	// Constraint: ec_op/q_x_addr.
	let value = (column5_row12678 - (column5_row4486 + FELT_1)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[137] * value;
	
	// Constraint: ec_op/q_y_addr.
	let value = (column5_row2438 - (column5_row12678 + FELT_1)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[138] * value;
	
	// Constraint: ec_op/m_addr.
	let value = (column5_row10630 - (column5_row2438 + FELT_1)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[139] * value;
	
	// Constraint: ec_op/r_x_addr.
	let value = (column5_row6534 - (column5_row10630 + FELT_1)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[140] * value;
	
	// Constraint: ec_op/r_y_addr.
	let value = (column5_row14726 - (column5_row6534 + FELT_1)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[141] * value;
	
	// Constraint: ec_op/doubling_q/slope.
	let value = (ec_op_doubling_q_x_squared_0 + ec_op_doubling_q_x_squared_0 + ec_op_doubling_q_x_squared_0 + global_values.ec_op_curve_config.alpha - (column8_row25 + column8_row25) * column8_row57) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[142] * value;
	
	// Constraint: ec_op/doubling_q/x.
	let value = (column8_row57 * column8_row57 - (column8_row41 + column8_row41 + column8_row105)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[143] * value;
	
	// Constraint: ec_op/doubling_q/y.
	let value = (column8_row25 + column8_row89 - column8_row57 * (column8_row41 - column8_row105)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[144] * value;
	
	// Constraint: ec_op/get_q_x.
	let value = (column5_row12679 - column8_row41).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[145] * value;
	
	// Constraint: ec_op/get_q_y.
	let value = (column5_row2439 - column8_row25).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[146] * value;
	
	// Constraint: ec_op/ec_subset_sum/bit_unpacking/last_one_is_zero.
	let value = (column8_row16371 * (column8_row21 - (column8_row85 + column8_row85))).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[147] * value;
	
	// Constraint: ec_op/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
	let value = (column8_row16371 * (column8_row85 - FELT_3138550867693340381917894711603833208051177722232017256448 * column8_row12309)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[148] * value;
	
	// Constraint: ec_op/ec_subset_sum/bit_unpacking/cumulative_bit192.
	let value = (column8_row16371 - column8_row16339 * (column8_row12309 - (column8_row12373 + column8_row12373))).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[149] * value;
	
	// Constraint: ec_op/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
	let value = (column8_row16339 * (column8_row12373 - FELT_8 * column8_row12565)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[150] * value;
	
	// Constraint: ec_op/ec_subset_sum/bit_unpacking/cumulative_bit196.
	let value = (column8_row16339 - (column8_row16085 - (column8_row16149 + column8_row16149)) * (column8_row12565 - (column8_row12629 + column8_row12629))).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[151] * value;
	
	// Constraint: ec_op/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
	let value = ((column8_row16085 - (column8_row16149 + column8_row16149)) * (column8_row12629 - FELT_18014398509481984 * column8_row16085)).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[152] * value;
	
	// Constraint: ec_op/ec_subset_sum/booleanity_test.
	let value = (ec_op_ec_subset_sum_bit_0 * (ec_op_ec_subset_sum_bit_0 - FELT_1)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[153] * value;
	
	// Constraint: ec_op/ec_subset_sum/bit_extraction_end.
	let value = (column8_row21).field_div(&felt_nonzero!(domain24));
	let total_sum = total_sum + constraint_coefficients[154] * value;
	
	// Constraint: ec_op/ec_subset_sum/zeros_tail.
	let value = (column8_row21).field_div(&felt_nonzero!(domain21));
	let total_sum = total_sum + constraint_coefficients[155] * value;
	
	// Constraint: ec_op/ec_subset_sum/add_points/slope.
	let value = (ec_op_ec_subset_sum_bit_0 * (column8_row37 - column8_row25) - column8_row11 * (column8_row5 - column8_row41)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[156] * value;
	
	// Constraint: ec_op/ec_subset_sum/add_points/x.
	let value = (column8_row11 * column8_row11 - ec_op_ec_subset_sum_bit_0 * (column8_row5 + column8_row41 + column8_row69)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[157] * value;
	
	// Constraint: ec_op/ec_subset_sum/add_points/y.
	let value = (ec_op_ec_subset_sum_bit_0 * (column8_row37 + column8_row101) - column8_row11 * (column8_row5 - column8_row69)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[158] * value;
	
	// Constraint: ec_op/ec_subset_sum/add_points/x_diff_inv.
	let value = (column8_row43 * (column8_row5 - column8_row41) - FELT_1) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[159] * value;
	
	// Constraint: ec_op/ec_subset_sum/copy_point/x.
	let value = (ec_op_ec_subset_sum_bit_neg_0 * (column8_row69 - column8_row5)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[160] * value;
	
	// Constraint: ec_op/ec_subset_sum/copy_point/y.
	let value = (ec_op_ec_subset_sum_bit_neg_0 * (column8_row101 - column8_row37)) * domain21.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[161] * value;
	
	// Constraint: ec_op/get_m.
	let value = (column8_row21 - column5_row10631).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[162] * value;
	
	// Constraint: ec_op/get_p_x.
	let value = (column5_row8583 - column8_row5).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[163] * value;
	
	// Constraint: ec_op/get_p_y.
	let value = (column5_row4487 - column8_row37).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[164] * value;
	
	// Constraint: ec_op/set_r_x.
	let value = (column5_row6535 - column8_row16325).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[165] * value;
	
	// Constraint: ec_op/set_r_y.
	let value = (column5_row14727 - column8_row16357).field_div(&felt_nonzero!(domain23));
	let total_sum = total_sum + constraint_coefficients[166] * value;
	
	// Constraint: poseidon/param_0/init_input_output_addr.
	let value = (column5_row38 - global_values.initial_poseidon_addr).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[167] * value;
	
	// Constraint: poseidon/param_0/addr_input_output_step.
	let value = (column5_row294 - (column5_row38 + FELT_3)) * domain34.field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[168] * value;
	
	// Constraint: poseidon/param_1/init_input_output_addr.
	let value = (column5_row166 - (global_values.initial_poseidon_addr + FELT_1)).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[169] * value;
	
	// Constraint: poseidon/param_1/addr_input_output_step.
	let value = (column5_row422 - (column5_row166 + FELT_3)) * domain34.field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[170] * value;
	
	// Constraint: poseidon/param_2/init_input_output_addr.
	let value = (column5_row102 - (global_values.initial_poseidon_addr + FELT_2)).field_div(&felt_nonzero!(domain29));
	let total_sum = total_sum + constraint_coefficients[171] * value;
	
	// Constraint: poseidon/param_2/addr_input_output_step.
	let value = (column5_row358 - (column5_row102 + FELT_3)) * domain34.field_div(&felt_nonzero!(domain8));
	let total_sum = total_sum + constraint_coefficients[172] * value;
	
	// Constraint: poseidon/poseidon/full_rounds_state0_squaring.
	let value = (column8_row53 * column8_row53 - column8_row29).field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[173] * value;
	
	// Constraint: poseidon/poseidon/full_rounds_state1_squaring.
	let value = (column8_row13 * column8_row13 - column8_row61).field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[174] * value;
	
	// Constraint: poseidon/poseidon/full_rounds_state2_squaring.
	let value = (column8_row45 * column8_row45 - column8_row3).field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[175] * value;
	
	// Constraint: poseidon/poseidon/partial_rounds_state0_squaring.
	let value = (column7_row3 * column7_row3 - column7_row7).field_div(&felt_nonzero!(domain3));
	let total_sum = total_sum + constraint_coefficients[176] * value;
	
	// Constraint: poseidon/poseidon/partial_rounds_state1_squaring.
	let value = (column8_row6 * column8_row6 - column8_row14) * domain15.field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[177] * value;
	
	// Constraint: poseidon/poseidon/add_first_round_key0.
	let value = (column5_row39 + FELT_2950795762459345168613727575620414179244544320470208355568817838579231751791 - column8_row53).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[178] * value;
	
	// Constraint: poseidon/poseidon/add_first_round_key1.
	let value = (column5_row167 + FELT_1587446564224215276866294500450702039420286416111469274423465069420553242820 - column8_row13).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[179] * value;
	
	// Constraint: poseidon/poseidon/add_first_round_key2.
	let value = (column5_row103 + FELT_1645965921169490687904413452218868659025437693527479459426157555728339600137 - column8_row45).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[180] * value;
	
	// Constraint: poseidon/poseidon/full_round0.
	let value = (column8_row117 - (poseidon_poseidon_full_rounds_state0_cubed_0 + poseidon_poseidon_full_rounds_state0_cubed_0 + poseidon_poseidon_full_rounds_state0_cubed_0 + poseidon_poseidon_full_rounds_state1_cubed_0 + poseidon_poseidon_full_rounds_state2_cubed_0 + global_values.poseidon_poseidon_full_round_key0)) * domain11.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[181] * value;
	
	// Constraint: poseidon/poseidon/full_round1.
	let value = (column8_row77 + poseidon_poseidon_full_rounds_state1_cubed_0 - (poseidon_poseidon_full_rounds_state0_cubed_0 + poseidon_poseidon_full_rounds_state2_cubed_0 + global_values.poseidon_poseidon_full_round_key1)) * domain11.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[182] * value;
	
	// Constraint: poseidon/poseidon/full_round2.
	let value = (column8_row109 + poseidon_poseidon_full_rounds_state2_cubed_0 + poseidon_poseidon_full_rounds_state2_cubed_0 - (poseidon_poseidon_full_rounds_state0_cubed_0 + poseidon_poseidon_full_rounds_state1_cubed_0 + global_values.poseidon_poseidon_full_round_key2)) * domain11.field_div(&felt_nonzero!(domain6));
	let total_sum = total_sum + constraint_coefficients[183] * value;
	
	// Constraint: poseidon/poseidon/last_full_round0.
	let value = (column5_row295 - (poseidon_poseidon_full_rounds_state0_cubed_7 + poseidon_poseidon_full_rounds_state0_cubed_7 + poseidon_poseidon_full_rounds_state0_cubed_7 + poseidon_poseidon_full_rounds_state1_cubed_7 + poseidon_poseidon_full_rounds_state2_cubed_7)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[184] * value;
	
	// Constraint: poseidon/poseidon/last_full_round1.
	let value = (column5_row423 + poseidon_poseidon_full_rounds_state1_cubed_7 - (poseidon_poseidon_full_rounds_state0_cubed_7 + poseidon_poseidon_full_rounds_state2_cubed_7)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[185] * value;
	
	// Constraint: poseidon/poseidon/last_full_round2.
	let value = (column5_row359 + poseidon_poseidon_full_rounds_state2_cubed_7 + poseidon_poseidon_full_rounds_state2_cubed_7 - (poseidon_poseidon_full_rounds_state0_cubed_7 + poseidon_poseidon_full_rounds_state1_cubed_7)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[186] * value;
	
	// Constraint: poseidon/poseidon/copy_partial_rounds0_i0.
	let value = (column7_row491 - column8_row6).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[187] * value;
	
	// Constraint: poseidon/poseidon/copy_partial_rounds0_i1.
	let value = (column7_row499 - column8_row22).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[188] * value;
	
	// Constraint: poseidon/poseidon/copy_partial_rounds0_i2.
	let value = (column7_row507 - column8_row38).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[189] * value;
	
	// Constraint: poseidon/poseidon/margin_full_to_partial0.
	let value = (column7_row3 + poseidon_poseidon_full_rounds_state2_cubed_3 + poseidon_poseidon_full_rounds_state2_cubed_3 - (poseidon_poseidon_full_rounds_state0_cubed_3 + poseidon_poseidon_full_rounds_state1_cubed_3 + FELT_2121140748740143694053732746913428481442990369183417228688865837805149503386)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[190] * value;
	
	// Constraint: poseidon/poseidon/margin_full_to_partial1.
	let value = (column7_row11 - (FELT_3618502788666131213697322783095070105623107215331596699973092056135872020477 * poseidon_poseidon_full_rounds_state1_cubed_3 + FELT_10 * poseidon_poseidon_full_rounds_state2_cubed_3 + FELT_4 * column7_row3 + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479 * poseidon_poseidon_partial_rounds_state0_cubed_0 + FELT_2006642341318481906727563724340978325665491359415674592697055778067937914672)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[191] * value;
	
	// Constraint: poseidon/poseidon/margin_full_to_partial2.
	let value = (column7_row19 - (FELT_8 * poseidon_poseidon_full_rounds_state2_cubed_3 + FELT_4 * column7_row3 + FELT_6 * poseidon_poseidon_partial_rounds_state0_cubed_0 + column7_row11 + column7_row11 + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479 * poseidon_poseidon_partial_rounds_state0_cubed_1 + FELT_427751140904099001132521606468025610873158555767197326325930641757709538586)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[192] * value;
	
	// Constraint: poseidon/poseidon/partial_round0.
	let value = (column7_row27 - (FELT_8 * poseidon_poseidon_partial_rounds_state0_cubed_0 + FELT_4 * column7_row11 + FELT_6 * poseidon_poseidon_partial_rounds_state0_cubed_1 + column7_row19 + column7_row19 + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479 * poseidon_poseidon_partial_rounds_state0_cubed_2 + global_values.poseidon_poseidon_partial_round_key0)) * domain16.field_div(&felt_nonzero!(domain3));
	let total_sum = total_sum + constraint_coefficients[193] * value;
	
	// Constraint: poseidon/poseidon/partial_round1.
	let value = (column8_row54 - (FELT_8 * poseidon_poseidon_partial_rounds_state1_cubed_0 + FELT_4 * column8_row22 + FELT_6 * poseidon_poseidon_partial_rounds_state1_cubed_1 + column8_row38 + column8_row38 + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479 * poseidon_poseidon_partial_rounds_state1_cubed_2 + global_values.poseidon_poseidon_partial_round_key1)) * domain17.field_div(&felt_nonzero!(domain5));
	let total_sum = total_sum + constraint_coefficients[194] * value;
	
	// Constraint: poseidon/poseidon/margin_partial_to_full0.
	let value = (column8_row309 - (FELT_16 * poseidon_poseidon_partial_rounds_state1_cubed_19 + FELT_8 * column8_row326 + FELT_16 * poseidon_poseidon_partial_rounds_state1_cubed_20 + FELT_6 * column8_row342 + poseidon_poseidon_partial_rounds_state1_cubed_21 + FELT_560279373700919169769089400651532183647886248799764942664266404650165812023)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[195] * value;
	
	// Constraint: poseidon/poseidon/margin_partial_to_full1.
	let value = (column8_row269 - (FELT_4 * poseidon_poseidon_partial_rounds_state1_cubed_20 + column8_row342 + column8_row342 + poseidon_poseidon_partial_rounds_state1_cubed_21 + FELT_1401754474293352309994371631695783042590401941592571735921592823982231996415)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[196] * value;
	
	// Constraint: poseidon/poseidon/margin_partial_to_full2.
	let value = (column8_row301 - (FELT_8 * poseidon_poseidon_partial_rounds_state1_cubed_19 + FELT_4 * column8_row326 + FELT_6 * poseidon_poseidon_partial_rounds_state1_cubed_20 + column8_row342 + column8_row342 + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479 * poseidon_poseidon_partial_rounds_state1_cubed_21 + FELT_1246177936547655338400308396717835700699368047388302793172818304164989556526)).field_div(&felt_nonzero!(domain13));
	let total_sum = total_sum + constraint_coefficients[197] * value;
	
	total_sum
}
