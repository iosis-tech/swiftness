use crate::{
    consts::*,
    felt_nonzero,
    layout::recursive::{LayoutTrait, StaticLayoutTrait},
};
use starknet_crypto::Felt;
use starknet_types_core::felt::NonZeroFelt;

// Too big to be inlined
#[inline(never)]
pub fn eval_oods_polynomial_inner<Layout: StaticLayoutTrait + LayoutTrait>(
    powers: &mut [Felt; 72],
    column_values: &[Felt],
    oods_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    oods_point: &Felt,
    trace_generator: &Felt,
) -> Felt {
    // let mut powers = Box::new([Felt::ZERO; 72]);

    // Compute powers.
    powers[0] = trace_generator.pow_felt(&(FELT_0));
    powers[1] = trace_generator.pow_felt(&(FELT_1004));
    powers[2] = trace_generator.pow_felt(&(FELT_768));
    powers[3] = trace_generator.pow_felt(&(FELT_522));
    powers[4] = trace_generator.pow_felt(&(FELT_1));
    powers[5] = powers[3] * powers[4]; // pow(trace_generator, 523).
    powers[6] = powers[4] * powers[4]; // pow(trace_generator, 2).
    powers[7] = powers[4] * powers[6]; // pow(trace_generator, 3).
    powers[8] = powers[4] * powers[7]; // pow(trace_generator, 4).
    powers[9] = powers[1] * powers[8]; // pow(trace_generator, 1008).
    powers[10] = powers[2] * powers[8]; // pow(trace_generator, 772).
    powers[11] = powers[4] * powers[8]; // pow(trace_generator, 5).
    powers[12] = powers[4] * powers[11]; // pow(trace_generator, 6).
    powers[13] = powers[4] * powers[12]; // pow(trace_generator, 7).
    powers[14] = powers[4] * powers[13]; // pow(trace_generator, 8).
    powers[15] = powers[4] * powers[14]; // pow(trace_generator, 9).
    powers[16] = powers[4] * powers[15]; // pow(trace_generator, 10).
    powers[17] = powers[4] * powers[16]; // pow(trace_generator, 11).
    powers[18] = powers[4] * powers[17]; // pow(trace_generator, 12).
    powers[19] = powers[4] * powers[18]; // pow(trace_generator, 13).
    powers[20] = powers[4] * powers[19]; // pow(trace_generator, 14).
    powers[21] = powers[4] * powers[20]; // pow(trace_generator, 15).
    powers[22] = powers[4] * powers[21]; // pow(trace_generator, 16).
    powers[23] = powers[2] * powers[22]; // pow(trace_generator, 784).
    powers[24] = powers[4] * powers[22]; // pow(trace_generator, 17).
    powers[25] = powers[1] * powers[24]; // pow(trace_generator, 1021).
    powers[26] = powers[4] * powers[24]; // pow(trace_generator, 18).
    powers[27] = powers[1] * powers[26]; // pow(trace_generator, 1022).
    powers[28] = powers[4] * powers[27]; // pow(trace_generator, 1023).
    powers[29] = powers[6] * powers[26]; // pow(trace_generator, 20).
    powers[30] = powers[6] * powers[29]; // pow(trace_generator, 22).
    powers[31] = powers[6] * powers[30]; // pow(trace_generator, 24).
    powers[32] = powers[4] * powers[31]; // pow(trace_generator, 25).
    powers[33] = powers[4] * powers[32]; // pow(trace_generator, 26).
    powers[34] = powers[1] * powers[29]; // pow(trace_generator, 1024).
    powers[35] = powers[25] * powers[34]; // pow(trace_generator, 2045).
    powers[36] = powers[4] * powers[34]; // pow(trace_generator, 1025).
    powers[37] = powers[6] * powers[36]; // pow(trace_generator, 1027).
    powers[38] = powers[4] * powers[33]; // pow(trace_generator, 27).
    powers[39] = powers[4] * powers[38]; // pow(trace_generator, 28).
    powers[40] = powers[6] * powers[39]; // pow(trace_generator, 30).
    powers[41] = powers[6] * powers[40]; // pow(trace_generator, 32).
    powers[42] = powers[4] * powers[41]; // pow(trace_generator, 33).
    powers[43] = powers[1] * powers[40]; // pow(trace_generator, 1034).
    powers[44] = powers[4] * powers[43]; // pow(trace_generator, 1035).
    powers[45] = powers[19] * powers[35]; // pow(trace_generator, 2058).
    powers[46] = powers[15] * powers[42]; // pow(trace_generator, 42).
    powers[47] = powers[4] * powers[46]; // pow(trace_generator, 43).
    powers[48] = powers[4] * powers[47]; // pow(trace_generator, 44).
    powers[49] = powers[20] * powers[48]; // pow(trace_generator, 58).
    powers[50] = powers[6] * powers[49]; // pow(trace_generator, 60).
    powers[51] = powers[2] * powers[29]; // pow(trace_generator, 788).
    powers[52] = powers[8] * powers[50]; // pow(trace_generator, 64).
    powers[53] = powers[4] * powers[52]; // pow(trace_generator, 65).
    powers[54] = powers[15] * powers[53]; // pow(trace_generator, 74).
    powers[55] = powers[4] * powers[54]; // pow(trace_generator, 75).
    powers[56] = powers[4] * powers[55]; // pow(trace_generator, 76).
    powers[57] = powers[18] * powers[56]; // pow(trace_generator, 88).
    powers[58] = powers[6] * powers[57]; // pow(trace_generator, 90).
    powers[59] = powers[4] * powers[58]; // pow(trace_generator, 91).
    powers[60] = powers[4] * powers[59]; // pow(trace_generator, 92).
    powers[61] = powers[6] * powers[60]; // pow(trace_generator, 94).
    powers[62] = powers[6] * powers[61]; // pow(trace_generator, 96).
    powers[63] = powers[4] * powers[62]; // pow(trace_generator, 97).
    powers[64] = powers[17] * powers[63]; // pow(trace_generator, 108).
    powers[65] = powers[18] * powers[64]; // pow(trace_generator, 120).
    powers[66] = powers[6] * powers[65]; // pow(trace_generator, 122).
    powers[67] = powers[4] * powers[66]; // pow(trace_generator, 123).
    powers[68] = powers[4] * powers[67]; // pow(trace_generator, 124).
    powers[69] = powers[6] * powers[68]; // pow(trace_generator, 126).
    powers[70] = powers[56] * powers[69]; // pow(trace_generator, 202).
    powers[71] = powers[39] * powers[69]; // pow(trace_generator, 154).

    // Fetch columns.
    // let column0 = column_values[0];
    // let column1 = column_values[1];
    // let column2 = column_values[2];
    // let column3 = column_values[3];
    // let column4 = column_values[4];
    // let column5 = column_values[5];
    // let column6 = column_values[6];
    // let column7 = column_values[7];
    // let column8 = column_values[8];
    // let column9 = column_values[9];

    // Sum the OODS constraints on the trace polynomials.
    let mut total_sum = FELT_0;
    let mut value;

    value = (column_values[0] - oods_values[0])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[0] * value;

    // return FELT_0; OK

    value = (column_values[0] - oods_values[1])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[1] * value;

    value = (column_values[0] - oods_values[2])
        .field_div(&felt_nonzero!(point - powers[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[2] * value;

    value = (column_values[0] - oods_values[3])
        .field_div(&felt_nonzero!(point - powers[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[3] * value;

    value = (column_values[0] - oods_values[4])
        .field_div(&felt_nonzero!(point - powers[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[4] * value;

    value = (column_values[0] - oods_values[5])
        .field_div(&felt_nonzero!(point - powers[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[5] * value;

    value = (column_values[0] - oods_values[6])
        .field_div(&felt_nonzero!(point - powers[12] * oods_point));
    total_sum = total_sum + constraint_coefficients[6] * value;

    value = (column_values[0] - oods_values[7])
        .field_div(&felt_nonzero!(point - powers[13] * oods_point));
    total_sum = total_sum + constraint_coefficients[7] * value;

    value = (column_values[0] - oods_values[8])
        .field_div(&felt_nonzero!(point - powers[14] * oods_point));
    total_sum = total_sum + constraint_coefficients[8] * value;

    value = (column_values[0] - oods_values[9])
        .field_div(&felt_nonzero!(point - powers[15] * oods_point));
    total_sum = total_sum + constraint_coefficients[9] * value;

    value = (column_values[0] - oods_values[10])
        .field_div(&felt_nonzero!(point - powers[16] * oods_point));
    total_sum = total_sum + constraint_coefficients[10] * value;

    // return FELT_0; OK

    value = (column_values[0] - oods_values[11])
        .field_div(&felt_nonzero!(point - powers[17] * oods_point));
    total_sum = total_sum + constraint_coefficients[11] * value;

    value = (column_values[0] - oods_values[12])
        .field_div(&felt_nonzero!(point - powers[18] * oods_point));
    total_sum = total_sum + constraint_coefficients[12] * value;

    value = (column_values[0] - oods_values[13])
        .field_div(&felt_nonzero!(point - powers[19] * oods_point));
    total_sum = total_sum + constraint_coefficients[13] * value;

    value = (column_values[0] - oods_values[14])
        .field_div(&felt_nonzero!(point - powers[20] * oods_point));
    total_sum = total_sum + constraint_coefficients[14] * value;

    value = (column_values[0] - oods_values[15])
        .field_div(&felt_nonzero!(point - powers[21] * oods_point));
    total_sum = total_sum + constraint_coefficients[15] * value;

    value = (column_values[0] - oods_values[16])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[16] * value;

    value = (column_values[0] - oods_values[17])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[17] * value;

    // return FELT_0; // OK

    value = (column_values[0] - oods_values[18])
        .field_div(&felt_nonzero!(point - powers[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[18] * value;

    value = (column_values[0] - oods_values[19])
        .field_div(&felt_nonzero!(point - powers[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[19] * value;

    value = (column_values[0] - oods_values[20])
        .field_div(&felt_nonzero!(point - powers[12] * oods_point));
    total_sum = total_sum + constraint_coefficients[20] * value;

    value = (column_values[0] - oods_values[21])
        .field_div(&felt_nonzero!(point - powers[14] * oods_point));
    total_sum = total_sum + constraint_coefficients[21] * value;

    value = (column_values[0] - oods_values[22])
        .field_div(&felt_nonzero!(point - powers[16] * oods_point));
    total_sum = total_sum + constraint_coefficients[22] * value;

    value = (column_values[0] - oods_values[23])
        .field_div(&felt_nonzero!(point - powers[18] * oods_point));
    total_sum = total_sum + constraint_coefficients[23] * value;

    value = (column_values[0] - oods_values[24])
        .field_div(&felt_nonzero!(point - powers[20] * oods_point));
    total_sum = total_sum + constraint_coefficients[24] * value;

    value = (column_values[0] - oods_values[25])
        .field_div(&felt_nonzero!(point - powers[22] * oods_point));
    total_sum = total_sum + constraint_coefficients[25] * value;

    value = (column_values[0] - oods_values[26])
        .field_div(&felt_nonzero!(point - powers[26] * oods_point));
    total_sum = total_sum + constraint_coefficients[26] * value;

    value = (column_values[0] - oods_values[27])
        .field_div(&felt_nonzero!(point - powers[29] * oods_point));
    total_sum = total_sum + constraint_coefficients[27] * value;

    value = (column_values[0] - oods_values[28])
        .field_div(&felt_nonzero!(point - powers[30] * oods_point));
    total_sum = total_sum + constraint_coefficients[28] * value;

    value = (column_values[0] - oods_values[29])
        .field_div(&felt_nonzero!(point - powers[31] * oods_point));
    total_sum = total_sum + constraint_coefficients[29] * value;

    // return FELT_0; // OK

    value = (column_values[0] - oods_values[30])
        .field_div(&felt_nonzero!(point - powers[33] * oods_point));
    total_sum = total_sum + constraint_coefficients[30] * value;

    value = (column_values[0] - oods_values[31])
        .field_div(&felt_nonzero!(point - powers[39] * oods_point));
    total_sum = total_sum + constraint_coefficients[31] * value;

    value = (column_values[0] - oods_values[32])
        .field_div(&felt_nonzero!(point - powers[40] * oods_point));
    total_sum = total_sum + constraint_coefficients[32] * value;

    value = (column_values[0] - oods_values[33])
        .field_div(&felt_nonzero!(point - powers[41] * oods_point));
    total_sum = total_sum + constraint_coefficients[33] * value;

    value = (column_values[0] - oods_values[34])
        .field_div(&felt_nonzero!(point - powers[42] * oods_point));
    total_sum = total_sum + constraint_coefficients[34] * value;

    value = (column_values[0] - oods_values[35])
        .field_div(&felt_nonzero!(point - powers[52] * oods_point));
    total_sum = total_sum + constraint_coefficients[35] * value;

    value = (column_values[0] - oods_values[36])
        .field_div(&felt_nonzero!(point - powers[53] * oods_point));
    total_sum = total_sum + constraint_coefficients[36] * value;

    value = (column_values[0] - oods_values[37])
        .field_div(&felt_nonzero!(point - powers[57] * oods_point));
    total_sum = total_sum + constraint_coefficients[37] * value;

    // return FELT_0; // OK

    value = (column_values[0] - oods_values[38])
        .field_div(&felt_nonzero!(point - powers[58] * oods_point));
    total_sum = total_sum + constraint_coefficients[38] * value;

    value = (column_values[0] - oods_values[39])
        .field_div(&felt_nonzero!(point - powers[60] * oods_point));
    total_sum = total_sum + constraint_coefficients[39] * value;

    value = (column_values[0] - oods_values[40])
        .field_div(&felt_nonzero!(point - powers[61] * oods_point));
    total_sum = total_sum + constraint_coefficients[40] * value;

    value = (column_values[0] - oods_values[41])
        .field_div(&felt_nonzero!(point - powers[62] * oods_point));
    total_sum = total_sum + constraint_coefficients[41] * value;

    value = (column_values[0] - oods_values[42])
        .field_div(&felt_nonzero!(point - powers[63] * oods_point));
    total_sum = total_sum + constraint_coefficients[42] * value;

    value = (column_values[0] - oods_values[43])
        .field_div(&felt_nonzero!(point - powers[65] * oods_point));
    total_sum = total_sum + constraint_coefficients[43] * value;

    value = (column_values[0] - oods_values[44])
        .field_div(&felt_nonzero!(point - powers[66] * oods_point));
    total_sum = total_sum + constraint_coefficients[44] * value;

    value = (column_values[0] - oods_values[45])
        .field_div(&felt_nonzero!(point - powers[68] * oods_point));
    total_sum = total_sum + constraint_coefficients[45] * value;

    value = (column_values[0] - oods_values[46])
        .field_div(&felt_nonzero!(point - powers[69] * oods_point));
    total_sum = total_sum + constraint_coefficients[46] * value;

    value = (column_values[0] - oods_values[47])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[47] * value;

    value = (column_values[2] - oods_values[48])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[48] * value;

    value = (column_values[0] - oods_values[49])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[49] * value;

    value = (column_values[0] - oods_values[50])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[50] * value;

    // return FELT_0; OK

    value = (column_values[0] - oods_values[51])
        .field_div(&felt_nonzero!(point - powers[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[51] * value;

    value = (column_values[0] - oods_values[52])
        .field_div(&felt_nonzero!(point - powers[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[52] * value;

    value = (column_values[0] - oods_values[53])
        .field_div(&felt_nonzero!(point - powers[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[53] * value;

    value = (column_values[0] - oods_values[54])
        .field_div(&felt_nonzero!(point - powers[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[54] * value;

    value = (column_values[0] - oods_values[55])
        .field_div(&felt_nonzero!(point - powers[14] * oods_point));
    total_sum = total_sum + constraint_coefficients[55] * value;

    // return FELT_0; // OK

    value = (column_values[0] - oods_values[56])
        .field_div(&felt_nonzero!(point - powers[15] * oods_point));
    total_sum = total_sum + constraint_coefficients[56] * value;

    value = (column_values[0] - oods_values[57])
        .field_div(&felt_nonzero!(point - powers[16] * oods_point));
    total_sum = total_sum + constraint_coefficients[57] * value;

    // return FELT_0; // OK

    value = (column_values[0] - oods_values[58])
        .field_div(&felt_nonzero!(point - powers[17] * oods_point));
    total_sum = total_sum + constraint_coefficients[58] * value;

    // return FELT_0; // Access violation in unknown section at address 0x10 of size 8

    value = (column_values[0] - oods_values[59])
        .field_div(&felt_nonzero!(point - powers[18] * oods_point));
    total_sum = total_sum + constraint_coefficients[59] * value;

    value = (column_values[0] - oods_values[60])
        .field_div(&felt_nonzero!(point - powers[19] * oods_point));
    total_sum = total_sum + constraint_coefficients[60] * value;

    value = (column_values[0] - oods_values[61])
        .field_div(&felt_nonzero!(point - powers[22] * oods_point));
    total_sum = total_sum + constraint_coefficients[61] * value;

    value = (column_values[0] - oods_values[62])
        .field_div(&felt_nonzero!(point - powers[33] * oods_point));
    total_sum = total_sum + constraint_coefficients[62] * value;

    value = (column_values[0] - oods_values[63])
        .field_div(&felt_nonzero!(point - powers[38] * oods_point));
    total_sum = total_sum + constraint_coefficients[63] * value;

    value = (column_values[0] - oods_values[64])
        .field_div(&felt_nonzero!(point - powers[46] * oods_point));
    total_sum = total_sum + constraint_coefficients[64] * value;

    value = (column_values[0] - oods_values[65])
        .field_div(&felt_nonzero!(point - powers[47] * oods_point));
    total_sum = total_sum + constraint_coefficients[65] * value;

    value = (column_values[0] - oods_values[66])
        .field_div(&felt_nonzero!(point - powers[49] * oods_point));
    total_sum = total_sum + constraint_coefficients[66] * value;

    value = (column_values[0] - oods_values[67])
        .field_div(&felt_nonzero!(point - powers[54] * oods_point));
    total_sum = total_sum + constraint_coefficients[67] * value;

    value = (column_values[0] - oods_values[68])
        .field_div(&felt_nonzero!(point - powers[55] * oods_point));
    total_sum = total_sum + constraint_coefficients[68] * value;

    value = (column_values[0] - oods_values[69])
        .field_div(&felt_nonzero!(point - powers[59] * oods_point));
    total_sum = total_sum + constraint_coefficients[69] * value;

    value = (column_values[0] - oods_values[70])
        .field_div(&felt_nonzero!(point - powers[66] * oods_point));
    total_sum = total_sum + constraint_coefficients[70] * value;

    value = (column_values[0] - oods_values[71])
        .field_div(&felt_nonzero!(point - powers[67] * oods_point));
    total_sum = total_sum + constraint_coefficients[71] * value;

    value = (column_values[0] - oods_values[72])
        .field_div(&felt_nonzero!(point - powers[71] * oods_point));
    total_sum = total_sum + constraint_coefficients[72] * value;

    value = (column_values[0] - oods_values[73])
        .field_div(&felt_nonzero!(point - powers[70] * oods_point));
    total_sum = total_sum + constraint_coefficients[73] * value;

    value = (column_values[0] - oods_values[74])
        .field_div(&felt_nonzero!(point - powers[3] * oods_point));
    total_sum = total_sum + constraint_coefficients[74] * value;

    value = (column_values[0] - oods_values[75])
        .field_div(&felt_nonzero!(point - powers[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[75] * value;

    // return FELT_0; // VBAD

    value = (column_values[0] - oods_values[76])
        .field_div(&felt_nonzero!(point - powers[43] * oods_point));
    total_sum = total_sum + constraint_coefficients[76] * value;

    value = (column_values[0] - oods_values[77])
        .field_div(&felt_nonzero!(point - powers[44] * oods_point));
    total_sum = total_sum + constraint_coefficients[77] * value;

    value = (column_values[0] - oods_values[78])
        .field_div(&felt_nonzero!(point - powers[45] * oods_point));
    total_sum = total_sum + constraint_coefficients[78] * value;

    value = (column_values[0] - oods_values[79])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[79] * value;

    value = (column_values[0] - oods_values[80])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[80] * value;

    value = (column_values[0] - oods_values[81])
        .field_div(&felt_nonzero!(point - powers[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[81] * value;

    value = (column_values[0] - oods_values[82])
        .field_div(&felt_nonzero!(point - powers[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[82] * value;

    value = (column_values[0] - oods_values[83])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[83] * value;

    value = (column_values[0] - oods_values[84])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[84] * value;

    value = (column_values[0] - oods_values[85])
        .field_div(&felt_nonzero!(point - powers[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[85] * value;

    value = (column_values[0] - oods_values[86])
        .field_div(&felt_nonzero!(point - powers[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[86] * value;

    value = (column_values[0] - oods_values[87])
        .field_div(&felt_nonzero!(point - powers[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[87] * value;

    value = (column_values[0] - oods_values[88])
        .field_div(&felt_nonzero!(point - powers[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[88] * value;

    value = (column_values[0] - oods_values[89])
        .field_div(&felt_nonzero!(point - powers[12] * oods_point));
    total_sum = total_sum + constraint_coefficients[89] * value;

    value = (column_values[0] - oods_values[90])
        .field_div(&felt_nonzero!(point - powers[13] * oods_point));
    total_sum = total_sum + constraint_coefficients[90] * value;

    value = (column_values[0] - oods_values[91])
        .field_div(&felt_nonzero!(point - powers[14] * oods_point));
    total_sum = total_sum + constraint_coefficients[91] * value;

    value = (column_values[0] - oods_values[92])
        .field_div(&felt_nonzero!(point - powers[18] * oods_point));
    total_sum = total_sum + constraint_coefficients[92] * value;

    value = (column_values[0] - oods_values[93])
        .field_div(&felt_nonzero!(point - powers[39] * oods_point));
    total_sum = total_sum + constraint_coefficients[93] * value;

    value = (column_values[0] - oods_values[94])
        .field_div(&felt_nonzero!(point - powers[48] * oods_point));
    total_sum = total_sum + constraint_coefficients[94] * value;

    value = (column_values[0] - oods_values[95])
        .field_div(&felt_nonzero!(point - powers[50] * oods_point));
    total_sum = total_sum + constraint_coefficients[95] * value;

    value = (column_values[0] - oods_values[96])
        .field_div(&felt_nonzero!(point - powers[56] * oods_point));
    total_sum = total_sum + constraint_coefficients[96] * value;

    value = (column_values[0] - oods_values[97])
        .field_div(&felt_nonzero!(point - powers[60] * oods_point));
    total_sum = total_sum + constraint_coefficients[97] * value;

    value = (column_values[0] - oods_values[98])
        .field_div(&felt_nonzero!(point - powers[64] * oods_point));
    total_sum = total_sum + constraint_coefficients[98] * value;

    // return FELT_0; // VBAD

    value = (column_values[0] - oods_values[99])
        .field_div(&felt_nonzero!(point - powers[68] * oods_point));
    total_sum = total_sum + constraint_coefficients[99] * value;

    value = (column_values[0] - oods_values[100])
        .field_div(&felt_nonzero!(point - powers[25] * oods_point));
    total_sum = total_sum + constraint_coefficients[100] * value;

    value = (column_values[0] - oods_values[101])
        .field_div(&felt_nonzero!(point - powers[28] * oods_point));
    total_sum = total_sum + constraint_coefficients[101] * value;

    value = (column_values[0] - oods_values[102])
        .field_div(&felt_nonzero!(point - powers[36] * oods_point));
    total_sum = total_sum + constraint_coefficients[102] * value;

    value = (column_values[0] - oods_values[103])
        .field_div(&felt_nonzero!(point - powers[37] * oods_point));
    total_sum = total_sum + constraint_coefficients[103] * value;

    value = (column_values[0] - oods_values[104])
        .field_div(&felt_nonzero!(point - powers[35] * oods_point));
    total_sum = total_sum + constraint_coefficients[104] * value;

    value = (column_values[0] - oods_values[105])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[105] * value;

    value = (column_values[0] - oods_values[106])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[106] * value;

    value = (column_values[0] - oods_values[107])
        .field_div(&felt_nonzero!(point - powers[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[107] * value;

    value = (column_values[0] - oods_values[108])
        .field_div(&felt_nonzero!(point - powers[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[108] * value;

    value = (column_values[0] - oods_values[109])
        .field_div(&felt_nonzero!(point - powers[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[109] * value;

    value = (column_values[0] - oods_values[110])
        .field_div(&felt_nonzero!(point - powers[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[110] * value;

    value = (column_values[0] - oods_values[111])
        .field_div(&felt_nonzero!(point - powers[13] * oods_point));
    total_sum = total_sum + constraint_coefficients[111] * value;

    value = (column_values[0] - oods_values[112])
        .field_div(&felt_nonzero!(point - powers[15] * oods_point));
    total_sum = total_sum + constraint_coefficients[112] * value;

    value = (column_values[0] - oods_values[113])
        .field_div(&felt_nonzero!(point - powers[17] * oods_point));
    total_sum = total_sum + constraint_coefficients[113] * value;

    value = (column_values[0] - oods_values[114])
        .field_div(&felt_nonzero!(point - powers[19] * oods_point));
    total_sum = total_sum + constraint_coefficients[114] * value;

    value = (column_values[0] - oods_values[115])
        .field_div(&felt_nonzero!(point - powers[24] * oods_point));
    total_sum = total_sum + constraint_coefficients[115] * value;

    value = (column_values[0] - oods_values[116])
        .field_div(&felt_nonzero!(point - powers[32] * oods_point));
    total_sum = total_sum + constraint_coefficients[116] * value;

    value = (column_values[0] - oods_values[117])
        .field_div(&felt_nonzero!(point - powers[2] * oods_point));
    total_sum = total_sum + constraint_coefficients[117] * value;

    value = (column_values[0] - oods_values[118])
        .field_div(&felt_nonzero!(point - powers[10] * oods_point));
    total_sum = total_sum + constraint_coefficients[118] * value;

    // return FELT_0; // VBAD

    value = (column_values[0] - oods_values[119])
        .field_div(&felt_nonzero!(point - powers[23] * oods_point));
    total_sum = total_sum + constraint_coefficients[119] * value;

    value = (column_values[0] - oods_values[120])
        .field_div(&felt_nonzero!(point - powers[51] * oods_point));
    total_sum = total_sum + constraint_coefficients[120] * value;

    value = (column_values[0] - oods_values[121])
        .field_div(&felt_nonzero!(point - powers[1] * oods_point));
    total_sum = total_sum + constraint_coefficients[121] * value;

    value = (column_values[0] - oods_values[122])
        .field_div(&felt_nonzero!(point - powers[9] * oods_point));
    total_sum = total_sum + constraint_coefficients[122] * value;

    value = (column_values[0] - oods_values[123])
        .field_div(&felt_nonzero!(point - powers[27] * oods_point));
    total_sum = total_sum + constraint_coefficients[123] * value;

    value = (column_values[0] - oods_values[124])
        .field_div(&felt_nonzero!(point - powers[34] * oods_point));
    total_sum = total_sum + constraint_coefficients[124] * value;

    value = (column_values[0] - oods_values[125])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[125] * value;

    value = (column_values[0] - oods_values[126])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[126] * value;

    value = (column_values[0] - oods_values[127])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[127] * value;

    value = (column_values[0] - oods_values[128])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[128] * value;

    value = (column_values[0] - oods_values[129])
        .field_div(&felt_nonzero!(point - powers[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[129] * value;

    value = (column_values[0] - oods_values[130])
        .field_div(&felt_nonzero!(point - powers[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[130] * value;

    value = (column_values[0] - oods_values[131])
        .field_div(&felt_nonzero!(point - powers[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[131] * value;

    value = (column_values[0] - oods_values[132])
        .field_div(&felt_nonzero!(point - powers[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[132] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow_felt(&(Layout::CONSTRAINT_DEGREE.into()));

    value = (column_values
        [Layout::NUM_COLUMNS_FIRST as usize + Layout::NUM_COLUMNS_SECOND as usize]
        - oods_values[133])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));
    total_sum = total_sum + constraint_coefficients[133] * value;

    value = (column_values
        [Layout::NUM_COLUMNS_FIRST as usize + Layout::NUM_COLUMNS_SECOND as usize + 1]
        - oods_values[134])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));

    let result = total_sum + constraint_coefficients[134] * value;

    result
}
