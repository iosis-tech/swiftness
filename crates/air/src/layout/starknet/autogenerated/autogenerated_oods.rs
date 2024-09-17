use crate::{
    consts::*,
    felt_nonzero,
    layout::starknet::{LayoutTrait, StaticLayoutTrait},
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
    let pow1 = trace_generator.pow_felt(&(FELT_32715));
    let pow2 = trace_generator.pow_felt(&(FELT_32667));
    let pow3 = trace_generator.pow_felt(&(FELT_32647));
    let pow4 = trace_generator.pow_felt(&(FELT_16325));
    let pow5 = trace_generator.pow_felt(&(FELT_16149));
    let pow6 = trace_generator.pow_felt(&(FELT_16085));
    let pow7 = trace_generator.pow_felt(&(FELT_12373));
    let pow8 = trace_generator.pow_felt(&(FELT_12309));
    let pow9 = trace_generator.pow_felt(&(FELT_24966));
    let pow10 = trace_generator.pow_felt(&(FELT_16774));
    let pow11 = trace_generator.pow_felt(&(FELT_14726));
    let pow12 = trace_generator.pow_felt(&(FELT_10630));
    let pow13 = trace_generator.pow_felt(&(FELT_8582));
    let pow14 = trace_generator.pow_felt(&(FELT_6534));
    let pow15 = trace_generator.pow_felt(&(FELT_4486));
    let pow16 = trace_generator.pow_felt(&(FELT_2438));
    let pow17 = trace_generator.pow_felt(&(FELT_1));
    let pow18 = pow11 * pow17; // pow(trace_generator, 14727).
    let pow19 = pow12 * pow17; // pow(trace_generator, 10631).
    let pow20 = pow13 * pow17; // pow(trace_generator, 8583).
    let pow21 = pow14 * pow17; // pow(trace_generator, 6535).
    let pow22 = pow15 * pow17; // pow(trace_generator, 4487).
    let pow23 = pow16 * pow17; // pow(trace_generator, 2439).
    let pow24 = pow17 * pow17; // pow(trace_generator, 2).
    let pow25 = pow17 * pow24; // pow(trace_generator, 3).
    let pow26 = pow17 * pow25; // pow(trace_generator, 4).
    let pow27 = pow17 * pow26; // pow(trace_generator, 5).
    let pow28 = pow17 * pow27; // pow(trace_generator, 6).
    let pow29 = pow4 * pow28; // pow(trace_generator, 16331).
    let pow30 = pow17 * pow28; // pow(trace_generator, 7).
    let pow31 = pow17 * pow30; // pow(trace_generator, 8).
    let pow32 = pow17 * pow31; // pow(trace_generator, 9).
    let pow33 = pow17 * pow32; // pow(trace_generator, 10).
    let pow34 = pow17 * pow33; // pow(trace_generator, 11).
    let pow35 = pow17 * pow34; // pow(trace_generator, 12).
    let pow36 = pow17 * pow35; // pow(trace_generator, 13).
    let pow37 = pow17 * pow36; // pow(trace_generator, 14).
    let pow38 = pow17 * pow37; // pow(trace_generator, 15).
    let pow39 = pow17 * pow38; // pow(trace_generator, 16).
    let pow40 = pow17 * pow39; // pow(trace_generator, 17).
    let pow41 = pow24 * pow40; // pow(trace_generator, 19).
    let pow42 = pow24 * pow41; // pow(trace_generator, 21).
    let pow43 = pow17 * pow42; // pow(trace_generator, 22).
    let pow44 = pow17 * pow43; // pow(trace_generator, 23).
    let pow45 = pow17 * pow44; // pow(trace_generator, 24).
    let pow46 = pow17 * pow45; // pow(trace_generator, 25).
    let pow47 = pow24 * pow46; // pow(trace_generator, 27).
    let pow48 = pow24 * pow47; // pow(trace_generator, 29).
    let pow49 = pow17 * pow48; // pow(trace_generator, 30).
    let pow50 = pow25 * pow49; // pow(trace_generator, 33).
    let pow51 = pow24 * pow50; // pow(trace_generator, 35).
    let pow52 = pow24 * pow51; // pow(trace_generator, 37).
    let pow53 = pow17 * pow52; // pow(trace_generator, 38).
    let pow54 = pow17 * pow53; // pow(trace_generator, 39).
    let pow55 = pow24 * pow54; // pow(trace_generator, 41).
    let pow56 = pow24 * pow55; // pow(trace_generator, 43).
    let pow57 = pow17 * pow56; // pow(trace_generator, 44).
    let pow58 = pow17 * pow57; // pow(trace_generator, 45).
    let pow59 = pow17 * pow58; // pow(trace_generator, 46).
    let pow60 = pow25 * pow59; // pow(trace_generator, 49).
    let pow61 = pow24 * pow60; // pow(trace_generator, 51).
    let pow62 = pow24 * pow61; // pow(trace_generator, 53).
    let pow63 = pow17 * pow62; // pow(trace_generator, 54).
    let pow64 = pow1 * pow28; // pow(trace_generator, 32721).
    let pow65 = pow1 * pow39; // pow(trace_generator, 32731).
    let pow66 = pow39 * pow65; // pow(trace_generator, 32747).
    let pow67 = pow1 * pow53; // pow(trace_generator, 32753).
    let pow68 = pow33 * pow67; // pow(trace_generator, 32763).
    let pow69 = pow25 * pow63; // pow(trace_generator, 57).
    let pow70 = pow24 * pow69; // pow(trace_generator, 59).
    let pow71 = pow24 * pow70; // pow(trace_generator, 61).
    let pow72 = pow26 * pow71; // pow(trace_generator, 65).
    let pow73 = pow26 * pow72; // pow(trace_generator, 69).
    let pow74 = pow17 * pow73; // pow(trace_generator, 70).
    let pow75 = pow17 * pow74; // pow(trace_generator, 71).
    let pow76 = pow24 * pow75; // pow(trace_generator, 73).
    let pow77 = pow25 * pow76; // pow(trace_generator, 76).
    let pow78 = pow17 * pow77; // pow(trace_generator, 77).
    let pow79 = pow26 * pow78; // pow(trace_generator, 81).
    let pow80 = pow26 * pow79; // pow(trace_generator, 85).
    let pow81 = pow26 * pow80; // pow(trace_generator, 89).
    let pow82 = pow24 * pow81; // pow(trace_generator, 91).
    let pow83 = pow28 * pow82; // pow(trace_generator, 97).
    let pow84 = pow26 * pow83; // pow(trace_generator, 101).
    let pow85 = pow17 * pow84; // pow(trace_generator, 102).
    let pow86 = pow17 * pow85; // pow(trace_generator, 103).
    let pow87 = pow24 * pow86; // pow(trace_generator, 105).
    let pow88 = pow25 * pow87; // pow(trace_generator, 108).
    let pow89 = pow17 * pow88; // pow(trace_generator, 109).
    let pow90 = pow26 * pow89; // pow(trace_generator, 113).
    let pow91 = pow26 * pow90; // pow(trace_generator, 117).
    let pow92 = pow28 * pow91; // pow(trace_generator, 123).
    let pow93 = pow28 * pow92; // pow(trace_generator, 129).
    let pow94 = pow27 * pow93; // pow(trace_generator, 134).
    let pow95 = pow17 * pow94; // pow(trace_generator, 135).
    let pow96 = pow27 * pow95; // pow(trace_generator, 140).
    let pow97 = pow27 * pow96; // pow(trace_generator, 145).
    let pow98 = pow33 * pow97; // pow(trace_generator, 155).
    let pow99 = pow28 * pow98; // pow(trace_generator, 161).
    let pow100 = pow27 * pow99; // pow(trace_generator, 166).
    let pow101 = pow17 * pow100; // pow(trace_generator, 167).
    let pow102 = pow27 * pow101; // pow(trace_generator, 172).
    let pow103 = pow27 * pow102; // pow(trace_generator, 177).
    let pow104 = pow33 * pow103; // pow(trace_generator, 187).
    let pow105 = pow27 * pow104; // pow(trace_generator, 192).
    let pow106 = pow17 * pow105; // pow(trace_generator, 193).
    let pow107 = pow24 * pow106; // pow(trace_generator, 195).
    let pow108 = pow17 * pow107; // pow(trace_generator, 196).
    let pow109 = pow17 * pow108; // pow(trace_generator, 197).
    let pow110 = pow17 * pow109; // pow(trace_generator, 198).
    let pow111 = pow17 * pow110; // pow(trace_generator, 199).
    let pow112 = pow27 * pow111; // pow(trace_generator, 204).
    let pow113 = pow17 * pow112; // pow(trace_generator, 205).
    let pow114 = pow26 * pow113; // pow(trace_generator, 209).
    let pow115 = pow33 * pow114; // pow(trace_generator, 219).
    let pow116 = pow24 * pow115; // pow(trace_generator, 221).
    let pow117 = pow26 * pow116; // pow(trace_generator, 225).
    let pow118 = pow34 * pow117; // pow(trace_generator, 236).
    let pow119 = pow17 * pow118; // pow(trace_generator, 237).
    let pow120 = pow26 * pow119; // pow(trace_generator, 241).
    let pow121 = pow26 * pow120; // pow(trace_generator, 245).
    let pow122 = pow28 * pow121; // pow(trace_generator, 251).
    let pow123 = pow17 * pow122; // pow(trace_generator, 252).
    let pow124 = pow4 * pow35; // pow(trace_generator, 16337).
    let pow125 = pow4 * pow37; // pow(trace_generator, 16339).
    let pow126 = pow4 * pow49; // pow(trace_generator, 16355).
    let pow127 = pow24 * pow126; // pow(trace_generator, 16357).
    let pow128 = pow4 * pow53; // pow(trace_generator, 16363).
    let pow129 = pow4 * pow57; // pow(trace_generator, 16369).
    let pow130 = pow4 * pow59; // pow(trace_generator, 16371).
    let pow131 = pow5 * pow118; // pow(trace_generator, 16385).
    let pow132 = pow59 * pow130; // pow(trace_generator, 16417).
    let pow133 = pow17 * pow123; // pow(trace_generator, 253).
    let pow134 = pow24 * pow133; // pow(trace_generator, 255).
    let pow135 = pow17 * pow134; // pow(trace_generator, 256).
    let pow136 = pow17 * pow135; // pow(trace_generator, 257).
    let pow137 = pow7 * pow135; // pow(trace_generator, 12629).
    let pow138 = pow7 * pow105; // pow(trace_generator, 12565).
    let pow139 = pow60 * pow137; // pow(trace_generator, 12678).
    let pow140 = pow17 * pow139; // pow(trace_generator, 12679).
    let pow141 = pow27 * pow136; // pow(trace_generator, 262).
    let pow142 = pow17 * pow141; // pow(trace_generator, 263).
    let pow143 = pow24 * pow142; // pow(trace_generator, 265).
    let pow144 = pow26 * pow143; // pow(trace_generator, 269).
    let pow145 = pow46 * pow144; // pow(trace_generator, 294).
    let pow146 = pow17 * pow145; // pow(trace_generator, 295).
    let pow147 = pow28 * pow146; // pow(trace_generator, 301).
    let pow148 = pow31 * pow147; // pow(trace_generator, 309).
    let pow149 = pow17 * pow148; // pow(trace_generator, 310).
    let pow150 = pow31 * pow149; // pow(trace_generator, 318).
    let pow151 = pow90 * pow148; // pow(trace_generator, 422).
    let pow152 = pow79 * pow148; // pow(trace_generator, 390).
    let pow153 = pow31 * pow150; // pow(trace_generator, 326).
    let pow154 = pow31 * pow153; // pow(trace_generator, 334).
    let pow155 = pow31 * pow154; // pow(trace_generator, 342).
    let pow156 = pow31 * pow155; // pow(trace_generator, 350).
    let pow157 = pow31 * pow156; // pow(trace_generator, 358).
    let pow158 = pow17 * pow151; // pow(trace_generator, 423).
    let pow159 = pow17 * pow152; // pow(trace_generator, 391).
    let pow160 = pow17 * pow157; // pow(trace_generator, 359).
    let pow161 = pow10 * pow17; // pow(trace_generator, 16775).
    let pow162 = pow48 * pow151; // pow(trace_generator, 451).
    let pow163 = pow25 * pow162; // pow(trace_generator, 454).
    let pow164 = pow30 * pow163; // pow(trace_generator, 461).
    let pow165 = pow39 * pow164; // pow(trace_generator, 477).
    let pow166 = pow37 * pow165; // pow(trace_generator, 491).
    let pow167 = pow24 * pow166; // pow(trace_generator, 493).
    let pow168 = pow28 * pow167; // pow(trace_generator, 499).
    let pow169 = pow24 * pow168; // pow(trace_generator, 501).
    let pow170 = pow28 * pow169; // pow(trace_generator, 507).
    let pow171 = pow24 * pow170; // pow(trace_generator, 509).
    let pow172 = pow24 * pow171; // pow(trace_generator, 511).
    let pow173 = pow2 * pow166; // pow(trace_generator, 33158).
    let pow174 = pow24 * pow172; // pow(trace_generator, 513).
    let pow175 = pow27 * pow174; // pow(trace_generator, 518).
    let pow176 = pow104 * pow175; // pow(trace_generator, 705).
    let pow177 = pow109 * pow176; // pow(trace_generator, 902).
    let pow178 = pow28 * pow176; // pow(trace_generator, 711).
    let pow179 = pow33 * pow178; // pow(trace_generator, 721).
    let pow180 = pow39 * pow179; // pow(trace_generator, 737).
    let pow181 = pow39 * pow180; // pow(trace_generator, 753).
    let pow182 = pow39 * pow181; // pow(trace_generator, 769).
    let pow183 = pow70 * pow177; // pow(trace_generator, 961).
    let pow184 = pow27 * pow183; // pow(trace_generator, 966).
    let pow185 = pow17 * pow184; // pow(trace_generator, 967).
    let pow186 = pow33 * pow185; // pow(trace_generator, 977).
    let pow187 = pow121 * pow186; // pow(trace_generator, 1222).
    let pow188 = pow17 * pow177; // pow(trace_generator, 903).
    let pow189 = pow39 * pow186; // pow(trace_generator, 993).
    let pow190 = pow39 * pow189; // pow(trace_generator, 1009).
    let pow191 = pow25 * pow175; // pow(trace_generator, 521).
    let pow192 = pow31 * pow182; // pow(trace_generator, 777).

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

    let value = (column0 - oods_values[1]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[1] * value;

    let value = (column0 - oods_values[2]).field_div(&felt_nonzero!(point - pow24 * oods_point));
    let total_sum = total_sum + constraint_coefficients[2] * value;

    let value = (column0 - oods_values[3]).field_div(&felt_nonzero!(point - pow25 * oods_point));
    let total_sum = total_sum + constraint_coefficients[3] * value;

    let value = (column0 - oods_values[4]).field_div(&felt_nonzero!(point - pow26 * oods_point));
    let total_sum = total_sum + constraint_coefficients[4] * value;

    let value = (column0 - oods_values[5]).field_div(&felt_nonzero!(point - pow27 * oods_point));
    let total_sum = total_sum + constraint_coefficients[5] * value;

    let value = (column0 - oods_values[6]).field_div(&felt_nonzero!(point - pow28 * oods_point));
    let total_sum = total_sum + constraint_coefficients[6] * value;

    let value = (column0 - oods_values[7]).field_div(&felt_nonzero!(point - pow30 * oods_point));
    let total_sum = total_sum + constraint_coefficients[7] * value;

    let value = (column0 - oods_values[8]).field_div(&felt_nonzero!(point - pow31 * oods_point));
    let total_sum = total_sum + constraint_coefficients[8] * value;

    let value = (column0 - oods_values[9]).field_div(&felt_nonzero!(point - pow32 * oods_point));
    let total_sum = total_sum + constraint_coefficients[9] * value;

    let value = (column0 - oods_values[10]).field_div(&felt_nonzero!(point - pow33 * oods_point));
    let total_sum = total_sum + constraint_coefficients[10] * value;

    let value = (column0 - oods_values[11]).field_div(&felt_nonzero!(point - pow34 * oods_point));
    let total_sum = total_sum + constraint_coefficients[11] * value;

    let value = (column0 - oods_values[12]).field_div(&felt_nonzero!(point - pow35 * oods_point));
    let total_sum = total_sum + constraint_coefficients[12] * value;

    let value = (column0 - oods_values[13]).field_div(&felt_nonzero!(point - pow36 * oods_point));
    let total_sum = total_sum + constraint_coefficients[13] * value;

    let value = (column0 - oods_values[14]).field_div(&felt_nonzero!(point - pow37 * oods_point));
    let total_sum = total_sum + constraint_coefficients[14] * value;

    let value = (column0 - oods_values[15]).field_div(&felt_nonzero!(point - pow38 * oods_point));
    let total_sum = total_sum + constraint_coefficients[15] * value;

    let value = (column1 - oods_values[16]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[16] * value;

    let value = (column1 - oods_values[17]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[17] * value;

    let value =
        (column1 - oods_values[18]).field_div(&felt_nonzero!(point - pow134 * oods_point));
    let total_sum = total_sum + constraint_coefficients[18] * value;

    let value =
        (column1 - oods_values[19]).field_div(&felt_nonzero!(point - pow135 * oods_point));
    let total_sum = total_sum + constraint_coefficients[19] * value;

    let value =
        (column1 - oods_values[20]).field_div(&felt_nonzero!(point - pow172 * oods_point));
    let total_sum = total_sum + constraint_coefficients[20] * value;

    let value = (column2 - oods_values[21]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[21] * value;

    let value = (column2 - oods_values[22]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[22] * value;

    let value =
        (column2 - oods_values[23]).field_div(&felt_nonzero!(point - pow134 * oods_point));
    let total_sum = total_sum + constraint_coefficients[23] * value;

    let value =
        (column2 - oods_values[24]).field_div(&felt_nonzero!(point - pow135 * oods_point));
    let total_sum = total_sum + constraint_coefficients[24] * value;

    let value = (column3 - oods_values[25]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[25] * value;

    let value = (column3 - oods_values[26]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[26] * value;

    let value =
        (column3 - oods_values[27]).field_div(&felt_nonzero!(point - pow105 * oods_point));
    let total_sum = total_sum + constraint_coefficients[27] * value;

    let value =
        (column3 - oods_values[28]).field_div(&felt_nonzero!(point - pow106 * oods_point));
    let total_sum = total_sum + constraint_coefficients[28] * value;

    let value =
        (column3 - oods_values[29]).field_div(&felt_nonzero!(point - pow108 * oods_point));
    let total_sum = total_sum + constraint_coefficients[29] * value;

    let value =
        (column3 - oods_values[30]).field_div(&felt_nonzero!(point - pow109 * oods_point));
    let total_sum = total_sum + constraint_coefficients[30] * value;

    let value =
        (column3 - oods_values[31]).field_div(&felt_nonzero!(point - pow122 * oods_point));
    let total_sum = total_sum + constraint_coefficients[31] * value;

    let value =
        (column3 - oods_values[32]).field_div(&felt_nonzero!(point - pow123 * oods_point));
    let total_sum = total_sum + constraint_coefficients[32] * value;

    let value =
        (column3 - oods_values[33]).field_div(&felt_nonzero!(point - pow135 * oods_point));
    let total_sum = total_sum + constraint_coefficients[33] * value;

    let value = (column4 - oods_values[34]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[34] * value;

    let value =
        (column4 - oods_values[35]).field_div(&felt_nonzero!(point - pow134 * oods_point));
    let total_sum = total_sum + constraint_coefficients[35] * value;

    let value = (column5 - oods_values[36]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[36] * value;

    let value = (column5 - oods_values[37]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[37] * value;

    let value = (column5 - oods_values[38]).field_div(&felt_nonzero!(point - pow24 * oods_point));
    let total_sum = total_sum + constraint_coefficients[38] * value;

    let value = (column5 - oods_values[39]).field_div(&felt_nonzero!(point - pow25 * oods_point));
    let total_sum = total_sum + constraint_coefficients[39] * value;

    let value = (column5 - oods_values[40]).field_div(&felt_nonzero!(point - pow26 * oods_point));
    let total_sum = total_sum + constraint_coefficients[40] * value;

    let value = (column5 - oods_values[41]).field_div(&felt_nonzero!(point - pow27 * oods_point));
    let total_sum = total_sum + constraint_coefficients[41] * value;

    let value = (column5 - oods_values[42]).field_div(&felt_nonzero!(point - pow28 * oods_point));
    let total_sum = total_sum + constraint_coefficients[42] * value;

    let value = (column5 - oods_values[43]).field_div(&felt_nonzero!(point - pow30 * oods_point));
    let total_sum = total_sum + constraint_coefficients[43] * value;

    let value = (column5 - oods_values[44]).field_div(&felt_nonzero!(point - pow31 * oods_point));
    let total_sum = total_sum + constraint_coefficients[44] * value;

    let value = (column5 - oods_values[45]).field_div(&felt_nonzero!(point - pow32 * oods_point));
    let total_sum = total_sum + constraint_coefficients[45] * value;

    let value = (column5 - oods_values[46]).field_div(&felt_nonzero!(point - pow35 * oods_point));
    let total_sum = total_sum + constraint_coefficients[46] * value;

    let value = (column5 - oods_values[47]).field_div(&felt_nonzero!(point - pow36 * oods_point));
    let total_sum = total_sum + constraint_coefficients[47] * value;

    let value = (column5 - oods_values[48]).field_div(&felt_nonzero!(point - pow39 * oods_point));
    let total_sum = total_sum + constraint_coefficients[48] * value;

    let value = (column5 - oods_values[49]).field_div(&felt_nonzero!(point - pow53 * oods_point));
    let total_sum = total_sum + constraint_coefficients[49] * value;

    let value = (column5 - oods_values[50]).field_div(&felt_nonzero!(point - pow54 * oods_point));
    let total_sum = total_sum + constraint_coefficients[50] * value;

    let value = (column5 - oods_values[51]).field_div(&felt_nonzero!(point - pow74 * oods_point));
    let total_sum = total_sum + constraint_coefficients[51] * value;

    let value = (column5 - oods_values[52]).field_div(&felt_nonzero!(point - pow75 * oods_point));
    let total_sum = total_sum + constraint_coefficients[52] * value;

    let value = (column5 - oods_values[53]).field_div(&felt_nonzero!(point - pow85 * oods_point));
    let total_sum = total_sum + constraint_coefficients[53] * value;

    let value = (column5 - oods_values[54]).field_div(&felt_nonzero!(point - pow86 * oods_point));
    let total_sum = total_sum + constraint_coefficients[54] * value;

    let value = (column5 - oods_values[55]).field_div(&felt_nonzero!(point - pow94 * oods_point));
    let total_sum = total_sum + constraint_coefficients[55] * value;

    let value = (column5 - oods_values[56]).field_div(&felt_nonzero!(point - pow95 * oods_point));
    let total_sum = total_sum + constraint_coefficients[56] * value;

    let value =
        (column5 - oods_values[57]).field_div(&felt_nonzero!(point - pow100 * oods_point));
    let total_sum = total_sum + constraint_coefficients[57] * value;

    let value =
        (column5 - oods_values[58]).field_div(&felt_nonzero!(point - pow101 * oods_point));
    let total_sum = total_sum + constraint_coefficients[58] * value;

    let value =
        (column5 - oods_values[59]).field_div(&felt_nonzero!(point - pow110 * oods_point));
    let total_sum = total_sum + constraint_coefficients[59] * value;

    let value =
        (column5 - oods_values[60]).field_div(&felt_nonzero!(point - pow111 * oods_point));
    let total_sum = total_sum + constraint_coefficients[60] * value;

    let value =
        (column5 - oods_values[61]).field_div(&felt_nonzero!(point - pow141 * oods_point));
    let total_sum = total_sum + constraint_coefficients[61] * value;

    let value =
        (column5 - oods_values[62]).field_div(&felt_nonzero!(point - pow142 * oods_point));
    let total_sum = total_sum + constraint_coefficients[62] * value;

    let value =
        (column5 - oods_values[63]).field_div(&felt_nonzero!(point - pow145 * oods_point));
    let total_sum = total_sum + constraint_coefficients[63] * value;

    let value =
        (column5 - oods_values[64]).field_div(&felt_nonzero!(point - pow146 * oods_point));
    let total_sum = total_sum + constraint_coefficients[64] * value;

    let value =
        (column5 - oods_values[65]).field_div(&felt_nonzero!(point - pow153 * oods_point));
    let total_sum = total_sum + constraint_coefficients[65] * value;

    let value =
        (column5 - oods_values[66]).field_div(&felt_nonzero!(point - pow157 * oods_point));
    let total_sum = total_sum + constraint_coefficients[66] * value;

    let value =
        (column5 - oods_values[67]).field_div(&felt_nonzero!(point - pow160 * oods_point));
    let total_sum = total_sum + constraint_coefficients[67] * value;

    let value =
        (column5 - oods_values[68]).field_div(&felt_nonzero!(point - pow152 * oods_point));
    let total_sum = total_sum + constraint_coefficients[68] * value;

    let value =
        (column5 - oods_values[69]).field_div(&felt_nonzero!(point - pow159 * oods_point));
    let total_sum = total_sum + constraint_coefficients[69] * value;

    let value =
        (column5 - oods_values[70]).field_div(&felt_nonzero!(point - pow151 * oods_point));
    let total_sum = total_sum + constraint_coefficients[70] * value;

    let value =
        (column5 - oods_values[71]).field_div(&felt_nonzero!(point - pow158 * oods_point));
    let total_sum = total_sum + constraint_coefficients[71] * value;

    let value =
        (column5 - oods_values[72]).field_div(&felt_nonzero!(point - pow163 * oods_point));
    let total_sum = total_sum + constraint_coefficients[72] * value;

    let value =
        (column5 - oods_values[73]).field_div(&felt_nonzero!(point - pow175 * oods_point));
    let total_sum = total_sum + constraint_coefficients[73] * value;

    let value =
        (column5 - oods_values[74]).field_div(&felt_nonzero!(point - pow178 * oods_point));
    let total_sum = total_sum + constraint_coefficients[74] * value;

    let value =
        (column5 - oods_values[75]).field_div(&felt_nonzero!(point - pow177 * oods_point));
    let total_sum = total_sum + constraint_coefficients[75] * value;

    let value =
        (column5 - oods_values[76]).field_div(&felt_nonzero!(point - pow188 * oods_point));
    let total_sum = total_sum + constraint_coefficients[76] * value;

    let value =
        (column5 - oods_values[77]).field_div(&felt_nonzero!(point - pow184 * oods_point));
    let total_sum = total_sum + constraint_coefficients[77] * value;

    let value =
        (column5 - oods_values[78]).field_div(&felt_nonzero!(point - pow185 * oods_point));
    let total_sum = total_sum + constraint_coefficients[78] * value;

    let value =
        (column5 - oods_values[79]).field_div(&felt_nonzero!(point - pow187 * oods_point));
    let total_sum = total_sum + constraint_coefficients[79] * value;

    let value = (column5 - oods_values[80]).field_div(&felt_nonzero!(point - pow16 * oods_point));
    let total_sum = total_sum + constraint_coefficients[80] * value;

    let value = (column5 - oods_values[81]).field_div(&felt_nonzero!(point - pow23 * oods_point));
    let total_sum = total_sum + constraint_coefficients[81] * value;

    let value = (column5 - oods_values[82]).field_div(&felt_nonzero!(point - pow15 * oods_point));
    let total_sum = total_sum + constraint_coefficients[82] * value;

    let value = (column5 - oods_values[83]).field_div(&felt_nonzero!(point - pow22 * oods_point));
    let total_sum = total_sum + constraint_coefficients[83] * value;

    let value = (column5 - oods_values[84]).field_div(&felt_nonzero!(point - pow14 * oods_point));
    let total_sum = total_sum + constraint_coefficients[84] * value;

    let value = (column5 - oods_values[85]).field_div(&felt_nonzero!(point - pow21 * oods_point));
    let total_sum = total_sum + constraint_coefficients[85] * value;

    let value = (column5 - oods_values[86]).field_div(&felt_nonzero!(point - pow13 * oods_point));
    let total_sum = total_sum + constraint_coefficients[86] * value;

    let value = (column5 - oods_values[87]).field_div(&felt_nonzero!(point - pow20 * oods_point));
    let total_sum = total_sum + constraint_coefficients[87] * value;

    let value = (column5 - oods_values[88]).field_div(&felt_nonzero!(point - pow12 * oods_point));
    let total_sum = total_sum + constraint_coefficients[88] * value;

    let value = (column5 - oods_values[89]).field_div(&felt_nonzero!(point - pow19 * oods_point));
    let total_sum = total_sum + constraint_coefficients[89] * value;

    let value =
        (column5 - oods_values[90]).field_div(&felt_nonzero!(point - pow139 * oods_point));
    let total_sum = total_sum + constraint_coefficients[90] * value;

    let value =
        (column5 - oods_values[91]).field_div(&felt_nonzero!(point - pow140 * oods_point));
    let total_sum = total_sum + constraint_coefficients[91] * value;

    let value = (column5 - oods_values[92]).field_div(&felt_nonzero!(point - pow11 * oods_point));
    let total_sum = total_sum + constraint_coefficients[92] * value;

    let value = (column5 - oods_values[93]).field_div(&felt_nonzero!(point - pow18 * oods_point));
    let total_sum = total_sum + constraint_coefficients[93] * value;

    let value = (column5 - oods_values[94]).field_div(&felt_nonzero!(point - pow10 * oods_point));
    let total_sum = total_sum + constraint_coefficients[94] * value;

    let value =
        (column5 - oods_values[95]).field_div(&felt_nonzero!(point - pow161 * oods_point));
    let total_sum = total_sum + constraint_coefficients[95] * value;

    let value = (column5 - oods_values[96]).field_div(&felt_nonzero!(point - pow9 * oods_point));
    let total_sum = total_sum + constraint_coefficients[96] * value;

    let value =
        (column5 - oods_values[97]).field_div(&felt_nonzero!(point - pow173 * oods_point));
    let total_sum = total_sum + constraint_coefficients[97] * value;

    let value = (column6 - oods_values[98]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[98] * value;

    let value = (column6 - oods_values[99]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[99] * value;

    let value =
        (column6 - oods_values[100]).field_div(&felt_nonzero!(point - pow24 * oods_point));
    let total_sum = total_sum + constraint_coefficients[100] * value;

    let value =
        (column6 - oods_values[101]).field_div(&felt_nonzero!(point - pow25 * oods_point));
    let total_sum = total_sum + constraint_coefficients[101] * value;

    let value = (column7 - oods_values[102]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[102] * value;

    let value =
        (column7 - oods_values[103]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[103] * value;

    let value =
        (column7 - oods_values[104]).field_div(&felt_nonzero!(point - pow24 * oods_point));
    let total_sum = total_sum + constraint_coefficients[104] * value;

    let value =
        (column7 - oods_values[105]).field_div(&felt_nonzero!(point - pow25 * oods_point));
    let total_sum = total_sum + constraint_coefficients[105] * value;

    let value =
        (column7 - oods_values[106]).field_div(&felt_nonzero!(point - pow26 * oods_point));
    let total_sum = total_sum + constraint_coefficients[106] * value;

    let value =
        (column7 - oods_values[107]).field_div(&felt_nonzero!(point - pow27 * oods_point));
    let total_sum = total_sum + constraint_coefficients[107] * value;

    let value =
        (column7 - oods_values[108]).field_div(&felt_nonzero!(point - pow28 * oods_point));
    let total_sum = total_sum + constraint_coefficients[108] * value;

    let value =
        (column7 - oods_values[109]).field_div(&felt_nonzero!(point - pow30 * oods_point));
    let total_sum = total_sum + constraint_coefficients[109] * value;

    let value =
        (column7 - oods_values[110]).field_div(&felt_nonzero!(point - pow31 * oods_point));
    let total_sum = total_sum + constraint_coefficients[110] * value;

    let value =
        (column7 - oods_values[111]).field_div(&felt_nonzero!(point - pow32 * oods_point));
    let total_sum = total_sum + constraint_coefficients[111] * value;

    let value =
        (column7 - oods_values[112]).field_div(&felt_nonzero!(point - pow34 * oods_point));
    let total_sum = total_sum + constraint_coefficients[112] * value;

    let value =
        (column7 - oods_values[113]).field_div(&felt_nonzero!(point - pow35 * oods_point));
    let total_sum = total_sum + constraint_coefficients[113] * value;

    let value =
        (column7 - oods_values[114]).field_div(&felt_nonzero!(point - pow36 * oods_point));
    let total_sum = total_sum + constraint_coefficients[114] * value;

    let value =
        (column7 - oods_values[115]).field_div(&felt_nonzero!(point - pow38 * oods_point));
    let total_sum = total_sum + constraint_coefficients[115] * value;

    let value =
        (column7 - oods_values[116]).field_div(&felt_nonzero!(point - pow40 * oods_point));
    let total_sum = total_sum + constraint_coefficients[116] * value;

    let value =
        (column7 - oods_values[117]).field_div(&felt_nonzero!(point - pow41 * oods_point));
    let total_sum = total_sum + constraint_coefficients[117] * value;

    let value =
        (column7 - oods_values[118]).field_div(&felt_nonzero!(point - pow44 * oods_point));
    let total_sum = total_sum + constraint_coefficients[118] * value;

    let value =
        (column7 - oods_values[119]).field_div(&felt_nonzero!(point - pow47 * oods_point));
    let total_sum = total_sum + constraint_coefficients[119] * value;

    let value =
        (column7 - oods_values[120]).field_div(&felt_nonzero!(point - pow50 * oods_point));
    let total_sum = total_sum + constraint_coefficients[120] * value;

    let value =
        (column7 - oods_values[121]).field_div(&felt_nonzero!(point - pow57 * oods_point));
    let total_sum = total_sum + constraint_coefficients[121] * value;

    let value =
        (column7 - oods_values[122]).field_div(&felt_nonzero!(point - pow60 * oods_point));
    let total_sum = total_sum + constraint_coefficients[122] * value;

    let value =
        (column7 - oods_values[123]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[123] * value;

    let value =
        (column7 - oods_values[124]).field_div(&felt_nonzero!(point - pow77 * oods_point));
    let total_sum = total_sum + constraint_coefficients[124] * value;

    let value =
        (column7 - oods_values[125]).field_div(&felt_nonzero!(point - pow79 * oods_point));
    let total_sum = total_sum + constraint_coefficients[125] * value;

    let value =
        (column7 - oods_values[126]).field_div(&felt_nonzero!(point - pow83 * oods_point));
    let total_sum = total_sum + constraint_coefficients[126] * value;

    let value =
        (column7 - oods_values[127]).field_div(&felt_nonzero!(point - pow88 * oods_point));
    let total_sum = total_sum + constraint_coefficients[127] * value;

    let value =
        (column7 - oods_values[128]).field_div(&felt_nonzero!(point - pow90 * oods_point));
    let total_sum = total_sum + constraint_coefficients[128] * value;

    let value =
        (column7 - oods_values[129]).field_div(&felt_nonzero!(point - pow93 * oods_point));
    let total_sum = total_sum + constraint_coefficients[129] * value;

    let value =
        (column7 - oods_values[130]).field_div(&felt_nonzero!(point - pow96 * oods_point));
    let total_sum = total_sum + constraint_coefficients[130] * value;

    let value =
        (column7 - oods_values[131]).field_div(&felt_nonzero!(point - pow97 * oods_point));
    let total_sum = total_sum + constraint_coefficients[131] * value;

    let value =
        (column7 - oods_values[132]).field_div(&felt_nonzero!(point - pow99 * oods_point));
    let total_sum = total_sum + constraint_coefficients[132] * value;

    let value =
        (column7 - oods_values[133]).field_div(&felt_nonzero!(point - pow102 * oods_point));
    let total_sum = total_sum + constraint_coefficients[133] * value;

    let value =
        (column7 - oods_values[134]).field_div(&felt_nonzero!(point - pow103 * oods_point));
    let total_sum = total_sum + constraint_coefficients[134] * value;

    let value =
        (column7 - oods_values[135]).field_div(&felt_nonzero!(point - pow106 * oods_point));
    let total_sum = total_sum + constraint_coefficients[135] * value;

    let value =
        (column7 - oods_values[136]).field_div(&felt_nonzero!(point - pow112 * oods_point));
    let total_sum = total_sum + constraint_coefficients[136] * value;

    let value =
        (column7 - oods_values[137]).field_div(&felt_nonzero!(point - pow114 * oods_point));
    let total_sum = total_sum + constraint_coefficients[137] * value;

    let value =
        (column7 - oods_values[138]).field_div(&felt_nonzero!(point - pow117 * oods_point));
    let total_sum = total_sum + constraint_coefficients[138] * value;

    let value =
        (column7 - oods_values[139]).field_div(&felt_nonzero!(point - pow118 * oods_point));
    let total_sum = total_sum + constraint_coefficients[139] * value;

    let value =
        (column7 - oods_values[140]).field_div(&felt_nonzero!(point - pow120 * oods_point));
    let total_sum = total_sum + constraint_coefficients[140] * value;

    let value =
        (column7 - oods_values[141]).field_div(&felt_nonzero!(point - pow136 * oods_point));
    let total_sum = total_sum + constraint_coefficients[141] * value;

    let value =
        (column7 - oods_values[142]).field_div(&felt_nonzero!(point - pow143 * oods_point));
    let total_sum = total_sum + constraint_coefficients[142] * value;

    let value =
        (column7 - oods_values[143]).field_div(&felt_nonzero!(point - pow166 * oods_point));
    let total_sum = total_sum + constraint_coefficients[143] * value;

    let value =
        (column7 - oods_values[144]).field_div(&felt_nonzero!(point - pow168 * oods_point));
    let total_sum = total_sum + constraint_coefficients[144] * value;

    let value =
        (column7 - oods_values[145]).field_div(&felt_nonzero!(point - pow170 * oods_point));
    let total_sum = total_sum + constraint_coefficients[145] * value;

    let value =
        (column7 - oods_values[146]).field_div(&felt_nonzero!(point - pow174 * oods_point));
    let total_sum = total_sum + constraint_coefficients[146] * value;

    let value =
        (column7 - oods_values[147]).field_div(&felt_nonzero!(point - pow191 * oods_point));
    let total_sum = total_sum + constraint_coefficients[147] * value;

    let value =
        (column7 - oods_values[148]).field_div(&felt_nonzero!(point - pow176 * oods_point));
    let total_sum = total_sum + constraint_coefficients[148] * value;

    let value =
        (column7 - oods_values[149]).field_div(&felt_nonzero!(point - pow179 * oods_point));
    let total_sum = total_sum + constraint_coefficients[149] * value;

    let value =
        (column7 - oods_values[150]).field_div(&felt_nonzero!(point - pow180 * oods_point));
    let total_sum = total_sum + constraint_coefficients[150] * value;

    let value =
        (column7 - oods_values[151]).field_div(&felt_nonzero!(point - pow181 * oods_point));
    let total_sum = total_sum + constraint_coefficients[151] * value;

    let value =
        (column7 - oods_values[152]).field_div(&felt_nonzero!(point - pow182 * oods_point));
    let total_sum = total_sum + constraint_coefficients[152] * value;

    let value =
        (column7 - oods_values[153]).field_div(&felt_nonzero!(point - pow192 * oods_point));
    let total_sum = total_sum + constraint_coefficients[153] * value;

    let value =
        (column7 - oods_values[154]).field_div(&felt_nonzero!(point - pow183 * oods_point));
    let total_sum = total_sum + constraint_coefficients[154] * value;

    let value =
        (column7 - oods_values[155]).field_div(&felt_nonzero!(point - pow186 * oods_point));
    let total_sum = total_sum + constraint_coefficients[155] * value;

    let value =
        (column7 - oods_values[156]).field_div(&felt_nonzero!(point - pow189 * oods_point));
    let total_sum = total_sum + constraint_coefficients[156] * value;

    let value =
        (column7 - oods_values[157]).field_div(&felt_nonzero!(point - pow190 * oods_point));
    let total_sum = total_sum + constraint_coefficients[157] * value;

    let value = (column8 - oods_values[158]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[158] * value;

    let value =
        (column8 - oods_values[159]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[159] * value;

    let value =
        (column8 - oods_values[160]).field_div(&felt_nonzero!(point - pow24 * oods_point));
    let total_sum = total_sum + constraint_coefficients[160] * value;

    let value =
        (column8 - oods_values[161]).field_div(&felt_nonzero!(point - pow25 * oods_point));
    let total_sum = total_sum + constraint_coefficients[161] * value;

    let value =
        (column8 - oods_values[162]).field_div(&felt_nonzero!(point - pow26 * oods_point));
    let total_sum = total_sum + constraint_coefficients[162] * value;

    let value =
        (column8 - oods_values[163]).field_div(&felt_nonzero!(point - pow27 * oods_point));
    let total_sum = total_sum + constraint_coefficients[163] * value;

    let value =
        (column8 - oods_values[164]).field_div(&felt_nonzero!(point - pow28 * oods_point));
    let total_sum = total_sum + constraint_coefficients[164] * value;

    let value =
        (column8 - oods_values[165]).field_div(&felt_nonzero!(point - pow30 * oods_point));
    let total_sum = total_sum + constraint_coefficients[165] * value;

    let value =
        (column8 - oods_values[166]).field_div(&felt_nonzero!(point - pow31 * oods_point));
    let total_sum = total_sum + constraint_coefficients[166] * value;

    let value =
        (column8 - oods_values[167]).field_div(&felt_nonzero!(point - pow32 * oods_point));
    let total_sum = total_sum + constraint_coefficients[167] * value;

    let value =
        (column8 - oods_values[168]).field_div(&felt_nonzero!(point - pow33 * oods_point));
    let total_sum = total_sum + constraint_coefficients[168] * value;

    let value =
        (column8 - oods_values[169]).field_div(&felt_nonzero!(point - pow34 * oods_point));
    let total_sum = total_sum + constraint_coefficients[169] * value;

    let value =
        (column8 - oods_values[170]).field_div(&felt_nonzero!(point - pow35 * oods_point));
    let total_sum = total_sum + constraint_coefficients[170] * value;

    let value =
        (column8 - oods_values[171]).field_div(&felt_nonzero!(point - pow36 * oods_point));
    let total_sum = total_sum + constraint_coefficients[171] * value;

    let value =
        (column8 - oods_values[172]).field_div(&felt_nonzero!(point - pow37 * oods_point));
    let total_sum = total_sum + constraint_coefficients[172] * value;

    let value =
        (column8 - oods_values[173]).field_div(&felt_nonzero!(point - pow39 * oods_point));
    let total_sum = total_sum + constraint_coefficients[173] * value;

    let value =
        (column8 - oods_values[174]).field_div(&felt_nonzero!(point - pow40 * oods_point));
    let total_sum = total_sum + constraint_coefficients[174] * value;

    let value =
        (column8 - oods_values[175]).field_div(&felt_nonzero!(point - pow41 * oods_point));
    let total_sum = total_sum + constraint_coefficients[175] * value;

    let value =
        (column8 - oods_values[176]).field_div(&felt_nonzero!(point - pow42 * oods_point));
    let total_sum = total_sum + constraint_coefficients[176] * value;

    let value =
        (column8 - oods_values[177]).field_div(&felt_nonzero!(point - pow43 * oods_point));
    let total_sum = total_sum + constraint_coefficients[177] * value;

    let value =
        (column8 - oods_values[178]).field_div(&felt_nonzero!(point - pow45 * oods_point));
    let total_sum = total_sum + constraint_coefficients[178] * value;

    let value =
        (column8 - oods_values[179]).field_div(&felt_nonzero!(point - pow46 * oods_point));
    let total_sum = total_sum + constraint_coefficients[179] * value;

    let value =
        (column8 - oods_values[180]).field_div(&felt_nonzero!(point - pow47 * oods_point));
    let total_sum = total_sum + constraint_coefficients[180] * value;

    let value =
        (column8 - oods_values[181]).field_div(&felt_nonzero!(point - pow48 * oods_point));
    let total_sum = total_sum + constraint_coefficients[181] * value;

    let value =
        (column8 - oods_values[182]).field_div(&felt_nonzero!(point - pow49 * oods_point));
    let total_sum = total_sum + constraint_coefficients[182] * value;

    let value =
        (column8 - oods_values[183]).field_div(&felt_nonzero!(point - pow50 * oods_point));
    let total_sum = total_sum + constraint_coefficients[183] * value;

    let value =
        (column8 - oods_values[184]).field_div(&felt_nonzero!(point - pow51 * oods_point));
    let total_sum = total_sum + constraint_coefficients[184] * value;

    let value =
        (column8 - oods_values[185]).field_div(&felt_nonzero!(point - pow52 * oods_point));
    let total_sum = total_sum + constraint_coefficients[185] * value;

    let value =
        (column8 - oods_values[186]).field_div(&felt_nonzero!(point - pow53 * oods_point));
    let total_sum = total_sum + constraint_coefficients[186] * value;

    let value =
        (column8 - oods_values[187]).field_div(&felt_nonzero!(point - pow55 * oods_point));
    let total_sum = total_sum + constraint_coefficients[187] * value;

    let value =
        (column8 - oods_values[188]).field_div(&felt_nonzero!(point - pow56 * oods_point));
    let total_sum = total_sum + constraint_coefficients[188] * value;

    let value =
        (column8 - oods_values[189]).field_div(&felt_nonzero!(point - pow58 * oods_point));
    let total_sum = total_sum + constraint_coefficients[189] * value;

    let value =
        (column8 - oods_values[190]).field_div(&felt_nonzero!(point - pow59 * oods_point));
    let total_sum = total_sum + constraint_coefficients[190] * value;

    let value =
        (column8 - oods_values[191]).field_div(&felt_nonzero!(point - pow60 * oods_point));
    let total_sum = total_sum + constraint_coefficients[191] * value;

    let value =
        (column8 - oods_values[192]).field_div(&felt_nonzero!(point - pow61 * oods_point));
    let total_sum = total_sum + constraint_coefficients[192] * value;

    let value =
        (column8 - oods_values[193]).field_div(&felt_nonzero!(point - pow62 * oods_point));
    let total_sum = total_sum + constraint_coefficients[193] * value;

    let value =
        (column8 - oods_values[194]).field_div(&felt_nonzero!(point - pow63 * oods_point));
    let total_sum = total_sum + constraint_coefficients[194] * value;

    let value =
        (column8 - oods_values[195]).field_div(&felt_nonzero!(point - pow69 * oods_point));
    let total_sum = total_sum + constraint_coefficients[195] * value;

    let value =
        (column8 - oods_values[196]).field_div(&felt_nonzero!(point - pow70 * oods_point));
    let total_sum = total_sum + constraint_coefficients[196] * value;

    let value =
        (column8 - oods_values[197]).field_div(&felt_nonzero!(point - pow71 * oods_point));
    let total_sum = total_sum + constraint_coefficients[197] * value;

    let value =
        (column8 - oods_values[198]).field_div(&felt_nonzero!(point - pow72 * oods_point));
    let total_sum = total_sum + constraint_coefficients[198] * value;

    let value =
        (column8 - oods_values[199]).field_div(&felt_nonzero!(point - pow73 * oods_point));
    let total_sum = total_sum + constraint_coefficients[199] * value;

    let value =
        (column8 - oods_values[200]).field_div(&felt_nonzero!(point - pow75 * oods_point));
    let total_sum = total_sum + constraint_coefficients[200] * value;

    let value =
        (column8 - oods_values[201]).field_div(&felt_nonzero!(point - pow76 * oods_point));
    let total_sum = total_sum + constraint_coefficients[201] * value;

    let value =
        (column8 - oods_values[202]).field_div(&felt_nonzero!(point - pow78 * oods_point));
    let total_sum = total_sum + constraint_coefficients[202] * value;

    let value =
        (column8 - oods_values[203]).field_div(&felt_nonzero!(point - pow79 * oods_point));
    let total_sum = total_sum + constraint_coefficients[203] * value;

    let value =
        (column8 - oods_values[204]).field_div(&felt_nonzero!(point - pow80 * oods_point));
    let total_sum = total_sum + constraint_coefficients[204] * value;

    let value =
        (column8 - oods_values[205]).field_div(&felt_nonzero!(point - pow81 * oods_point));
    let total_sum = total_sum + constraint_coefficients[205] * value;

    let value =
        (column8 - oods_values[206]).field_div(&felt_nonzero!(point - pow82 * oods_point));
    let total_sum = total_sum + constraint_coefficients[206] * value;

    let value =
        (column8 - oods_values[207]).field_div(&felt_nonzero!(point - pow83 * oods_point));
    let total_sum = total_sum + constraint_coefficients[207] * value;

    let value =
        (column8 - oods_values[208]).field_div(&felt_nonzero!(point - pow84 * oods_point));
    let total_sum = total_sum + constraint_coefficients[208] * value;

    let value =
        (column8 - oods_values[209]).field_div(&felt_nonzero!(point - pow87 * oods_point));
    let total_sum = total_sum + constraint_coefficients[209] * value;

    let value =
        (column8 - oods_values[210]).field_div(&felt_nonzero!(point - pow89 * oods_point));
    let total_sum = total_sum + constraint_coefficients[210] * value;

    let value =
        (column8 - oods_values[211]).field_div(&felt_nonzero!(point - pow90 * oods_point));
    let total_sum = total_sum + constraint_coefficients[211] * value;

    let value =
        (column8 - oods_values[212]).field_div(&felt_nonzero!(point - pow91 * oods_point));
    let total_sum = total_sum + constraint_coefficients[212] * value;

    let value =
        (column8 - oods_values[213]).field_div(&felt_nonzero!(point - pow92 * oods_point));
    let total_sum = total_sum + constraint_coefficients[213] * value;

    let value =
        (column8 - oods_values[214]).field_div(&felt_nonzero!(point - pow98 * oods_point));
    let total_sum = total_sum + constraint_coefficients[214] * value;

    let value =
        (column8 - oods_values[215]).field_div(&felt_nonzero!(point - pow104 * oods_point));
    let total_sum = total_sum + constraint_coefficients[215] * value;

    let value =
        (column8 - oods_values[216]).field_div(&felt_nonzero!(point - pow107 * oods_point));
    let total_sum = total_sum + constraint_coefficients[216] * value;

    let value =
        (column8 - oods_values[217]).field_div(&felt_nonzero!(point - pow113 * oods_point));
    let total_sum = total_sum + constraint_coefficients[217] * value;

    let value =
        (column8 - oods_values[218]).field_div(&felt_nonzero!(point - pow115 * oods_point));
    let total_sum = total_sum + constraint_coefficients[218] * value;

    let value =
        (column8 - oods_values[219]).field_div(&felt_nonzero!(point - pow116 * oods_point));
    let total_sum = total_sum + constraint_coefficients[219] * value;

    let value =
        (column8 - oods_values[220]).field_div(&felt_nonzero!(point - pow119 * oods_point));
    let total_sum = total_sum + constraint_coefficients[220] * value;

    let value =
        (column8 - oods_values[221]).field_div(&felt_nonzero!(point - pow121 * oods_point));
    let total_sum = total_sum + constraint_coefficients[221] * value;

    let value =
        (column8 - oods_values[222]).field_div(&felt_nonzero!(point - pow133 * oods_point));
    let total_sum = total_sum + constraint_coefficients[222] * value;

    let value =
        (column8 - oods_values[223]).field_div(&felt_nonzero!(point - pow144 * oods_point));
    let total_sum = total_sum + constraint_coefficients[223] * value;

    let value =
        (column8 - oods_values[224]).field_div(&felt_nonzero!(point - pow147 * oods_point));
    let total_sum = total_sum + constraint_coefficients[224] * value;

    let value =
        (column8 - oods_values[225]).field_div(&felt_nonzero!(point - pow148 * oods_point));
    let total_sum = total_sum + constraint_coefficients[225] * value;

    let value =
        (column8 - oods_values[226]).field_div(&felt_nonzero!(point - pow149 * oods_point));
    let total_sum = total_sum + constraint_coefficients[226] * value;

    let value =
        (column8 - oods_values[227]).field_div(&felt_nonzero!(point - pow150 * oods_point));
    let total_sum = total_sum + constraint_coefficients[227] * value;

    let value =
        (column8 - oods_values[228]).field_div(&felt_nonzero!(point - pow153 * oods_point));
    let total_sum = total_sum + constraint_coefficients[228] * value;

    let value =
        (column8 - oods_values[229]).field_div(&felt_nonzero!(point - pow154 * oods_point));
    let total_sum = total_sum + constraint_coefficients[229] * value;

    let value =
        (column8 - oods_values[230]).field_div(&felt_nonzero!(point - pow155 * oods_point));
    let total_sum = total_sum + constraint_coefficients[230] * value;

    let value =
        (column8 - oods_values[231]).field_div(&felt_nonzero!(point - pow156 * oods_point));
    let total_sum = total_sum + constraint_coefficients[231] * value;

    let value =
        (column8 - oods_values[232]).field_div(&felt_nonzero!(point - pow162 * oods_point));
    let total_sum = total_sum + constraint_coefficients[232] * value;

    let value =
        (column8 - oods_values[233]).field_div(&felt_nonzero!(point - pow164 * oods_point));
    let total_sum = total_sum + constraint_coefficients[233] * value;

    let value =
        (column8 - oods_values[234]).field_div(&felt_nonzero!(point - pow165 * oods_point));
    let total_sum = total_sum + constraint_coefficients[234] * value;

    let value =
        (column8 - oods_values[235]).field_div(&felt_nonzero!(point - pow167 * oods_point));
    let total_sum = total_sum + constraint_coefficients[235] * value;

    let value =
        (column8 - oods_values[236]).field_div(&felt_nonzero!(point - pow169 * oods_point));
    let total_sum = total_sum + constraint_coefficients[236] * value;

    let value =
        (column8 - oods_values[237]).field_div(&felt_nonzero!(point - pow171 * oods_point));
    let total_sum = total_sum + constraint_coefficients[237] * value;

    let value = (column8 - oods_values[238]).field_div(&felt_nonzero!(point - pow8 * oods_point));
    let total_sum = total_sum + constraint_coefficients[238] * value;

    let value = (column8 - oods_values[239]).field_div(&felt_nonzero!(point - pow7 * oods_point));
    let total_sum = total_sum + constraint_coefficients[239] * value;

    let value =
        (column8 - oods_values[240]).field_div(&felt_nonzero!(point - pow138 * oods_point));
    let total_sum = total_sum + constraint_coefficients[240] * value;

    let value =
        (column8 - oods_values[241]).field_div(&felt_nonzero!(point - pow137 * oods_point));
    let total_sum = total_sum + constraint_coefficients[241] * value;

    let value = (column8 - oods_values[242]).field_div(&felt_nonzero!(point - pow6 * oods_point));
    let total_sum = total_sum + constraint_coefficients[242] * value;

    let value = (column8 - oods_values[243]).field_div(&felt_nonzero!(point - pow5 * oods_point));
    let total_sum = total_sum + constraint_coefficients[243] * value;

    let value = (column8 - oods_values[244]).field_div(&felt_nonzero!(point - pow4 * oods_point));
    let total_sum = total_sum + constraint_coefficients[244] * value;

    let value =
        (column8 - oods_values[245]).field_div(&felt_nonzero!(point - pow29 * oods_point));
    let total_sum = total_sum + constraint_coefficients[245] * value;

    let value =
        (column8 - oods_values[246]).field_div(&felt_nonzero!(point - pow124 * oods_point));
    let total_sum = total_sum + constraint_coefficients[246] * value;

    let value =
        (column8 - oods_values[247]).field_div(&felt_nonzero!(point - pow125 * oods_point));
    let total_sum = total_sum + constraint_coefficients[247] * value;

    let value =
        (column8 - oods_values[248]).field_div(&felt_nonzero!(point - pow126 * oods_point));
    let total_sum = total_sum + constraint_coefficients[248] * value;

    let value =
        (column8 - oods_values[249]).field_div(&felt_nonzero!(point - pow127 * oods_point));
    let total_sum = total_sum + constraint_coefficients[249] * value;

    let value =
        (column8 - oods_values[250]).field_div(&felt_nonzero!(point - pow128 * oods_point));
    let total_sum = total_sum + constraint_coefficients[250] * value;

    let value =
        (column8 - oods_values[251]).field_div(&felt_nonzero!(point - pow129 * oods_point));
    let total_sum = total_sum + constraint_coefficients[251] * value;

    let value =
        (column8 - oods_values[252]).field_div(&felt_nonzero!(point - pow130 * oods_point));
    let total_sum = total_sum + constraint_coefficients[252] * value;

    let value =
        (column8 - oods_values[253]).field_div(&felt_nonzero!(point - pow131 * oods_point));
    let total_sum = total_sum + constraint_coefficients[253] * value;

    let value =
        (column8 - oods_values[254]).field_div(&felt_nonzero!(point - pow132 * oods_point));
    let total_sum = total_sum + constraint_coefficients[254] * value;

    let value = (column8 - oods_values[255]).field_div(&felt_nonzero!(point - pow3 * oods_point));
    let total_sum = total_sum + constraint_coefficients[255] * value;

    let value = (column8 - oods_values[256]).field_div(&felt_nonzero!(point - pow2 * oods_point));
    let total_sum = total_sum + constraint_coefficients[256] * value;

    let value = (column8 - oods_values[257]).field_div(&felt_nonzero!(point - pow1 * oods_point));
    let total_sum = total_sum + constraint_coefficients[257] * value;

    let value =
        (column8 - oods_values[258]).field_div(&felt_nonzero!(point - pow64 * oods_point));
    let total_sum = total_sum + constraint_coefficients[258] * value;

    let value =
        (column8 - oods_values[259]).field_div(&felt_nonzero!(point - pow65 * oods_point));
    let total_sum = total_sum + constraint_coefficients[259] * value;

    let value =
        (column8 - oods_values[260]).field_div(&felt_nonzero!(point - pow66 * oods_point));
    let total_sum = total_sum + constraint_coefficients[260] * value;

    let value =
        (column8 - oods_values[261]).field_div(&felt_nonzero!(point - pow67 * oods_point));
    let total_sum = total_sum + constraint_coefficients[261] * value;

    let value =
        (column8 - oods_values[262]).field_div(&felt_nonzero!(point - pow68 * oods_point));
    let total_sum = total_sum + constraint_coefficients[262] * value;

    let value = (column9 - oods_values[263]).field_div(&felt_nonzero!(point - pow0 * oods_point));
    let total_sum = total_sum + constraint_coefficients[263] * value;

    let value =
        (column9 - oods_values[264]).field_div(&felt_nonzero!(point - pow17 * oods_point));
    let total_sum = total_sum + constraint_coefficients[264] * value;

    let value =
        (column9 - oods_values[265]).field_div(&felt_nonzero!(point - pow24 * oods_point));
    let total_sum = total_sum + constraint_coefficients[265] * value;

    let value =
        (column9 - oods_values[266]).field_div(&felt_nonzero!(point - pow25 * oods_point));
    let total_sum = total_sum + constraint_coefficients[266] * value;

    let value =
        (column9 - oods_values[267]).field_div(&felt_nonzero!(point - pow27 * oods_point));
    let total_sum = total_sum + constraint_coefficients[267] * value;

    let value =
        (column9 - oods_values[268]).field_div(&felt_nonzero!(point - pow30 * oods_point));
    let total_sum = total_sum + constraint_coefficients[268] * value;

    let value =
        (column9 - oods_values[269]).field_div(&felt_nonzero!(point - pow34 * oods_point));
    let total_sum = total_sum + constraint_coefficients[269] * value;

    let value =
        (column9 - oods_values[270]).field_div(&felt_nonzero!(point - pow38 * oods_point));
    let total_sum = total_sum + constraint_coefficients[270] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow_felt(&(Layout::CONSTRAINT_DEGREE.into()));

    let value = (column_values[Layout::NUM_COLUMNS_FIRST + Layout::NUM_COLUMNS_SECOND]
        - oods_values[271])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));
    let total_sum = total_sum + constraint_coefficients[271] * value;

    let value = (column_values[Layout::NUM_COLUMNS_FIRST + Layout::NUM_COLUMNS_SECOND + 1]
        - oods_values[272])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));
    

    total_sum + constraint_coefficients[272] * value
}
