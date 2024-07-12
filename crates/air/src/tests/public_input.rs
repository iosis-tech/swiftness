use crate::{
    public_memory::PublicInput,
    types::{AddrValue, Page, SegmentInfo},
};
use starknet_crypto::Felt;

#[test]
fn test_public_input_hash() {
    let public_input = get();
    assert_eq!(
        public_input.get_public_input_hash(),
        Felt::from_hex_unchecked(
            "0xaf91f2c71f4a594b1575d258ce82464475c82d8fb244142d0db450491c1b52"
        )
    );
}

pub fn get() -> PublicInput {
    PublicInput {
        log_n_steps: Felt::from_hex_unchecked("0xe"),
        range_check_min: Felt::from_hex_unchecked("0x7ffa"),
        range_check_max: Felt::from_hex_unchecked("0x8001"),
        layout: Felt::from_hex_unchecked("0x726563757273697665"),
        dynamic_params: vec![],
        segments: vec![
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1"),
                stop_ptr: Felt::from_hex_unchecked("0x5"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x25"),
                stop_ptr: Felt::from_hex_unchecked("0x68"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x68"),
                stop_ptr: Felt::from_hex_unchecked("0x6a"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x6a"),
                stop_ptr: Felt::from_hex_unchecked("0x6a"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1ea"),
                stop_ptr: Felt::from_hex_unchecked("0x1ea"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x9ea"),
                stop_ptr: Felt::from_hex_unchecked("0x9ea"),
            },
        ],
        padding_addr: Felt::from_hex_unchecked("0x1"),
        padding_value: Felt::from_hex_unchecked("0x40780017fff7fff"),
        main_page: Page(vec![
            AddrValue {
                address: Felt::from_hex("0x1").unwrap(),
                value: Felt::from_hex("0x40780017fff7fff").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x2").unwrap(),
                value: Felt::from_hex("0x4").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x3").unwrap(),
                value: Felt::from_hex("0x1104800180018000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x4").unwrap(),
                value: Felt::from_hex("0x4").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x5").unwrap(),
                value: Felt::from_hex("0x10780017fff7fff").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x6").unwrap(),
                value: Felt::from_hex("0x0").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x7").unwrap(),
                value: Felt::from_hex("0x40780017fff7fff").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x8").unwrap(),
                value: Felt::from_hex("0x1").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x9").unwrap(),
                value: Felt::from_hex("0x400380007ffa8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0xa").unwrap(),
                value: Felt::from_hex("0x480680017fff8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0xb").unwrap(),
                value: Felt::from_hex("0x1").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0xc").unwrap(),
                value: Felt::from_hex("0x480680017fff8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0xd").unwrap(),
                value: Felt::from_hex("0x1").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0xe").unwrap(),
                value: Felt::from_hex("0x480a80007fff8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0xf").unwrap(),
                value: Felt::from_hex("0x1104800180018000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x10").unwrap(),
                value: Felt::from_hex("0x9").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x11").unwrap(),
                value: Felt::from_hex("0x400280017ffa7fff").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x12").unwrap(),
                value: Felt::from_hex("0x482680017ffa8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x13").unwrap(),
                value: Felt::from_hex("0x2").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x14").unwrap(),
                value: Felt::from_hex("0x480a7ffb7fff8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x15").unwrap(),
                value: Felt::from_hex("0x480a7ffc7fff8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x16").unwrap(),
                value: Felt::from_hex("0x480a7ffd7fff8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x17").unwrap(),
                value: Felt::from_hex("0x208b7fff7fff7ffe").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x18").unwrap(),
                value: Felt::from_hex("0x20780017fff7ffd").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x19").unwrap(),
                value: Felt::from_hex("0x4").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x1a").unwrap(),
                value: Felt::from_hex("0x480a7ffc7fff8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x1b").unwrap(),
                value: Felt::from_hex("0x208b7fff7fff7ffe").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x1c").unwrap(),
                value: Felt::from_hex("0x480a7ffc7fff8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x1d").unwrap(),
                value: Felt::from_hex("0x482a7ffc7ffb8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x1e").unwrap(),
                value: Felt::from_hex("0x482680017ffd8000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x1f").unwrap(),
                value: Felt::from_hex(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                )
                .unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x20").unwrap(),
                value: Felt::from_hex("0x1104800180018000").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x21").unwrap(),
                value: Felt::from_hex(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffff9",
                )
                .unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x22").unwrap(),
                value: Felt::from_hex("0x208b7fff7fff7ffe").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x23").unwrap(),
                value: Felt::from_hex("0x25").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x24").unwrap(),
                value: Felt::from_hex("0x0").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x25").unwrap(),
                value: Felt::from_hex("0x68").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x26").unwrap(),
                value: Felt::from_hex("0x6a").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x27").unwrap(),
                value: Felt::from_hex("0x1ea").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x28").unwrap(),
                value: Felt::from_hex("0x9ea").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x64").unwrap(),
                value: Felt::from_hex("0x6a").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x65").unwrap(),
                value: Felt::from_hex("0x6a").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x66").unwrap(),
                value: Felt::from_hex("0x1ea").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x67").unwrap(),
                value: Felt::from_hex("0x9ea").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x68").unwrap(),
                value: Felt::from_hex("0xa").unwrap(),
            },
            AddrValue {
                address: Felt::from_hex("0x69").unwrap(),
                value: Felt::from_hex("0x90").unwrap(),
            },
        ]),
        continuous_page_headers: vec![],
    }
}
