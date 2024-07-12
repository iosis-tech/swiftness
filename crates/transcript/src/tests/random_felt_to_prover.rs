use starknet_crypto::Felt;

use crate::transcript::Transcript;

#[test]
fn test_random_felts_to_prover_0() {
    let mut transcript = Transcript::new_with_counter(
        Felt::from_hex_unchecked(
            "0x55c2e30068db90407013a4cfcedee7895b328c6ba64b8bd5e4c71e470af5fde",
        ),
        Felt::from_hex_unchecked("0x0"),
    );

    assert!(
        transcript.random_felt_to_prover()
            == Felt::from_hex_unchecked(
                "0x120149e03e4939d3c2a4ca2fcaa6e9cfff0c64fbe115f54c439d76ff09c3dc7"
            ),
    );
    assert!(
        *transcript.digest()
            == Felt::from_hex_unchecked(
                "0x55c2e30068db90407013a4cfcedee7895b328c6ba64b8bd5e4c71e470af5fde"
            ),
    );

    assert!(*transcript.counter() == Felt::from_hex_unchecked("0x1"));
}

#[test]
fn test_random_felts_to_prover_1() {
    let mut transcript = Transcript::new_with_counter(
        Felt::from_hex_unchecked(
            "0xc5952bab5731090a953716ac821ee7374cc6c4bad31d21b7134f62d6e00593",
        ),
        Felt::from_hex_unchecked("0x1"),
    );

    assert!(
        transcript.random_felt_to_prover()
            == Felt::from_hex_unchecked(
                "0x2aaadb36d1b43f25fa226bf80910dbf930cde1b14b6fea843ae83ff4ed3babc"
            ),
    );
    assert!(
        *transcript.digest()
            == Felt::from_hex_unchecked(
                "0xc5952bab5731090a953716ac821ee7374cc6c4bad31d21b7134f62d6e00593"
            )
    );
    assert!(*transcript.counter() == Felt::from_hex_unchecked("0x2"));
}
