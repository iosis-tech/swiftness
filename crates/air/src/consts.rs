use starknet_crypto::Felt;

#[macro_export]
macro_rules! felt_hex {
    ($expr:expr) => {
        Felt::from_hex_unchecked($expr)
    };
}

#[macro_export]
macro_rules! felt_nonzero {
    ($expr:expr) => {
        NonZeroFelt::from_felt_unchecked($expr)
    };
}

pub const FELT_0: Felt = felt_hex!("0x0");
pub const FELT_1: Felt = felt_hex!("0x1");
pub const FELT_2: Felt = felt_hex!("0x2");
pub const FELT_3: Felt = felt_hex!("0x3");
pub const FELT_4: Felt = felt_hex!("0x4");
pub const FELT_5: Felt = felt_hex!("0x5");
pub const FELT_6: Felt = felt_hex!("0x6");
pub const FELT_7: Felt = felt_hex!("0x7");
pub const FELT_8: Felt = felt_hex!("0x8");
pub const FELT_16: Felt = felt_hex!("0x10");
pub const FELT_66: Felt = felt_hex!("0x42");
pub const FELT_68: Felt = felt_hex!("0x44");
pub const FELT_16384: Felt = felt_hex!("0x4000");
pub const FELT_10: Felt = felt_hex!("0xa");
pub const FELT_11: Felt = felt_hex!("0xb");
pub const FELT_13: Felt = felt_hex!("0xd");
pub const FELT_15: Felt = felt_hex!("0xf");
pub const FELT_19: Felt = felt_hex!("0x13");
pub const FELT_21: Felt = felt_hex!("0x15");
pub const FELT_23: Felt = felt_hex!("0x17");
pub const FELT_25: Felt = felt_hex!("0x19");
pub const FELT_27: Felt = felt_hex!("0x1b");
pub const FELT_29: Felt = felt_hex!("0x1d");
pub const FELT_31: Felt = felt_hex!("0x1f");
pub const FELT_32: Felt = felt_hex!("0x20");
pub const FELT_61: Felt = felt_hex!("0x3d");
pub const FELT_63: Felt = felt_hex!("0x3f");
pub const FELT_64: Felt = felt_hex!("0x40");
pub const FELT_128: Felt = felt_hex!("0x80");
pub const FELT_251: Felt = felt_hex!("0xfb");
pub const FELT_255: Felt = felt_hex!("0xff");
pub const FELT_256: Felt = felt_hex!("0x100");
pub const FELT_512: Felt = felt_hex!("0x200");
pub const FELT_522: Felt = felt_hex!("0x20a");
pub const FELT_768: Felt = felt_hex!("0x300");
pub const FELT_1004: Felt = felt_hex!("0x3ec");
pub const FELT_2048: Felt = felt_hex!("0x800");
pub const FELT_4080: Felt = felt_hex!("0xff0");
pub const FELT_4096: Felt = felt_hex!("0x1000");
pub const FELT_8161: Felt = felt_hex!("0x1fe1");
pub const FELT_8192: Felt = felt_hex!("0x2000");
pub const FELT_4089: Felt = felt_hex!("0xff9");
pub const FELT_2011: Felt = felt_hex!("0x7db");
pub const FELT_1539: Felt = felt_hex!("0x603");
pub const FELT_8160: Felt = felt_hex!("0x1fe0");
pub const FELT_4081: Felt = felt_hex!("0xff1");
pub const FELT_1024: Felt = felt_hex!("0x400");
pub const FELT_32715: Felt = felt_hex!("0x7fcb");
pub const FELT_32667: Felt = felt_hex!("0x7f9b");
pub const FELT_32647: Felt = felt_hex!("0x7f87");
pub const FELT_16325: Felt = felt_hex!("0x3fc5");
pub const FELT_16149: Felt = felt_hex!("0x3f15");
pub const FELT_16085: Felt = felt_hex!("0x3ed5");
pub const FELT_12373: Felt = felt_hex!("0x3055");
pub const FELT_12309: Felt = felt_hex!("0x3015");
pub const FELT_24966: Felt = felt_hex!("0x6186");
pub const FELT_16774: Felt = felt_hex!("0x4186");
pub const FELT_14726: Felt = felt_hex!("0x3986");
pub const FELT_10630: Felt = felt_hex!("0x2986");
pub const FELT_8582: Felt = felt_hex!("0x2186");
pub const FELT_6534: Felt = felt_hex!("0x1986");
pub const FELT_4486: Felt = felt_hex!("0x1186");
pub const FELT_2438: Felt = felt_hex!("0x986");
pub const FELT_446471: Felt = felt_hex!("0x6d007");
pub const FELT_397827: Felt = felt_hex!("0x61203");
pub const FELT_384835: Felt = felt_hex!("0x5df43");
pub const FELT_321543: Felt = felt_hex!("0x4e807");
pub const FELT_132611: Felt = felt_hex!("0x20603");
pub const FELT_66307: Felt = felt_hex!("0x10303");
pub const FELT_3462: Felt = felt_hex!("0xd86");
pub const FELT_515841: Felt = felt_hex!("0x7df01");
pub const FELT_513025: Felt = felt_hex!("0x7d401");
pub const FELT_506306: Felt = felt_hex!("0x7b9c2");
pub const FELT_502017: Felt = felt_hex!("0x7a901");
pub const FELT_476932: Felt = felt_hex!("0x74704");
pub const FELT_455937: Felt = felt_hex!("0x6f501");
pub const FELT_450753: Felt = felt_hex!("0x6e0c1");
pub const FELT_448772: Felt = felt_hex!("0x6d904");
pub const FELT_445188: Felt = felt_hex!("0x6cb04");
pub const FELT_383426: Felt = felt_hex!("0x5d9c2");
pub const FELT_381956: Felt = felt_hex!("0x5d404");
pub const FELT_376388: Felt = felt_hex!("0x5be44");
pub const FELT_370689: Felt = felt_hex!("0x5a801");
pub const FELT_341761: Felt = felt_hex!("0x53701");
pub const FELT_337601: Felt = felt_hex!("0x526c1");
pub const FELT_325894: Felt = felt_hex!("0x4f906");
pub const FELT_325121: Felt = felt_hex!("0x4f601");
pub const FELT_320449: Felt = felt_hex!("0x4e3c1");
pub const FELT_304132: Felt = felt_hex!("0x4a404");
pub const FELT_228161: Felt = felt_hex!("0x37b41");
pub const FELT_225025: Felt = felt_hex!("0x36f01");
pub const FELT_212740: Felt = felt_hex!("0x33f04");
pub const FELT_211396: Felt = felt_hex!("0x339c4");
pub const FELT_208388: Felt = felt_hex!("0x32e04");
pub const FELT_207873: Felt = felt_hex!("0x32c01");
pub const FELT_195010: Felt = felt_hex!("0x2f9c2");
pub const FELT_192260: Felt = felt_hex!("0x2ef04");
pub const FELT_178433: Felt = felt_hex!("0x2b901");
pub const FELT_175108: Felt = felt_hex!("0x2ac04");
pub const FELT_172801: Felt = felt_hex!("0x2a301");
pub const FELT_162052: Felt = felt_hex!("0x27904");
pub const FELT_159748: Felt = felt_hex!("0x27004");
pub const FELT_155398: Felt = felt_hex!("0x25f06");
pub const FELT_151041: Felt = felt_hex!("0x24e01");
pub const FELT_130433: Felt = felt_hex!("0x1fd81");
pub const FELT_127489: Felt = felt_hex!("0x1f201");
pub const FELT_115713: Felt = felt_hex!("0x1c401");
pub const FELT_89281: Felt = felt_hex!("0x15cc1");
pub const FELT_86273: Felt = felt_hex!("0x15101");
pub const FELT_75780: Felt = felt_hex!("0x12804");
pub const FELT_55937: Felt = felt_hex!("0xda81");
pub const FELT_51969: Felt = felt_hex!("0xcb01");
pub const FELT_31169: Felt = felt_hex!("0x79c1");
pub const FELT_26369: Felt = felt_hex!("0x6701");
pub const FELT_524288: Felt = felt_hex!("0x80000");
pub const FELT_36893488147419103232: Felt = felt_hex!("0x20000000000000000");
pub const FELT_73786976294838206464: Felt = felt_hex!("0x40000000000000000");
pub const FELT_147573952589676412928: Felt = felt_hex!("0x80000000000000000");
pub const FELT_340282366920938463463374607431768211456: Felt =
    felt_hex!("0x100000000000000000000000000000000");
