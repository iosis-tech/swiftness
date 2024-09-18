use crate::{
    consts::*,
    felt_nonzero,
    layout::recursive::{LayoutTrait, StaticLayoutTrait},
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
    let pow1 = trace_generator.pow_felt(&(FELT_1004));
    let pow2 = trace_generator.pow_felt(&(FELT_768));
    let pow3 = trace_generator.pow_felt(&(FELT_522));
    let pow4 = trace_generator.pow_felt(&(FELT_1));
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
    let total_sum = FELT_0;

    let value = (column0 - oods_values[0]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[0] * value;

    let value = (column0 - oods_values[1]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[1] * value;

    let value = (column0 - oods_values[2]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[2] * value;

    let value = (column0 - oods_values[3]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[3] * value;

    let value = (column0 - oods_values[4]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[4] * value;

    let value = (column0 - oods_values[5]).field_div(&felt_nonzero!(point - pow11 * oods_point));
    let total_sum = total_sum + constraint_coefficients[5] * value;

    let value = (column0 - oods_values[6]).field_div(&felt_nonzero!(point - pow12 * oods_point));
    let total_sum = total_sum + constraint_coefficients[6] * value;

    let value = (column0 - oods_values[7]).field_div(&felt_nonzero!(point - pow13 * oods_point));
    let total_sum = total_sum + constraint_coefficients[7] * value;

    let value = (column0 - oods_values[8]).field_div(&felt_nonzero!(point - pow14 * oods_point));
    let total_sum = total_sum + constraint_coefficients[8] * value;

    let value = (column0 - oods_values[9]).field_div(&felt_nonzero!(point - pow15 * oods_point));
    let total_sum = total_sum + constraint_coefficients[9] * value;

    let value = (column0 - oods_values[10]).field_div(&felt_nonzero!(point - pow16 * oods_point));
    let total_sum = total_sum + constraint_coefficients[10] * value;

    let value = (column0 - oods_values[11]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[11] * value;

    let value = (column0 - oods_values[12]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[12] * value;

    let value = (column0 - oods_values[13]).field_div(&felt_nonzero!(point - pow19 * oods_point));
    let total_sum = total_sum + constraint_coefficients[13] * value;

    let value = (column0 - oods_values[14]).field_div(&felt_nonzero!(point - pow20 * oods_point));
    let total_sum = total_sum + constraint_coefficients[14] * value;

    let value = (column0 - oods_values[15]).field_div(&felt_nonzero!(point - pow21 * oods_point));
    let total_sum = total_sum + constraint_coefficients[15] * value;

    let value = (column1 - oods_values[16]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[16] * value;

    let value = (column1 - oods_values[17]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[17] * value;

    let value = (column1 - oods_values[18]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[18] * value;

    let value = (column1 - oods_values[19]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[19] * value;

    let value = (column1 - oods_values[20]).field_div(&felt_nonzero!(point - pow12 * oods_point));
    let total_sum = total_sum + constraint_coefficients[20] * value;

    let value = (column1 - oods_values[21]).field_div(&felt_nonzero!(point - pow14 * oods_point));
    let total_sum = total_sum + constraint_coefficients[21] * value;

    let value = (column1 - oods_values[22]).field_div(&felt_nonzero!(point - pow16 * oods_point));
    let total_sum = total_sum + constraint_coefficients[22] * value;

    let value = (column1 - oods_values[23]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[23] * value;

    let value = (column1 - oods_values[24]).field_div(&felt_nonzero!(point - pow20 * oods_point));
    let total_sum = total_sum + constraint_coefficients[24] * value;

    let value = (column1 - oods_values[25]).field_div(&felt_nonzero!(point - pow22 * oods_point));
    let total_sum = total_sum + constraint_coefficients[25] * value;

    let value = (column1 - oods_values[26]).field_div(&felt_nonzero!(point - pow26 * oods_point));
    let total_sum = total_sum + constraint_coefficients[26] * value;

    let value = (column1 - oods_values[27]).field_div(&felt_nonzero!(point - pow29 * oods_point));
    let total_sum = total_sum + constraint_coefficients[27] * value;

    let value = (column1 - oods_values[28]).field_div(&felt_nonzero!(point - pow30 * oods_point));
    let total_sum = total_sum + constraint_coefficients[28] * value;

    let value = (column1 - oods_values[29]).field_div(&felt_nonzero!(point - pow31 * oods_point));
    let total_sum = total_sum + constraint_coefficients[29] * value;

    let value = (column1 - oods_values[30]).field_div(&felt_nonzero!(point - pow33 * oods_point));
    let total_sum = total_sum + constraint_coefficients[30] * value;

    let value = (column1 - oods_values[31]).field_div(&felt_nonzero!(point - pow39 * oods_point));
    let total_sum = total_sum + constraint_coefficients[31] * value;

    let value = (column1 - oods_values[32]).field_div(&felt_nonzero!(point - pow40 * oods_point));
    let total_sum = total_sum + constraint_coefficients[32] * value;

    let value = (column1 - oods_values[33]).field_div(&felt_nonzero!(point - pow41 * oods_point));
    let total_sum = total_sum + constraint_coefficients[33] * value;

    let value = (column1 - oods_values[34]).field_div(&felt_nonzero!(point - pow42 * oods_point));
    let total_sum = total_sum + constraint_coefficients[34] * value;

    let value = (column1 - oods_values[35]).field_div(&felt_nonzero!(point - pow52 * oods_point));
    let total_sum = total_sum + constraint_coefficients[35] * value;

    let value = (column1 - oods_values[36]).field_div(&felt_nonzero!(point - pow53 * oods_point));
    let total_sum = total_sum + constraint_coefficients[36] * value;

    let value = (column1 - oods_values[37]).field_div(&felt_nonzero!(point - pow57 * oods_point));
    let total_sum = total_sum + constraint_coefficients[37] * value;

    let value = (column1 - oods_values[38]).field_div(&felt_nonzero!(point - pow58 * oods_point));
    let total_sum = total_sum + constraint_coefficients[38] * value;

    let value = (column1 - oods_values[39]).field_div(&felt_nonzero!(point - pow60 * oods_point));
    let total_sum = total_sum + constraint_coefficients[39] * value;

    let value = (column1 - oods_values[40]).field_div(&felt_nonzero!(point - pow61 * oods_point));
    let total_sum = total_sum + constraint_coefficients[40] * value;

    let value = (column1 - oods_values[41]).field_div(&felt_nonzero!(point - pow62 * oods_point));
    let total_sum = total_sum + constraint_coefficients[41] * value;

    let value = (column1 - oods_values[42]).field_div(&felt_nonzero!(point - pow63 * oods_point));
    let total_sum = total_sum + constraint_coefficients[42] * value;

    let value = (column1 - oods_values[43]).field_div(&felt_nonzero!(point - pow65 * oods_point));
    let total_sum = total_sum + constraint_coefficients[43] * value;

    let value = (column1 - oods_values[44]).field_div(&felt_nonzero!(point - pow66 * oods_point));
    let total_sum = total_sum + constraint_coefficients[44] * value;

    let value = (column1 - oods_values[45]).field_div(&felt_nonzero!(point - pow68 * oods_point));
    let total_sum = total_sum + constraint_coefficients[45] * value;

    let value = (column1 - oods_values[46]).field_div(&felt_nonzero!(point - pow69 * oods_point));
    let total_sum = total_sum + constraint_coefficients[46] * value;

    let value = (column2 - oods_values[47]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[47] * value;

    let value = (column2 - oods_values[48]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[48] * value;

    let value = (column3 - oods_values[49]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[49] * value;

    let value = (column3 - oods_values[50]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[50] * value;

    let value = (column3 - oods_values[51]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[51] * value;

    let value = (column3 - oods_values[52]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[52] * value;

    let value = (column3 - oods_values[53]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[53] * value;

    let value = (column3 - oods_values[54]).field_div(&felt_nonzero!(point - pow11 * oods_point));
    let total_sum = total_sum + constraint_coefficients[54] * value;

    let value = (column3 - oods_values[55]).field_div(&felt_nonzero!(point - pow14 * oods_point));
    let total_sum = total_sum + constraint_coefficients[55] * value;

    let value = (column3 - oods_values[56]).field_div(&felt_nonzero!(point - pow15 * oods_point));
    let total_sum = total_sum + constraint_coefficients[56] * value;

    let value = (column3 - oods_values[57]).field_div(&felt_nonzero!(point - pow16 * oods_point));
    let total_sum = total_sum + constraint_coefficients[57] * value;

    let value = (column3 - oods_values[58]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[58] * value;

    let value = (column3 - oods_values[59]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[59] * value;

    let value = (column3 - oods_values[60]).field_div(&felt_nonzero!(point - pow19 * oods_point));
    let total_sum = total_sum + constraint_coefficients[60] * value;

    let value = (column3 - oods_values[61]).field_div(&felt_nonzero!(point - pow22 * oods_point));
    let total_sum = total_sum + constraint_coefficients[61] * value;

    let value = (column3 - oods_values[62]).field_div(&felt_nonzero!(point - pow33 * oods_point));
    let total_sum = total_sum + constraint_coefficients[62] * value;

    let value = (column3 - oods_values[63]).field_div(&felt_nonzero!(point - pow38 * oods_point));
    let total_sum = total_sum + constraint_coefficients[63] * value;

    let value = (column3 - oods_values[64]).field_div(&felt_nonzero!(point - pow46 * oods_point));
    let total_sum = total_sum + constraint_coefficients[64] * value;

    let value = (column3 - oods_values[65]).field_div(&felt_nonzero!(point - pow47 * oods_point));
    let total_sum = total_sum + constraint_coefficients[65] * value;

    let value = (column3 - oods_values[66]).field_div(&felt_nonzero!(point - pow49 * oods_point));
    let total_sum = total_sum + constraint_coefficients[66] * value;

    let value = (column3 - oods_values[67]).field_div(&felt_nonzero!(point - pow54 * oods_point));
    let total_sum = total_sum + constraint_coefficients[67] * value;

    let value = (column3 - oods_values[68]).field_div(&felt_nonzero!(point - pow55 * oods_point));
    let total_sum = total_sum + constraint_coefficients[68] * value;

    let value = (column3 - oods_values[69]).field_div(&felt_nonzero!(point - pow59 * oods_point));
    let total_sum = total_sum + constraint_coefficients[69] * value;

    let value = (column3 - oods_values[70]).field_div(&felt_nonzero!(point - pow66 * oods_point));
    let total_sum = total_sum + constraint_coefficients[70] * value;

    let value = (column3 - oods_values[71]).field_div(&felt_nonzero!(point - pow67 * oods_point));
    let total_sum = total_sum + constraint_coefficients[71] * value;

    let value = (column3 - oods_values[72]).field_div(&felt_nonzero!(point - pow71 * oods_point));
    let total_sum = total_sum + constraint_coefficients[72] * value;

    let value = (column3 - oods_values[73]).field_div(&felt_nonzero!(point - pow70 * oods_point));
    let total_sum = total_sum + constraint_coefficients[73] * value;

    let value = (column3 - oods_values[74]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[74] * value;

    let value = (column3 - oods_values[75]).field_div(&felt_nonzero!(point - pow5 * oods_point));
    let total_sum = total_sum + constraint_coefficients[75] * value;

    let value = (column3 - oods_values[76]).field_div(&felt_nonzero!(point - pow43 * oods_point));
    let total_sum = total_sum + constraint_coefficients[76] * value;

    let value = (column3 - oods_values[77]).field_div(&felt_nonzero!(point - pow44 * oods_point));
    let total_sum = total_sum + constraint_coefficients[77] * value;

    let value = (column3 - oods_values[78]).field_div(&felt_nonzero!(point - pow45 * oods_point));
    let total_sum = total_sum + constraint_coefficients[78] * value;

    let value = (column4 - oods_values[79]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[79] * value;

    let value = (column4 - oods_values[80]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[80] * value;

    let value = (column4 - oods_values[81]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[81] * value;

    let value = (column4 - oods_values[82]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[82] * value;

    let value = (column5 - oods_values[83]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[83] * value;

    let value = (column5 - oods_values[84]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[84] * value;

    let value = (column5 - oods_values[85]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[85] * value;

    let value = (column5 - oods_values[86]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[86] * value;

    let value = (column5 - oods_values[87]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[87] * value;

    let value = (column5 - oods_values[88]).field_div(&felt_nonzero!(point - pow11 * oods_point));
    let total_sum = total_sum + constraint_coefficients[88] * value;

    let value = (column5 - oods_values[89]).field_div(&felt_nonzero!(point - pow12 * oods_point));
    let total_sum = total_sum + constraint_coefficients[89] * value;

    let value = (column5 - oods_values[90]).field_div(&felt_nonzero!(point - pow13 * oods_point));
    let total_sum = total_sum + constraint_coefficients[90] * value;

    let value = (column5 - oods_values[91]).field_div(&felt_nonzero!(point - pow14 * oods_point));
    let total_sum = total_sum + constraint_coefficients[91] * value;

    let value = (column5 - oods_values[92]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[92] * value;

    let value = (column5 - oods_values[93]).field_div(&felt_nonzero!(point - pow39 * oods_point));
    let total_sum = total_sum + constraint_coefficients[93] * value;

    let value = (column5 - oods_values[94]).field_div(&felt_nonzero!(point - pow48 * oods_point));
    let total_sum = total_sum + constraint_coefficients[94] * value;

    let value = (column5 - oods_values[95]).field_div(&felt_nonzero!(point - pow50 * oods_point));
    let total_sum = total_sum + constraint_coefficients[95] * value;

    let value = (column5 - oods_values[96]).field_div(&felt_nonzero!(point - pow56 * oods_point));
    let total_sum = total_sum + constraint_coefficients[96] * value;

    let value = (column5 - oods_values[97]).field_div(&felt_nonzero!(point - pow60 * oods_point));
    let total_sum = total_sum + constraint_coefficients[97] * value;

    let value = (column5 - oods_values[98]).field_div(&felt_nonzero!(point - pow64 * oods_point));
    let total_sum = total_sum + constraint_coefficients[98] * value;

    let value = (column5 - oods_values[99]).field_div(&felt_nonzero!(point - pow68 * oods_point));
    let total_sum = total_sum + constraint_coefficients[99] * value;

    let value = (column5 - oods_values[100]).field_div(&felt_nonzero!(point - pow25 * oods_point));
    let total_sum = total_sum + constraint_coefficients[100] * value;

    let value = (column5 - oods_values[101]).field_div(&felt_nonzero!(point - pow28 * oods_point));
    let total_sum = total_sum + constraint_coefficients[101] * value;

    let value = (column5 - oods_values[102]).field_div(&felt_nonzero!(point - pow36 * oods_point));
    let total_sum = total_sum + constraint_coefficients[102] * value;

    let value = (column5 - oods_values[103]).field_div(&felt_nonzero!(point - pow37 * oods_point));
    let total_sum = total_sum + constraint_coefficients[103] * value;

    let value = (column5 - oods_values[104]).field_div(&felt_nonzero!(point - pow35 * oods_point));
    let total_sum = total_sum + constraint_coefficients[104] * value;

    let value = (column6 - oods_values[105]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[105] * value;

    let value = (column6 - oods_values[106]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[106] * value;

    let value = (column6 - oods_values[107]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[107] * value;

    let value = (column6 - oods_values[108]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[108] * value;

    let value = (column6 - oods_values[109]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[109] * value;

    let value = (column6 - oods_values[110]).field_div(&felt_nonzero!(point - pow11 * oods_point));
    let total_sum = total_sum + constraint_coefficients[110] * value;

    let value = (column6 - oods_values[111]).field_div(&felt_nonzero!(point - pow13 * oods_point));
    let total_sum = total_sum + constraint_coefficients[111] * value;

    let value = (column6 - oods_values[112]).field_div(&felt_nonzero!(point - pow15 * oods_point));
    let total_sum = total_sum + constraint_coefficients[112] * value;

    let value = (column6 - oods_values[113]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[113] * value;

    let value = (column6 - oods_values[114]).field_div(&felt_nonzero!(point - pow19 * oods_point));
    let total_sum = total_sum + constraint_coefficients[114] * value;

    let value = (column6 - oods_values[115]).field_div(&felt_nonzero!(point - pow24 * oods_point));
    let total_sum = total_sum + constraint_coefficients[115] * value;

    let value = (column6 - oods_values[116]).field_div(&felt_nonzero!(point - pow32 * oods_point));
    let total_sum = total_sum + constraint_coefficients[116] * value;

    let value = (column6 - oods_values[117]).field_div(&felt_nonzero!(point - pow2 * oods_point));
    let total_sum = total_sum + constraint_coefficients[117] * value;

    let value = (column6 - oods_values[118]).field_div(&felt_nonzero!(point - pow10 * oods_point));
    let total_sum = total_sum + constraint_coefficients[118] * value;

    let value = (column6 - oods_values[119]).field_div(&felt_nonzero!(point - pow23 * oods_point));
    let total_sum = total_sum + constraint_coefficients[119] * value;

    let value = (column6 - oods_values[120]).field_div(&felt_nonzero!(point - pow51 * oods_point));
    let total_sum = total_sum + constraint_coefficients[120] * value;

    let value = (column6 - oods_values[121]).field_div(&felt_nonzero!(point - pow1 * oods_point));
    let total_sum = total_sum + constraint_coefficients[121] * value;

    let value = (column6 - oods_values[122]).field_div(&felt_nonzero!(point - pow9 * oods_point));
    let total_sum = total_sum + constraint_coefficients[122] * value;

    let value = (column6 - oods_values[123]).field_div(&felt_nonzero!(point - pow27 * oods_point));
    let total_sum = total_sum + constraint_coefficients[123] * value;

    let value = (column6 - oods_values[124]).field_div(&felt_nonzero!(point - pow34 * oods_point));
    let total_sum = total_sum + constraint_coefficients[124] * value;

    let value = (column7 - oods_values[125]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[125] * value;

    let value = (column7 - oods_values[126]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[126] * value;

    let value = (column8 - oods_values[127]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[127] * value;

    let value = (column8 - oods_values[128]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[128] * value;

    let value = (column9 - oods_values[129]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[129] * value;

    let value = (column9 - oods_values[130]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[130] * value;

    let value = (column9 - oods_values[131]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[131] * value;

    let value = (column9 - oods_values[132]).field_div(&felt_nonzero!(point - pow11 * oods_point));
    let total_sum = total_sum + constraint_coefficients[132] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow_felt(&(Layout::CONSTRAINT_DEGREE.into()));

    let value = (column_values[Layout::NUM_COLUMNS_FIRST + Layout::NUM_COLUMNS_SECOND]
        - oods_values[133])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));
    let total_sum = total_sum + constraint_coefficients[133] * value;

    let value = (column_values[Layout::NUM_COLUMNS_FIRST + Layout::NUM_COLUMNS_SECOND + 1]
        - oods_values[134])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));

    total_sum + constraint_coefficients[134] * value
}
