use starknet_crypto::Felt;

pub fn get() -> Vec<Felt> {
    vec![
        Felt::from_hex("0x3982a").unwrap(),
        Felt::from_hex("0x52d42").unwrap(),
        Felt::from_hex("0x585a8").unwrap(),
        Felt::from_hex("0x7c3cc").unwrap(),
        Felt::from_hex("0x8af7f").unwrap(),
        Felt::from_hex("0x8e6f3").unwrap(),
        Felt::from_hex("0x97846").unwrap(),
        Felt::from_hex("0x9e330").unwrap(),
        Felt::from_hex("0xa9b57").unwrap(),
        Felt::from_hex("0xfa009").unwrap(),
    ]
}