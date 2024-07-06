use crate::transcript::Transcript;
use starknet_crypto::Felt;

#[test]
fn test_read_felt_from_prover_0() {
    let mut channel = Transcript::new_with_counter(
        Felt::from_hex("0x6844d424314222ee92d547c4acf2e8bef666793a9ac7a87012e6ba2b4c79857")
            .unwrap(),
        Felt::from_hex("0x0").unwrap(),
    );
    channel.read_felt_from_prover(
        &Felt::from_hex("0x38eed0024fd52d22f0f2faf86d2a3d79cadad24762ee24ee457470cd6cd79f0")
            .unwrap(),
    );
    assert!(
        *channel.digest()
            == Felt::from_hex("0x55c2e30068db90407013a4cfcedee7895b328c6ba64b8bd5e4c71e470af5fde")
                .unwrap()
    );
    assert!(*channel.counter() == Felt::from_hex("0x0").unwrap());
}

#[test]
fn test_read_felt_from_prover_1() {
    let mut channel = Transcript::new_with_counter(
        Felt::from_hex("0x55c2e30068db90407013a4cfcedee7895b328c6ba64b8bd5e4c71e470af5fde")
            .unwrap(),
        Felt::from_hex("0x3").unwrap(),
    );
    channel.read_felt_from_prover(
        &Felt::from_hex("0xbf526d19978e73141431cd6f0e3131afb7ec700a07f41245aa510c5ea3d2f8")
            .unwrap(),
    );
    assert!(
        *channel.digest()
            == Felt::from_hex("0x54d0627583471735a2948dbe3cf00d8104b8d99a3b3be8a8410daef4aa556f9")
                .unwrap(),
    );
    assert!(*channel.counter() == Felt::from_hex("0x0").unwrap());
}