pub const FELT_680564733841876926926749214863536422912: Felt =
    felt_hex!("0x200000000000000000000000000000000");
pub const FELT_1361129467683753853853498429727072845824: Felt =
    felt_hex!("0x400000000000000000000000000000000");
pub const FELT_2722258935367507707706996859454145691648: Felt =
    felt_hex!("0x800000000000000000000000000000000");
pub const FELT_6277101735386680763835789423207666416102355444464034512896: Felt =
    felt_hex!("0x1000000000000000000000000000000000000000000000000");
pub const FELT_12554203470773361527671578846415332832204710888928069025792: Felt =
    felt_hex!("0x2000000000000000000000000000000000000000000000000");
pub const FELT_25108406941546723055343157692830665664409421777856138051584: Felt =
    felt_hex!("0x4000000000000000000000000000000000000000000000000");
pub const FELT_50216813883093446110686315385661331328818843555712276103168: Felt =
    felt_hex!("0x8000000000000000000000000000000000000000000000000");
pub const FELT_1606938044258990275541962092341162602522202993782792835301376: Felt =
    felt_hex!("0x100000000000000000000000000000000000000000000000000");
pub const FELT_1229782938247303441: Felt = felt_hex!("0x1111111111111111");
pub const FELT_65536: Felt = felt_hex!("0x10000");
pub const FELT_4294967296: Felt = felt_hex!("0x100000000");
pub const FELT_281474976710656: Felt = felt_hex!("0x1000000000000");
pub const FELT_18446744073709551616: Felt = felt_hex!("0x10000000000000000");
pub const FELT_1208925819614629174706176: Felt = felt_hex!("0x100000000000000000000");
pub const FELT_316912650057057350374175801344: Felt = felt_hex!("0x4000000000000000000000000");
pub const FELT_79228162514264337593543950336: Felt = felt_hex!("0x1000000000000000000000000");
pub const FELT_3138550867693340381917894711603833208051177722232017256448: Felt =
    felt_hex!("0x800000000000000000000000000000000000000000000000");
