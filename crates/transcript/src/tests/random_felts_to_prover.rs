use starknet_crypto::Felt;

use crate::transcript::Transcript;

#[test]
fn test_random_felts_to_prover() {
    let mut channel = Transcript::new_with_counter(
        Felt::from_hex("0x55c2e30068db90407013a4cfcedee7895b328c6ba64b8bd5e4c71e470af5fde")
            .unwrap(),
        Felt::from_hex("0x0").unwrap(),
    );

    assert!(
        channel.random_felts_to_prover(Felt::from_hex("0x3").unwrap())
            == vec![
                Felt::from_hex("0x120149e03e4939d3c2a4ca2fcaa6e9cfff0c64fbe115f54c439d76ff09c3dc7")
                    .unwrap(),
                Felt::from_hex("0x3639344cc4b04d4c710b812e109e21f43f87c68d8648749cb25d30503037e4d")
                    .unwrap(),
                Felt::from_hex("0xeca2849fb4c4c8e734eafe6d9cb7256c0f1bb43a5c4f2d27090cd8df21a699")
                    .unwrap(),
            ]
    );
    assert!(
        *channel.digest()
            == Felt::from_hex("0x55c2e30068db90407013a4cfcedee7895b328c6ba64b8bd5e4c71e470af5fde")
                .unwrap(),
    );
    assert!(*channel.counter() == Felt::from_hex("0x3").unwrap());
}
