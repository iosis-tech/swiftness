use crate::transcript::Transcript;
use starknet_crypto::Felt;

#[test]
fn test_read_uint64_from_prover_0() {
    let mut channel = Transcript::new_with_counter(
        Felt::from_hex("0x69eb7eb40004d1d7375b1ff9ccff8f7aed629e669b4fc3e11db4e748fdfbb2f")
            .unwrap(),
        Felt::from_hex("0x0").unwrap(),
    );
    channel.read_uint64_from_prover(0x1e7e0);
    assert!(
        *channel.digest()
            == Felt::from_hex("0xc5952bab5731090a953716ac821ee7374cc6c4bad31d21b7134f62d6e00593")
                .unwrap()
    );
    assert!(*channel.counter() == Felt::from_hex("0x0").unwrap());
}
