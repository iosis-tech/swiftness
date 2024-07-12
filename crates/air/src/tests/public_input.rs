use crate::fixtures::public_input::get;
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