pub const FELT_2950795762459345168613727575620414179244544320470208355568817838579231751791: Felt =
    felt_hex!("0x6861759EA556A2339DD92F9562A30B9E58E2AD98109AE4780B7FD8EAC77FE6F");
pub const FELT_1587446564224215276866294500450702039420286416111469274423465069420553242820: Felt =
    felt_hex!("0x3827681995D5AF9FFC8397A3D00425A3DA43F76ABF28A64E4AB1A22F27508C4");
pub const FELT_1645965921169490687904413452218868659025437693527479459426157555728339600137: Felt =
    felt_hex!("0x3A3956D2FAD44D0E7F760A2277DC7CB2CAC75DC279B2D687A0DBE17704A8309");
pub const FELT_2121140748740143694053732746913428481442990369183417228688865837805149503386: Felt =
    felt_hex!("0x4B085EB1DF4258C3453CC97445954BF3433B6AB9DD5A99592864C00F54A3F9A");
pub const FELT_3618502788666131213697322783095070105623107215331596699973092056135872020477: Felt =
    felt_hex!("0x800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD");
pub const FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479: Felt =
    felt_hex!("0x800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
pub const FELT_2006642341318481906727563724340978325665491359415674592697055778067937914672: Felt =
    felt_hex!("0x46FB825257FEC76C50FE043684D4E6D2D2F2FDFE9B7C8D7128CA7ACC0F66F30");
pub const FELT_1246177936547655338400308396717835700699368047388302793172818304164989556526: Felt =
    felt_hex!("0x2C14FCCABC26929170CC7AC9989C823608B9008BEF3B8E16B6089A5D33CD72E");
pub const FELT_427751140904099001132521606468025610873158555767197326325930641757709538586: Felt =
    felt_hex!("0xF2193BA0C7EA33CE6222D9446C1E166202AE5461005292F4A2BCB93420151A");
pub const FELT_18014398509481984: Felt = felt_hex!("0x40000000000000");
pub const FELT_560279373700919169769089400651532183647886248799764942664266404650165812023: Felt =
    felt_hex!("0x13D1B5CFD87693224F0AC561AB2C15CA53365D768311AF59CEFAF701BC53B37");
pub const FELT_1401754474293352309994371631695783042590401941592571735921592823982231996415: Felt =
    felt_hex!("0x3195D6B2D930E71CEDE286D5B8B41D49296DDF222BCD3BF3717A12A9A6947FF");
pub const FELT_49: Felt = felt_hex!("0x31");
pub const FELT_32768: Felt = felt_hex!("0x8000");
