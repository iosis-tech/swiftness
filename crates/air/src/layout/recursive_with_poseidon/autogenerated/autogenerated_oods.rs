use crate::{
    consts::*,
    felt_nonzero,
    layout::recursive_with_poseidon::{LayoutTrait, StaticLayoutTrait},
};
use starknet_crypto::Felt;
use starknet_types_core::felt::NonZeroFelt;

pub fn eval_oods_polynomial_inner<Layout: StaticLayoutTrait + LayoutTrait>(
    column_values: &[Felt],
    oods_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    oods_point: &Felt,
    trace_generator: &Felt,
) -> Felt {
    // Compute powers.
    let pow0 = trace_generator.pow_felt(&(FELT_0));
    let pow1 = trace_generator.pow_felt(&(FELT_4089));
    let pow2 = trace_generator.pow_felt(&(FELT_2011));
    let pow3 = trace_generator.pow_felt(&(FELT_1539));
    let pow4 = trace_generator.pow_felt(&(FELT_1));
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

    // Sum the OODS constraints on the trace polynomials.
    let total_sum = FELT_0;

    let value = (column0 - oods_values[0]).field_div(&felt_nonzero!((point - pow0 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[0] * value;

    let value = (column0 - oods_values[1]).field_div(&felt_nonzero!((point - pow4 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[1] * value;

    let value = (column0 - oods_values[2]).field_div(&felt_nonzero!((point - pow5 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[2] * value;

    let value = (column0 - oods_values[3]).field_div(&felt_nonzero!((point - pow6 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[3] * value;

    let value = (column0 - oods_values[4]).field_div(&felt_nonzero!((point - pow7 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[4] * value;

    let value = (column0 - oods_values[5]).field_div(&felt_nonzero!((point - pow8 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[5] * value;

    let value = (column0 - oods_values[6]).field_div(&felt_nonzero!((point - pow9 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[6] * value;

    let value = (column0 - oods_values[7]).field_div(&felt_nonzero!((point - pow10 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[7] * value;

    let value = (column0 - oods_values[8]).field_div(&felt_nonzero!((point - pow11 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[8] * value;

    let value = (column0 - oods_values[9]).field_div(&felt_nonzero!((point - pow13 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[9] * value;

    let value = (column0 - oods_values[10]).field_div(&felt_nonzero!((point - pow14 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[10] * value;

    let value = (column0 - oods_values[11]).field_div(&felt_nonzero!((point - pow15 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[11] * value;

    let value = (column0 - oods_values[12]).field_div(&felt_nonzero!((point - pow16 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[12] * value;

    let value = (column0 - oods_values[13]).field_div(&felt_nonzero!((point - pow17 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[13] * value;

    let value = (column0 - oods_values[14]).field_div(&felt_nonzero!((point - pow18 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[14] * value;

    let value = (column0 - oods_values[15]).field_div(&felt_nonzero!((point - pow19 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[15] * value;

    let value = (column1 - oods_values[16]).field_div(&felt_nonzero!((point - pow0 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[16] * value;

    let value = (column1 - oods_values[17]).field_div(&felt_nonzero!((point - pow4 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[17] * value;

    let value = (column1 - oods_values[18]).field_div(&felt_nonzero!((point - pow5 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[18] * value;

    let value = (column1 - oods_values[19]).field_div(&felt_nonzero!((point - pow6 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[19] * value;

    let value = (column1 - oods_values[20]).field_div(&felt_nonzero!((point - pow7 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[20] * value;

    let value = (column1 - oods_values[21]).field_div(&felt_nonzero!((point - pow8 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[21] * value;

    let value = (column1 - oods_values[22]).field_div(&felt_nonzero!((point - pow11 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[22] * value;

    let value = (column1 - oods_values[23]).field_div(&felt_nonzero!((point - pow13 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[23] * value;

    let value = (column1 - oods_values[24]).field_div(&felt_nonzero!((point - pow14 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[24] * value;

    let value = (column1 - oods_values[25]).field_div(&felt_nonzero!((point - pow15 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[25] * value;

    let value = (column1 - oods_values[26]).field_div(&felt_nonzero!((point - pow16 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[26] * value;

    let value = (column1 - oods_values[27]).field_div(&felt_nonzero!((point - pow17 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[27] * value;

    let value = (column1 - oods_values[28]).field_div(&felt_nonzero!((point - pow20 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[28] * value;

    let value = (column1 - oods_values[29]).field_div(&felt_nonzero!((point - pow35 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[29] * value;

    let value = (column1 - oods_values[30]).field_div(&felt_nonzero!((point - pow36 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[30] * value;

    let value = (column1 - oods_values[31]).field_div(&felt_nonzero!((point - pow50 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[31] * value;

    let value = (column1 - oods_values[32]).field_div(&felt_nonzero!((point - pow51 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[32] * value;

    let value = (column1 - oods_values[33]).field_div(&felt_nonzero!((point - pow57 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[33] * value;

    let value = (column1 - oods_values[34]).field_div(&felt_nonzero!((point - pow62 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[34] * value;

    let value = (column1 - oods_values[35]).field_div(&felt_nonzero!((point - pow63 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[35] * value;

    let value = (column1 - oods_values[36]).field_div(&felt_nonzero!((point - pow65 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[36] * value;

    let value = (column1 - oods_values[37]).field_div(&felt_nonzero!((point - pow75 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[37] * value;

    let value = (column1 - oods_values[38]).field_div(&felt_nonzero!((point - pow76 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[38] * value;

    let value = (column1 - oods_values[39]).field_div(&felt_nonzero!((point - pow79 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[39] * value;

    let value = (column1 - oods_values[40]).field_div(&felt_nonzero!((point - pow80 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[40] * value;

    let value = (column1 - oods_values[41]).field_div(&felt_nonzero!((point - pow86 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[41] * value;

    let value = (column1 - oods_values[42]).field_div(&felt_nonzero!((point - pow87 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[42] * value;

    let value =
        (column1 - oods_values[43]).field_div(&felt_nonzero!((point - pow115 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[43] * value;

    let value = (column1 - oods_values[44]).field_div(&felt_nonzero!((point - pow88 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[44] * value;

    let value = (column1 - oods_values[45]).field_div(&felt_nonzero!((point - pow92 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[45] * value;

    let value = (column1 - oods_values[46]).field_div(&felt_nonzero!((point - pow93 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[46] * value;

    let value =
        (column1 - oods_values[47]).field_div(&felt_nonzero!((point - pow109 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[47] * value;

    let value =
        (column1 - oods_values[48]).field_div(&felt_nonzero!((point - pow113 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[48] * value;

    let value =
        (column1 - oods_values[49]).field_div(&felt_nonzero!((point - pow110 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[49] * value;

    let value =
        (column1 - oods_values[50]).field_div(&felt_nonzero!((point - pow114 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[50] * value;

    let value =
        (column1 - oods_values[51]).field_div(&felt_nonzero!((point - pow111 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[51] * value;

    let value =
        (column1 - oods_values[52]).field_div(&felt_nonzero!((point - pow116 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[52] * value;

    let value =
        (column1 - oods_values[53]).field_div(&felt_nonzero!((point - pow122 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[53] * value;

    let value =
        (column1 - oods_values[54]).field_div(&felt_nonzero!((point - pow127 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[54] * value;

    let value =
        (column1 - oods_values[55]).field_div(&felt_nonzero!((point - pow131 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[55] * value;

    let value =
        (column1 - oods_values[56]).field_div(&felt_nonzero!((point - pow132 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[56] * value;

    let value =
        (column1 - oods_values[57]).field_div(&felt_nonzero!((point - pow133 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[57] * value;

    let value = (column2 - oods_values[58]).field_div(&felt_nonzero!((point - pow0 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[58] * value;

    let value = (column2 - oods_values[59]).field_div(&felt_nonzero!((point - pow4 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[59] * value;

    let value = (column2 - oods_values[60]).field_div(&felt_nonzero!((point - pow5 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[60] * value;

    let value = (column2 - oods_values[61]).field_div(&felt_nonzero!((point - pow6 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[61] * value;

    let value = (column3 - oods_values[62]).field_div(&felt_nonzero!((point - pow0 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[62] * value;

    let value = (column3 - oods_values[63]).field_div(&felt_nonzero!((point - pow4 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[63] * value;

    let value = (column3 - oods_values[64]).field_div(&felt_nonzero!((point - pow5 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[64] * value;

    let value = (column3 - oods_values[65]).field_div(&felt_nonzero!((point - pow6 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[65] * value;

    let value = (column3 - oods_values[66]).field_div(&felt_nonzero!((point - pow7 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[66] * value;

    let value = (column3 - oods_values[67]).field_div(&felt_nonzero!((point - pow11 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[67] * value;

    let value = (column3 - oods_values[68]).field_div(&felt_nonzero!((point - pow16 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[68] * value;

    let value = (column3 - oods_values[69]).field_div(&felt_nonzero!((point - pow20 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[69] * value;

    let value = (column3 - oods_values[70]).field_div(&felt_nonzero!((point - pow22 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[70] * value;

    let value = (column3 - oods_values[71]).field_div(&felt_nonzero!((point - pow24 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[71] * value;

    let value = (column3 - oods_values[72]).field_div(&felt_nonzero!((point - pow26 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[72] * value;

    let value = (column3 - oods_values[73]).field_div(&felt_nonzero!((point - pow28 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[73] * value;

    let value = (column3 - oods_values[74]).field_div(&felt_nonzero!((point - pow31 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[74] * value;

    let value = (column3 - oods_values[75]).field_div(&felt_nonzero!((point - pow33 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[75] * value;

    let value = (column3 - oods_values[76]).field_div(&felt_nonzero!((point - pow37 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[76] * value;

    let value = (column3 - oods_values[77]).field_div(&felt_nonzero!((point - pow39 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[77] * value;

    let value = (column3 - oods_values[78]).field_div(&felt_nonzero!((point - pow41 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[78] * value;

    let value = (column3 - oods_values[79]).field_div(&felt_nonzero!((point - pow43 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[79] * value;

    let value = (column3 - oods_values[80]).field_div(&felt_nonzero!((point - pow45 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[80] * value;

    let value = (column3 - oods_values[81]).field_div(&felt_nonzero!((point - pow46 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[81] * value;

    let value = (column3 - oods_values[82]).field_div(&felt_nonzero!((point - pow48 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[82] * value;

    let value = (column3 - oods_values[83]).field_div(&felt_nonzero!((point - pow59 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[83] * value;

    let value = (column3 - oods_values[84]).field_div(&felt_nonzero!((point - pow60 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[84] * value;

    let value = (column3 - oods_values[85]).field_div(&felt_nonzero!((point - pow68 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[85] * value;

    let value = (column3 - oods_values[86]).field_div(&felt_nonzero!((point - pow69 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[86] * value;

    let value = (column3 - oods_values[87]).field_div(&felt_nonzero!((point - pow70 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[87] * value;

    let value = (column3 - oods_values[88]).field_div(&felt_nonzero!((point - pow71 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[88] * value;

    let value = (column3 - oods_values[89]).field_div(&felt_nonzero!((point - pow72 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[89] * value;

    let value = (column3 - oods_values[90]).field_div(&felt_nonzero!((point - pow73 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[90] * value;

    let value = (column3 - oods_values[91]).field_div(&felt_nonzero!((point - pow82 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[91] * value;

    let value = (column3 - oods_values[92]).field_div(&felt_nonzero!((point - pow83 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[92] * value;

    let value = (column3 - oods_values[93]).field_div(&felt_nonzero!((point - pow84 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[93] * value;

    let value = (column3 - oods_values[94]).field_div(&felt_nonzero!((point - pow85 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[94] * value;

    let value = (column4 - oods_values[95]).field_div(&felt_nonzero!((point - pow0 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[95] * value;

    let value = (column4 - oods_values[96]).field_div(&felt_nonzero!((point - pow4 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[96] * value;

    let value = (column4 - oods_values[97]).field_div(&felt_nonzero!((point - pow5 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[97] * value;

    let value = (column4 - oods_values[98]).field_div(&felt_nonzero!((point - pow6 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[98] * value;

    let value = (column4 - oods_values[99]).field_div(&felt_nonzero!((point - pow7 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[99] * value;

    let value = (column4 - oods_values[100]).field_div(&felt_nonzero!((point - pow8 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[100] * value;

    let value = (column4 - oods_values[101]).field_div(&felt_nonzero!((point - pow9 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[101] * value;

    let value =
        (column4 - oods_values[102]).field_div(&felt_nonzero!((point - pow10 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[102] * value;

    let value =
        (column4 - oods_values[103]).field_div(&felt_nonzero!((point - pow11 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[103] * value;

    let value =
        (column4 - oods_values[104]).field_div(&felt_nonzero!((point - pow13 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[104] * value;

    let value =
        (column4 - oods_values[105]).field_div(&felt_nonzero!((point - pow15 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[105] * value;

    let value =
        (column4 - oods_values[106]).field_div(&felt_nonzero!((point - pow16 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[106] * value;

    let value =
        (column4 - oods_values[107]).field_div(&felt_nonzero!((point - pow17 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[107] * value;

    let value =
        (column4 - oods_values[108]).field_div(&felt_nonzero!((point - pow37 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[108] * value;

    let value =
        (column4 - oods_values[109]).field_div(&felt_nonzero!((point - pow52 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[109] * value;

    let value =
        (column4 - oods_values[110]).field_div(&felt_nonzero!((point - pow58 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[110] * value;

    let value =
        (column4 - oods_values[111]).field_div(&felt_nonzero!((point - pow66 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[111] * value;

    let value =
        (column4 - oods_values[112]).field_div(&felt_nonzero!((point - pow67 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[112] * value;

    let value =
        (column4 - oods_values[113]).field_div(&felt_nonzero!((point - pow78 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[113] * value;

    let value =
        (column4 - oods_values[114]).field_div(&felt_nonzero!((point - pow81 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[114] * value;

    let value = (column4 - oods_values[115]).field_div(&felt_nonzero!((point - pow3 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[115] * value;

    let value =
        (column4 - oods_values[116]).field_div(&felt_nonzero!((point - pow12 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[116] * value;

    let value =
        (column4 - oods_values[117]).field_div(&felt_nonzero!((point - pow30 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[117] * value;

    let value =
        (column4 - oods_values[118]).field_div(&felt_nonzero!((point - pow112 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[118] * value;

    let value = (column4 - oods_values[119]).field_div(&felt_nonzero!((point - pow2 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[119] * value;

    let value =
        (column4 - oods_values[120]).field_div(&felt_nonzero!((point - pow123 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[120] * value;

    let value =
        (column4 - oods_values[121]).field_div(&felt_nonzero!((point - pow124 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[121] * value;

    let value =
        (column4 - oods_values[122]).field_div(&felt_nonzero!((point - pow125 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[122] * value;

    let value =
        (column4 - oods_values[123]).field_div(&felt_nonzero!((point - pow126 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[123] * value;

    let value =
        (column4 - oods_values[124]).field_div(&felt_nonzero!((point - pow128 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[124] * value;

    let value =
        (column4 - oods_values[125]).field_div(&felt_nonzero!((point - pow129 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[125] * value;

    let value =
        (column4 - oods_values[126]).field_div(&felt_nonzero!((point - pow130 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[126] * value;

    let value = (column4 - oods_values[127]).field_div(&felt_nonzero!((point - pow1 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[127] * value;

    let value = (column5 - oods_values[128]).field_div(&felt_nonzero!((point - pow0 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[128] * value;

    let value = (column5 - oods_values[129]).field_div(&felt_nonzero!((point - pow4 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[129] * value;

    let value = (column5 - oods_values[130]).field_div(&felt_nonzero!((point - pow5 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[130] * value;

    let value = (column5 - oods_values[131]).field_div(&felt_nonzero!((point - pow7 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[131] * value;

    let value = (column5 - oods_values[132]).field_div(&felt_nonzero!((point - pow9 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[132] * value;

    let value =
        (column5 - oods_values[133]).field_div(&felt_nonzero!((point - pow11 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[133] * value;

    let value =
        (column5 - oods_values[134]).field_div(&felt_nonzero!((point - pow13 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[134] * value;

    let value =
        (column5 - oods_values[135]).field_div(&felt_nonzero!((point - pow14 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[135] * value;

    let value =
        (column5 - oods_values[136]).field_div(&felt_nonzero!((point - pow16 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[136] * value;

    let value =
        (column5 - oods_values[137]).field_div(&felt_nonzero!((point - pow18 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[137] * value;

    let value =
        (column5 - oods_values[138]).field_div(&felt_nonzero!((point - pow20 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[138] * value;

    let value =
        (column5 - oods_values[139]).field_div(&felt_nonzero!((point - pow21 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[139] * value;

    let value =
        (column5 - oods_values[140]).field_div(&felt_nonzero!((point - pow23 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[140] * value;

    let value =
        (column5 - oods_values[141]).field_div(&felt_nonzero!((point - pow24 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[141] * value;

    let value =
        (column5 - oods_values[142]).field_div(&felt_nonzero!((point - pow25 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[142] * value;

    let value =
        (column5 - oods_values[143]).field_div(&felt_nonzero!((point - pow27 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[143] * value;

    let value =
        (column5 - oods_values[144]).field_div(&felt_nonzero!((point - pow29 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[144] * value;

    let value =
        (column5 - oods_values[145]).field_div(&felt_nonzero!((point - pow32 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[145] * value;

    let value =
        (column5 - oods_values[146]).field_div(&felt_nonzero!((point - pow34 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[146] * value;

    let value =
        (column5 - oods_values[147]).field_div(&felt_nonzero!((point - pow38 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[147] * value;

    let value =
        (column5 - oods_values[148]).field_div(&felt_nonzero!((point - pow40 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[148] * value;

    let value =
        (column5 - oods_values[149]).field_div(&felt_nonzero!((point - pow42 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[149] * value;

    let value =
        (column5 - oods_values[150]).field_div(&felt_nonzero!((point - pow44 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[150] * value;

    let value =
        (column5 - oods_values[151]).field_div(&felt_nonzero!((point - pow47 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[151] * value;

    let value =
        (column5 - oods_values[152]).field_div(&felt_nonzero!((point - pow49 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[152] * value;

    let value =
        (column5 - oods_values[153]).field_div(&felt_nonzero!((point - pow53 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[153] * value;

    let value =
        (column5 - oods_values[154]).field_div(&felt_nonzero!((point - pow54 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[154] * value;

    let value =
        (column5 - oods_values[155]).field_div(&felt_nonzero!((point - pow55 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[155] * value;

    let value =
        (column5 - oods_values[156]).field_div(&felt_nonzero!((point - pow56 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[156] * value;

    let value =
        (column5 - oods_values[157]).field_div(&felt_nonzero!((point - pow61 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[157] * value;

    let value =
        (column5 - oods_values[158]).field_div(&felt_nonzero!((point - pow64 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[158] * value;

    let value =
        (column5 - oods_values[159]).field_div(&felt_nonzero!((point - pow74 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[159] * value;

    let value =
        (column5 - oods_values[160]).field_div(&felt_nonzero!((point - pow77 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[160] * value;

    let value =
        (column5 - oods_values[161]).field_div(&felt_nonzero!((point - pow89 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[161] * value;

    let value =
        (column5 - oods_values[162]).field_div(&felt_nonzero!((point - pow90 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[162] * value;

    let value =
        (column5 - oods_values[163]).field_div(&felt_nonzero!((point - pow91 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[163] * value;

    let value =
        (column5 - oods_values[164]).field_div(&felt_nonzero!((point - pow94 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[164] * value;

    let value =
        (column5 - oods_values[165]).field_div(&felt_nonzero!((point - pow95 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[165] * value;

    let value =
        (column5 - oods_values[166]).field_div(&felt_nonzero!((point - pow96 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[166] * value;

    let value =
        (column5 - oods_values[167]).field_div(&felt_nonzero!((point - pow97 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[167] * value;

    let value =
        (column5 - oods_values[168]).field_div(&felt_nonzero!((point - pow98 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[168] * value;

    let value =
        (column5 - oods_values[169]).field_div(&felt_nonzero!((point - pow99 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[169] * value;

    let value =
        (column5 - oods_values[170]).field_div(&felt_nonzero!((point - pow100 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[170] * value;

    let value =
        (column5 - oods_values[171]).field_div(&felt_nonzero!((point - pow101 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[171] * value;

    let value =
        (column5 - oods_values[172]).field_div(&felt_nonzero!((point - pow102 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[172] * value;

    let value =
        (column5 - oods_values[173]).field_div(&felt_nonzero!((point - pow104 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[173] * value;

    let value =
        (column5 - oods_values[174]).field_div(&felt_nonzero!((point - pow106 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[174] * value;

    let value =
        (column5 - oods_values[175]).field_div(&felt_nonzero!((point - pow103 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[175] * value;

    let value =
        (column5 - oods_values[176]).field_div(&felt_nonzero!((point - pow105 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[176] * value;

    let value =
        (column5 - oods_values[177]).field_div(&felt_nonzero!((point - pow107 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[177] * value;

    let value =
        (column5 - oods_values[178]).field_div(&felt_nonzero!((point - pow108 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[178] * value;

    let value =
        (column5 - oods_values[179]).field_div(&felt_nonzero!((point - pow117 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[179] * value;

    let value =
        (column5 - oods_values[180]).field_div(&felt_nonzero!((point - pow118 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[180] * value;

    let value =
        (column5 - oods_values[181]).field_div(&felt_nonzero!((point - pow119 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[181] * value;

    let value =
        (column5 - oods_values[182]).field_div(&felt_nonzero!((point - pow120 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[182] * value;

    let value =
        (column5 - oods_values[183]).field_div(&felt_nonzero!((point - pow121 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[183] * value;

    let value = (column6 - oods_values[184]).field_div(&felt_nonzero!((point - pow0 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[184] * value;

    let value = (column6 - oods_values[185]).field_div(&felt_nonzero!((point - pow4 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[185] * value;

    let value = (column6 - oods_values[186]).field_div(&felt_nonzero!((point - pow5 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[186] * value;

    let value = (column6 - oods_values[187]).field_div(&felt_nonzero!((point - pow6 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[187] * value;

    let value = (column7 - oods_values[188]).field_div(&felt_nonzero!((point - pow0 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[188] * value;

    let value = (column7 - oods_values[189]).field_div(&felt_nonzero!((point - pow4 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[189] * value;

    let value = (column7 - oods_values[190]).field_div(&felt_nonzero!((point - pow5 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[190] * value;

    let value = (column7 - oods_values[191]).field_div(&felt_nonzero!((point - pow8 * oods_point)));
    let total_sum = total_sum + constraint_coefficients[191] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow_felt(&(Layout::CONSTRAINT_DEGREE.into()));

    let value = (column_values[Layout::NUM_COLUMNS_FIRST + Layout::NUM_COLUMNS_SECOND]
        - oods_values[192])
        .field_div(&felt_nonzero!((point - oods_point_to_deg)));
    let total_sum = total_sum + constraint_coefficients[192] * value;

    let value = (column_values[Layout::NUM_COLUMNS_FIRST + Layout::NUM_COLUMNS_SECOND + 1]
        - oods_values[193])
        .field_div(&felt_nonzero!((point - oods_point_to_deg)));
    let total_sum = total_sum + constraint_coefficients[193] * value;

    total_sum
}
