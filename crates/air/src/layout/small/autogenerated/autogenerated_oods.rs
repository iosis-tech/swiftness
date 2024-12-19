use crate::{
    consts::*,
    felt_nonzero,
    layout::small::{LayoutTrait, StaticLayoutTrait},
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
    let pow1 = trace_generator.pow_felt(&(FELT_8160));
    let pow2 = trace_generator.pow_felt(&(FELT_4081));
    let pow3 = trace_generator.pow_felt(&(FELT_1));
    let pow4 = pow3 * pow3; // pow(trace_generator, 2).
    let pow5 = pow2 * pow4; // pow(trace_generator, 4083).
    let pow6 = pow3 * pow4; // pow(trace_generator, 3).
    let pow7 = pow3 * pow6; // pow(trace_generator, 4).
    let pow8 = pow3 * pow7; // pow(trace_generator, 5).
    let pow9 = pow3 * pow8; // pow(trace_generator, 6).
    let pow10 = pow3 * pow9; // pow(trace_generator, 7).
    let pow11 = pow1 * pow10; // pow(trace_generator, 8167).
    let pow12 = pow3 * pow10; // pow(trace_generator, 8).
    let pow13 = pow2 * pow12; // pow(trace_generator, 4089).
    let pow14 = pow3 * pow12; // pow(trace_generator, 9).
    let pow15 = pow3 * pow14; // pow(trace_generator, 10).
    let pow16 = pow2 * pow15; // pow(trace_generator, 4091).
    let pow17 = pow3 * pow15; // pow(trace_generator, 11).
    let pow18 = pow3 * pow17; // pow(trace_generator, 12).
    let pow19 = pow3 * pow18; // pow(trace_generator, 13).
    let pow20 = pow3 * pow19; // pow(trace_generator, 14).
    let pow21 = pow3 * pow20; // pow(trace_generator, 15).
    let pow22 = pow2 * pow18; // pow(trace_generator, 4093).
    let pow23 = pow3 * pow21; // pow(trace_generator, 16).
    let pow24 = pow3 * pow23; // pow(trace_generator, 17).
    let pow25 = pow7 * pow24; // pow(trace_generator, 21).
    let pow26 = pow2 * pow25; // pow(trace_generator, 4102).
    let pow27 = pow1 * pow24; // pow(trace_generator, 8177).
    let pow28 = pow4 * pow27; // pow(trace_generator, 8179).
    let pow29 = pow12 * pow26; // pow(trace_generator, 4110).
    let pow30 = pow3 * pow25; // pow(trace_generator, 22).
    let pow31 = pow3 * pow30; // pow(trace_generator, 23).
    let pow32 = pow3 * pow31; // pow(trace_generator, 24).
    let pow33 = pow3 * pow32; // pow(trace_generator, 25).
    let pow34 = pow12 * pow29; // pow(trace_generator, 4118).
    let pow35 = pow1 * pow31; // pow(trace_generator, 8183).
    let pow36 = pow1 * pow33; // pow(trace_generator, 8185).
    let pow37 = pow4 * pow36; // pow(trace_generator, 8187).
    let pow38 = pow6 * pow33; // pow(trace_generator, 28).
    let pow39 = pow4 * pow38; // pow(trace_generator, 30).
    let pow40 = pow3 * pow39; // pow(trace_generator, 31).
    let pow41 = pow1 * pow40; // pow(trace_generator, 8191).
    let pow42 = pow10 * pow40; // pow(trace_generator, 38).
    let pow43 = pow2 * pow42; // pow(trace_generator, 4119).
    let pow44 = pow3 * pow42; // pow(trace_generator, 39).
    let pow45 = pow8 * pow44; // pow(trace_generator, 44).
    let pow46 = pow6 * pow45; // pow(trace_generator, 47).
    let pow47 = pow12 * pow46; // pow(trace_generator, 55).
    let pow48 = pow11 * pow46; // pow(trace_generator, 8214).
    let pow49 = pow8 * pow47; // pow(trace_generator, 60).
    let pow50 = pow15 * pow49; // pow(trace_generator, 70).
    let pow51 = pow3 * pow50; // pow(trace_generator, 71).
    let pow52 = pow8 * pow51; // pow(trace_generator, 76).
    let pow53 = pow7 * pow52; // pow(trace_generator, 80).
    let pow54 = pow18 * pow53; // pow(trace_generator, 92).
    let pow55 = pow15 * pow54; // pow(trace_generator, 102).
    let pow56 = pow3 * pow55; // pow(trace_generator, 103).
    let pow57 = pow8 * pow56; // pow(trace_generator, 108).
    let pow58 = pow23 * pow57; // pow(trace_generator, 124).
    let pow59 = pow15 * pow58; // pow(trace_generator, 134).
    let pow60 = pow3 * pow59; // pow(trace_generator, 135).
    let pow61 = pow14 * pow60; // pow(trace_generator, 144).
    let pow62 = pow31 * pow61; // pow(trace_generator, 167).
    let pow63 = pow33 * pow62; // pow(trace_generator, 192).
    let pow64 = pow3 * pow63; // pow(trace_generator, 193).
    let pow65 = pow6 * pow64; // pow(trace_generator, 196).
    let pow66 = pow3 * pow65; // pow(trace_generator, 197).
    let pow67 = pow4 * pow66; // pow(trace_generator, 199).
    let pow68 = pow14 * pow67; // pow(trace_generator, 208).
    let pow69 = pow30 * pow68; // pow(trace_generator, 230).
    let pow70 = pow25 * pow69; // pow(trace_generator, 251).
    let pow71 = pow3 * pow70; // pow(trace_generator, 252).
    let pow72 = pow6 * pow71; // pow(trace_generator, 255).
    let pow73 = pow3 * pow72; // pow(trace_generator, 256).
    let pow74 = pow72 * pow73; // pow(trace_generator, 511).
    let pow75 = pow44 * pow73; // pow(trace_generator, 295).
    let pow76 = pow10 * pow73; // pow(trace_generator, 263).
    let pow77 = pow63 * pow76; // pow(trace_generator, 455).
    let pow78 = pow62 * pow73; // pow(trace_generator, 423).
    let pow79 = pow60 * pow73; // pow(trace_generator, 391).
    let pow80 = pow51 * pow73; // pow(trace_generator, 327).

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
    let column22 = column_values[22];
    let column23 = column_values[23];
    let column24 = column_values[24];

    // Sum the OODS constraints on the trace polynomials.
    let total_sum = FELT_0;

    let value = (column0 - oods_values[0]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[0] * value;

    let value = (column0 - oods_values[1]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[1] * value;

    let value = (column0 - oods_values[2]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[2] * value;

    let value = (column0 - oods_values[3]).field_div(&felt_nonzero!(point - pow12 * oods_point));
    let total_sum = total_sum + constraint_coefficients[3] * value;

    let value = (column0 - oods_values[4]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[4] * value;

    let value = (column0 - oods_values[5]).field_div(&felt_nonzero!(point - pow38 * oods_point));
    let total_sum = total_sum + constraint_coefficients[5] * value;

    let value = (column0 - oods_values[6]).field_div(&felt_nonzero!(point - pow45 * oods_point));
    let total_sum = total_sum + constraint_coefficients[6] * value;

    let value = (column0 - oods_values[7]).field_div(&felt_nonzero!(point - pow49 * oods_point));
    let total_sum = total_sum + constraint_coefficients[7] * value;

    let value = (column0 - oods_values[8]).field_div(&felt_nonzero!(point - pow52 * oods_point));
    let total_sum = total_sum + constraint_coefficients[8] * value;

    let value = (column0 - oods_values[9]).field_div(&felt_nonzero!(point - pow54 * oods_point));
    let total_sum = total_sum + constraint_coefficients[9] * value;

    let value = (column0 - oods_values[10]).field_div(&felt_nonzero!(point - pow57 * oods_point));
    let total_sum = total_sum + constraint_coefficients[10] * value;

    let value = (column0 - oods_values[11]).field_div(&felt_nonzero!(point - pow58 * oods_point));
    let total_sum = total_sum + constraint_coefficients[11] * value;

    let value = (column1 - oods_values[12]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[12] * value;

    let value = (column1 - oods_values[13]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[13] * value;

    let value = (column1 - oods_values[14]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[14] * value;

    let value = (column1 - oods_values[15]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[15] * value;

    let value = (column1 - oods_values[16]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[16] * value;

    let value = (column1 - oods_values[17]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[17] * value;

    let value = (column1 - oods_values[18]).field_div(&felt_nonzero!(point - pow9 * oods_point));
    let total_sum = total_sum + constraint_coefficients[18] * value;

    let value = (column1 - oods_values[19]).field_div(&felt_nonzero!(point - pow10 * oods_point));
    let total_sum = total_sum + constraint_coefficients[19] * value;

    let value = (column1 - oods_values[20]).field_div(&felt_nonzero!(point - pow12 * oods_point));
    let total_sum = total_sum + constraint_coefficients[20] * value;

    let value = (column1 - oods_values[21]).field_div(&felt_nonzero!(point - pow14 * oods_point));
    let total_sum = total_sum + constraint_coefficients[21] * value;

    let value = (column1 - oods_values[22]).field_div(&felt_nonzero!(point - pow15 * oods_point));
    let total_sum = total_sum + constraint_coefficients[22] * value;

    let value = (column1 - oods_values[23]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[23] * value;

    let value = (column1 - oods_values[24]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[24] * value;

    let value = (column1 - oods_values[25]).field_div(&felt_nonzero!(point - pow19 * oods_point));
    let total_sum = total_sum + constraint_coefficients[25] * value;

    let value = (column1 - oods_values[26]).field_div(&felt_nonzero!(point - pow20 * oods_point));
    let total_sum = total_sum + constraint_coefficients[26] * value;

    let value = (column1 - oods_values[27]).field_div(&felt_nonzero!(point - pow21 * oods_point));
    let total_sum = total_sum + constraint_coefficients[27] * value;

    let value = (column2 - oods_values[28]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[28] * value;

    let value = (column2 - oods_values[29]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[29] * value;

    let value = (column3 - oods_values[30]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[30] * value;

    let value = (column3 - oods_values[31]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[31] * value;

    let value = (column3 - oods_values[32]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[32] * value;

    let value = (column3 - oods_values[33]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[33] * value;

    let value = (column3 - oods_values[34]).field_div(&felt_nonzero!(point - pow74 * oods_point));
    let total_sum = total_sum + constraint_coefficients[34] * value;

    let value = (column4 - oods_values[35]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[35] * value;

    let value = (column4 - oods_values[36]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[36] * value;

    let value = (column4 - oods_values[37]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[37] * value;

    let value = (column4 - oods_values[38]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[38] * value;

    let value = (column5 - oods_values[39]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[39] * value;

    let value = (column5 - oods_values[40]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[40] * value;

    let value = (column5 - oods_values[41]).field_div(&felt_nonzero!(point - pow63 * oods_point));
    let total_sum = total_sum + constraint_coefficients[41] * value;

    let value = (column5 - oods_values[42]).field_div(&felt_nonzero!(point - pow64 * oods_point));
    let total_sum = total_sum + constraint_coefficients[42] * value;

    let value = (column5 - oods_values[43]).field_div(&felt_nonzero!(point - pow65 * oods_point));
    let total_sum = total_sum + constraint_coefficients[43] * value;

    let value = (column5 - oods_values[44]).field_div(&felt_nonzero!(point - pow66 * oods_point));
    let total_sum = total_sum + constraint_coefficients[44] * value;

    let value = (column5 - oods_values[45]).field_div(&felt_nonzero!(point - pow70 * oods_point));
    let total_sum = total_sum + constraint_coefficients[45] * value;

    let value = (column5 - oods_values[46]).field_div(&felt_nonzero!(point - pow71 * oods_point));
    let total_sum = total_sum + constraint_coefficients[46] * value;

    let value = (column5 - oods_values[47]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[47] * value;

    let value = (column6 - oods_values[48]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[48] * value;

    let value = (column6 - oods_values[49]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[49] * value;

    let value = (column6 - oods_values[50]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[50] * value;

    let value = (column6 - oods_values[51]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[51] * value;

    let value = (column6 - oods_values[52]).field_div(&felt_nonzero!(point - pow74 * oods_point));
    let total_sum = total_sum + constraint_coefficients[52] * value;

    let value = (column7 - oods_values[53]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[53] * value;

    let value = (column7 - oods_values[54]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[54] * value;

    let value = (column7 - oods_values[55]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[55] * value;

    let value = (column7 - oods_values[56]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[56] * value;

    let value = (column8 - oods_values[57]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[57] * value;

    let value = (column8 - oods_values[58]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[58] * value;

    let value = (column8 - oods_values[59]).field_div(&felt_nonzero!(point - pow63 * oods_point));
    let total_sum = total_sum + constraint_coefficients[59] * value;

    let value = (column8 - oods_values[60]).field_div(&felt_nonzero!(point - pow64 * oods_point));
    let total_sum = total_sum + constraint_coefficients[60] * value;

    let value = (column8 - oods_values[61]).field_div(&felt_nonzero!(point - pow65 * oods_point));
    let total_sum = total_sum + constraint_coefficients[61] * value;

    let value = (column8 - oods_values[62]).field_div(&felt_nonzero!(point - pow66 * oods_point));
    let total_sum = total_sum + constraint_coefficients[62] * value;

    let value = (column8 - oods_values[63]).field_div(&felt_nonzero!(point - pow70 * oods_point));
    let total_sum = total_sum + constraint_coefficients[63] * value;

    let value = (column8 - oods_values[64]).field_div(&felt_nonzero!(point - pow71 * oods_point));
    let total_sum = total_sum + constraint_coefficients[64] * value;

    let value = (column8 - oods_values[65]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[65] * value;

    let value = (column9 - oods_values[66]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[66] * value;

    let value = (column9 - oods_values[67]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[67] * value;

    let value = (column9 - oods_values[68]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[68] * value;

    let value = (column9 - oods_values[69]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[69] * value;

    let value = (column9 - oods_values[70]).field_div(&felt_nonzero!(point - pow74 * oods_point));
    let total_sum = total_sum + constraint_coefficients[70] * value;

    let value = (column10 - oods_values[71]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[71] * value;

    let value = (column10 - oods_values[72]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[72] * value;

    let value = (column10 - oods_values[73]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[73] * value;

    let value = (column10 - oods_values[74]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[74] * value;

    let value = (column11 - oods_values[75]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[75] * value;

    let value = (column11 - oods_values[76]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[76] * value;

    let value = (column11 - oods_values[77]).field_div(&felt_nonzero!(point - pow63 * oods_point));
    let total_sum = total_sum + constraint_coefficients[77] * value;

    let value = (column11 - oods_values[78]).field_div(&felt_nonzero!(point - pow64 * oods_point));
    let total_sum = total_sum + constraint_coefficients[78] * value;

    let value = (column11 - oods_values[79]).field_div(&felt_nonzero!(point - pow65 * oods_point));
    let total_sum = total_sum + constraint_coefficients[79] * value;

    let value = (column11 - oods_values[80]).field_div(&felt_nonzero!(point - pow66 * oods_point));
    let total_sum = total_sum + constraint_coefficients[80] * value;

    let value = (column11 - oods_values[81]).field_div(&felt_nonzero!(point - pow70 * oods_point));
    let total_sum = total_sum + constraint_coefficients[81] * value;

    let value = (column11 - oods_values[82]).field_div(&felt_nonzero!(point - pow71 * oods_point));
    let total_sum = total_sum + constraint_coefficients[82] * value;

    let value = (column11 - oods_values[83]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[83] * value;

    let value = (column12 - oods_values[84]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[84] * value;

    let value = (column12 - oods_values[85]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[85] * value;

    let value = (column12 - oods_values[86]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[86] * value;

    let value = (column12 - oods_values[87]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[87] * value;

    let value = (column12 - oods_values[88]).field_div(&felt_nonzero!(point - pow74 * oods_point));
    let total_sum = total_sum + constraint_coefficients[88] * value;

    let value = (column13 - oods_values[89]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[89] * value;

    let value = (column13 - oods_values[90]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[90] * value;

    let value = (column13 - oods_values[91]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[91] * value;

    let value = (column13 - oods_values[92]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[92] * value;

    let value = (column14 - oods_values[93]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[93] * value;

    let value = (column14 - oods_values[94]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[94] * value;

    let value = (column14 - oods_values[95]).field_div(&felt_nonzero!(point - pow63 * oods_point));
    let total_sum = total_sum + constraint_coefficients[95] * value;

    let value = (column14 - oods_values[96]).field_div(&felt_nonzero!(point - pow64 * oods_point));
    let total_sum = total_sum + constraint_coefficients[96] * value;

    let value = (column14 - oods_values[97]).field_div(&felt_nonzero!(point - pow65 * oods_point));
    let total_sum = total_sum + constraint_coefficients[97] * value;

    let value = (column14 - oods_values[98]).field_div(&felt_nonzero!(point - pow66 * oods_point));
    let total_sum = total_sum + constraint_coefficients[98] * value;

    let value = (column14 - oods_values[99]).field_div(&felt_nonzero!(point - pow70 * oods_point));
    let total_sum = total_sum + constraint_coefficients[99] * value;

    let value = (column14 - oods_values[100]).field_div(&felt_nonzero!(point - pow71 * oods_point));
    let total_sum = total_sum + constraint_coefficients[100] * value;

    let value = (column14 - oods_values[101]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[101] * value;

    let value = (column15 - oods_values[102]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[102] * value;

    let value = (column15 - oods_values[103]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[103] * value;

    let value = (column16 - oods_values[104]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[104] * value;

    let value = (column16 - oods_values[105]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[105] * value;

    let value = (column17 - oods_values[106]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[106] * value;

    let value = (column17 - oods_values[107]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[107] * value;

    let value = (column18 - oods_values[108]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[108] * value;

    let value = (column18 - oods_values[109]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[109] * value;

    let value = (column19 - oods_values[110]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[110] * value;

    let value = (column19 - oods_values[111]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[111] * value;

    let value = (column19 - oods_values[112]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[112] * value;

    let value = (column19 - oods_values[113]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[113] * value;

    let value = (column19 - oods_values[114]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[114] * value;

    let value = (column19 - oods_values[115]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[115] * value;

    let value = (column19 - oods_values[116]).field_div(&felt_nonzero!(point - pow9 * oods_point));
    let total_sum = total_sum + constraint_coefficients[116] * value;

    let value = (column19 - oods_values[117]).field_div(&felt_nonzero!(point - pow10 * oods_point));
    let total_sum = total_sum + constraint_coefficients[117] * value;

    let value = (column19 - oods_values[118]).field_div(&felt_nonzero!(point - pow12 * oods_point));
    let total_sum = total_sum + constraint_coefficients[118] * value;

    let value = (column19 - oods_values[119]).field_div(&felt_nonzero!(point - pow14 * oods_point));
    let total_sum = total_sum + constraint_coefficients[119] * value;

    let value = (column19 - oods_values[120]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[120] * value;

    let value = (column19 - oods_values[121]).field_div(&felt_nonzero!(point - pow19 * oods_point));
    let total_sum = total_sum + constraint_coefficients[121] * value;

    let value = (column19 - oods_values[122]).field_div(&felt_nonzero!(point - pow23 * oods_point));
    let total_sum = total_sum + constraint_coefficients[122] * value;

    let value = (column19 - oods_values[123]).field_div(&felt_nonzero!(point - pow30 * oods_point));
    let total_sum = total_sum + constraint_coefficients[123] * value;

    let value = (column19 - oods_values[124]).field_div(&felt_nonzero!(point - pow31 * oods_point));
    let total_sum = total_sum + constraint_coefficients[124] * value;

    let value = (column19 - oods_values[125]).field_div(&felt_nonzero!(point - pow42 * oods_point));
    let total_sum = total_sum + constraint_coefficients[125] * value;

    let value = (column19 - oods_values[126]).field_div(&felt_nonzero!(point - pow44 * oods_point));
    let total_sum = total_sum + constraint_coefficients[126] * value;

    let value = (column19 - oods_values[127]).field_div(&felt_nonzero!(point - pow50 * oods_point));
    let total_sum = total_sum + constraint_coefficients[127] * value;

    let value = (column19 - oods_values[128]).field_div(&felt_nonzero!(point - pow51 * oods_point));
    let total_sum = total_sum + constraint_coefficients[128] * value;

    let value = (column19 - oods_values[129]).field_div(&felt_nonzero!(point - pow55 * oods_point));
    let total_sum = total_sum + constraint_coefficients[129] * value;

    let value = (column19 - oods_values[130]).field_div(&felt_nonzero!(point - pow56 * oods_point));
    let total_sum = total_sum + constraint_coefficients[130] * value;

    let value = (column19 - oods_values[131]).field_div(&felt_nonzero!(point - pow59 * oods_point));
    let total_sum = total_sum + constraint_coefficients[131] * value;

    let value = (column19 - oods_values[132]).field_div(&felt_nonzero!(point - pow60 * oods_point));
    let total_sum = total_sum + constraint_coefficients[132] * value;

    let value = (column19 - oods_values[133]).field_div(&felt_nonzero!(point - pow62 * oods_point));
    let total_sum = total_sum + constraint_coefficients[133] * value;

    let value = (column19 - oods_values[134]).field_div(&felt_nonzero!(point - pow67 * oods_point));
    let total_sum = total_sum + constraint_coefficients[134] * value;

    let value = (column19 - oods_values[135]).field_div(&felt_nonzero!(point - pow69 * oods_point));
    let total_sum = total_sum + constraint_coefficients[135] * value;

    let value = (column19 - oods_values[136]).field_div(&felt_nonzero!(point - pow76 * oods_point));
    let total_sum = total_sum + constraint_coefficients[136] * value;

    let value = (column19 - oods_values[137]).field_div(&felt_nonzero!(point - pow75 * oods_point));
    let total_sum = total_sum + constraint_coefficients[137] * value;

    let value = (column19 - oods_values[138]).field_div(&felt_nonzero!(point - pow80 * oods_point));
    let total_sum = total_sum + constraint_coefficients[138] * value;

    let value = (column19 - oods_values[139]).field_div(&felt_nonzero!(point - pow79 * oods_point));
    let total_sum = total_sum + constraint_coefficients[139] * value;

    let value = (column19 - oods_values[140]).field_div(&felt_nonzero!(point - pow78 * oods_point));
    let total_sum = total_sum + constraint_coefficients[140] * value;

    let value = (column19 - oods_values[141]).field_div(&felt_nonzero!(point - pow77 * oods_point));
    let total_sum = total_sum + constraint_coefficients[141] * value;

    let value = (column19 - oods_values[142]).field_div(&felt_nonzero!(point - pow34 * oods_point));
    let total_sum = total_sum + constraint_coefficients[142] * value;

    let value = (column19 - oods_values[143]).field_div(&felt_nonzero!(point - pow43 * oods_point));
    let total_sum = total_sum + constraint_coefficients[143] * value;

    let value = (column19 - oods_values[144]).field_div(&felt_nonzero!(point - pow48 * oods_point));
    let total_sum = total_sum + constraint_coefficients[144] * value;

    let value = (column20 - oods_values[145]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[145] * value;

    let value = (column20 - oods_values[146]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[146] * value;

    let value = (column20 - oods_values[147]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[147] * value;

    let value = (column20 - oods_values[148]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[148] * value;

    let value = (column21 - oods_values[149]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[149] * value;

    let value = (column21 - oods_values[150]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[150] * value;

    let value = (column21 - oods_values[151]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[151] * value;

    let value = (column21 - oods_values[152]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[152] * value;

    let value = (column21 - oods_values[153]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[153] * value;

    let value = (column21 - oods_values[154]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[154] * value;

    let value = (column21 - oods_values[155]).field_div(&felt_nonzero!(point - pow9 * oods_point));
    let total_sum = total_sum + constraint_coefficients[155] * value;

    let value = (column21 - oods_values[156]).field_div(&felt_nonzero!(point - pow10 * oods_point));
    let total_sum = total_sum + constraint_coefficients[156] * value;

    let value = (column21 - oods_values[157]).field_div(&felt_nonzero!(point - pow12 * oods_point));
    let total_sum = total_sum + constraint_coefficients[157] * value;

    let value = (column21 - oods_values[158]).field_div(&felt_nonzero!(point - pow14 * oods_point));
    let total_sum = total_sum + constraint_coefficients[158] * value;

    let value = (column21 - oods_values[159]).field_div(&felt_nonzero!(point - pow15 * oods_point));
    let total_sum = total_sum + constraint_coefficients[159] * value;

    let value = (column21 - oods_values[160]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[160] * value;

    let value = (column21 - oods_values[161]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[161] * value;

    let value = (column21 - oods_values[162]).field_div(&felt_nonzero!(point - pow19 * oods_point));
    let total_sum = total_sum + constraint_coefficients[162] * value;

    let value = (column21 - oods_values[163]).field_div(&felt_nonzero!(point - pow20 * oods_point));
    let total_sum = total_sum + constraint_coefficients[163] * value;

    let value = (column21 - oods_values[164]).field_div(&felt_nonzero!(point - pow21 * oods_point));
    let total_sum = total_sum + constraint_coefficients[164] * value;

    let value = (column21 - oods_values[165]).field_div(&felt_nonzero!(point - pow23 * oods_point));
    let total_sum = total_sum + constraint_coefficients[165] * value;

    let value = (column21 - oods_values[166]).field_div(&felt_nonzero!(point - pow24 * oods_point));
    let total_sum = total_sum + constraint_coefficients[166] * value;

    let value = (column21 - oods_values[167]).field_div(&felt_nonzero!(point - pow25 * oods_point));
    let total_sum = total_sum + constraint_coefficients[167] * value;

    let value = (column21 - oods_values[168]).field_div(&felt_nonzero!(point - pow30 * oods_point));
    let total_sum = total_sum + constraint_coefficients[168] * value;

    let value = (column21 - oods_values[169]).field_div(&felt_nonzero!(point - pow31 * oods_point));
    let total_sum = total_sum + constraint_coefficients[169] * value;

    let value = (column21 - oods_values[170]).field_div(&felt_nonzero!(point - pow32 * oods_point));
    let total_sum = total_sum + constraint_coefficients[170] * value;

    let value = (column21 - oods_values[171]).field_div(&felt_nonzero!(point - pow33 * oods_point));
    let total_sum = total_sum + constraint_coefficients[171] * value;

    let value = (column21 - oods_values[172]).field_div(&felt_nonzero!(point - pow39 * oods_point));
    let total_sum = total_sum + constraint_coefficients[172] * value;

    let value = (column21 - oods_values[173]).field_div(&felt_nonzero!(point - pow40 * oods_point));
    let total_sum = total_sum + constraint_coefficients[173] * value;

    let value = (column21 - oods_values[174]).field_div(&felt_nonzero!(point - pow44 * oods_point));
    let total_sum = total_sum + constraint_coefficients[174] * value;

    let value = (column21 - oods_values[175]).field_div(&felt_nonzero!(point - pow46 * oods_point));
    let total_sum = total_sum + constraint_coefficients[175] * value;

    let value = (column21 - oods_values[176]).field_div(&felt_nonzero!(point - pow47 * oods_point));
    let total_sum = total_sum + constraint_coefficients[176] * value;

    let value = (column21 - oods_values[177]).field_div(&felt_nonzero!(point - pow2 * oods_point));
    let total_sum = total_sum + constraint_coefficients[177] * value;

    let value = (column21 - oods_values[178]).field_div(&felt_nonzero!(point - pow5 * oods_point));
    let total_sum = total_sum + constraint_coefficients[178] * value;

    let value = (column21 - oods_values[179]).field_div(&felt_nonzero!(point - pow13 * oods_point));
    let total_sum = total_sum + constraint_coefficients[179] * value;

    let value = (column21 - oods_values[180]).field_div(&felt_nonzero!(point - pow16 * oods_point));
    let total_sum = total_sum + constraint_coefficients[180] * value;

    let value = (column21 - oods_values[181]).field_div(&felt_nonzero!(point - pow22 * oods_point));
    let total_sum = total_sum + constraint_coefficients[181] * value;

    let value = (column21 - oods_values[182]).field_div(&felt_nonzero!(point - pow26 * oods_point));
    let total_sum = total_sum + constraint_coefficients[182] * value;

    let value = (column21 - oods_values[183]).field_div(&felt_nonzero!(point - pow29 * oods_point));
    let total_sum = total_sum + constraint_coefficients[183] * value;

    let value = (column21 - oods_values[184]).field_div(&felt_nonzero!(point - pow11 * oods_point));
    let total_sum = total_sum + constraint_coefficients[184] * value;

    let value = (column21 - oods_values[185]).field_div(&felt_nonzero!(point - pow27 * oods_point));
    let total_sum = total_sum + constraint_coefficients[185] * value;

    let value = (column21 - oods_values[186]).field_div(&felt_nonzero!(point - pow28 * oods_point));
    let total_sum = total_sum + constraint_coefficients[186] * value;

    let value = (column21 - oods_values[187]).field_div(&felt_nonzero!(point - pow35 * oods_point));
    let total_sum = total_sum + constraint_coefficients[187] * value;

    let value = (column21 - oods_values[188]).field_div(&felt_nonzero!(point - pow36 * oods_point));
    let total_sum = total_sum + constraint_coefficients[188] * value;

    let value = (column21 - oods_values[189]).field_div(&felt_nonzero!(point - pow37 * oods_point));
    let total_sum = total_sum + constraint_coefficients[189] * value;

    let value = (column21 - oods_values[190]).field_div(&felt_nonzero!(point - pow41 * oods_point));
    let total_sum = total_sum + constraint_coefficients[190] * value;

    let value = (column22 - oods_values[191]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[191] * value;

    let value = (column22 - oods_values[192]).field_div(&felt_nonzero!(point - pow23 * oods_point));
    let total_sum = total_sum + constraint_coefficients[192] * value;

    let value = (column22 - oods_values[193]).field_div(&felt_nonzero!(point - pow53 * oods_point));
    let total_sum = total_sum + constraint_coefficients[193] * value;

    let value = (column22 - oods_values[194]).field_div(&felt_nonzero!(point - pow61 * oods_point));
    let total_sum = total_sum + constraint_coefficients[194] * value;

    let value = (column22 - oods_values[195]).field_div(&felt_nonzero!(point - pow68 * oods_point));
    let total_sum = total_sum + constraint_coefficients[195] * value;

    let value = (column22 - oods_values[196]).field_div(&felt_nonzero!(point - pow1 * oods_point));
    let total_sum = total_sum + constraint_coefficients[196] * value;

    let value = (column23 - oods_values[197]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[197] * value;

    let value = (column23 - oods_values[198]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[198] * value;

    let value = (column24 - oods_values[199]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[199] * value;

    let value = (column24 - oods_values[200]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[200] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow_felt(&(Layout::CONSTRAINT_DEGREE.into()));

    let value = (column_values
        [Layout::NUM_COLUMNS_FIRST as usize + Layout::NUM_COLUMNS_SECOND as usize]
        - oods_values[201])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));
    let total_sum = total_sum + constraint_coefficients[201] * value;

    let value = (column_values
        [Layout::NUM_COLUMNS_FIRST as usize + Layout::NUM_COLUMNS_SECOND as usize + 1]
        - oods_values[202])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));

    total_sum + constraint_coefficients[202] * value
}
