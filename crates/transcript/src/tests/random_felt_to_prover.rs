use starknet_crypto::Felt;

use crate::transcript::Transcript;

#[test]
fn test_random_felts_to_prover_0() {
    let mut channel = Transcript::new_with_counter(
        Felt::from_hex("0x55c2e30068db90407013a4cfcedee7895b328c6ba64b8bd5e4c71e470af5fde")
            .unwrap(),
        Felt::from_hex("0x0").unwrap(),
    );

    assert!(
        channel.random_felt_to_prover()
            == Felt::from_hex("0x120149e03e4939d3c2a4ca2fcaa6e9cfff0c64fbe115f54c439d76ff09c3dc7")
                .unwrap(),
    );
    assert!(
        *channel.digest()
            == Felt::from_hex("0x55c2e30068db90407013a4cfcedee7895b328c6ba64b8bd5e4c71e470af5fde")
                .unwrap(),
    );

    assert!(*channel.counter() == Felt::from_hex("0x1").unwrap());
}

#[test]
fn test_random_felts_to_prover_1() {
    let mut channel = Transcript::new_with_counter(
        Felt::from_hex("0xc5952bab5731090a953716ac821ee7374cc6c4bad31d21b7134f62d6e00593").unwrap(),
        Felt::from_hex("0x1").unwrap(),
    );

    assert!(
        channel.random_felt_to_prover()
            == Felt::from_hex("0x2aaadb36d1b43f25fa226bf80910dbf930cde1b14b6fea843ae83ff4ed3babc")
                .unwrap(),
    );
    assert!(
        *channel.digest()
            == Felt::from_hex("0xc5952bab5731090a953716ac821ee7374cc6c4bad31d21b7134f62d6e00593")
                .unwrap()
    );
    assert!(*channel.counter() == Felt::from_hex("0x2").unwrap());
}
