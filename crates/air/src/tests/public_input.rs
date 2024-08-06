use crate::fixtures::public_input::get;
use starknet_crypto::Felt;

#[test]
fn test_public_input_hash() {
    let public_input = get();
    assert_eq!(
        public_input.get_hash(Felt::ZERO),
        Felt::from_hex_unchecked(
            "0x78995ef92826448325c224646b2904b3ede3d36fdf35c3d12839c2bbff6d2e7"
        )
    );
}
