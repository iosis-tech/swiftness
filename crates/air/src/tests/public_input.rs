use crate::fixtures::public_input::get;
use starknet_crypto::Felt;

#[test]
fn test_public_input_hash() {
    let public_input = get();
    let expected = {
        #[cfg(feature = "stone5")]
        {
            Felt::from_hex_unchecked(
                "0xaf91f2c71f4a594b1575d258ce82464475c82d8fb244142d0db450491c1b52",
            )
        }
        #[cfg(feature = "stone6")]
        {
            Felt::from_hex_unchecked(
                "0x78995ef92826448325c224646b2904b3ede3d36fdf35c3d12839c2bbff6d2e7",
            )
        }
    };
    assert_eq!(public_input.get_hash(Felt::ZERO), expected);
}
