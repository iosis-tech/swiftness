use cairovm_verifier_air::{fixtures::public_input, layout::recursive::Layout};
use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

use crate::{
    commit::stark_commit,
    fixtures::{commitment, config, domains, unsent_commitment},
};

#[test]
pub fn test_stark_commit() {
    let mut transcript = Transcript::new_with_counter(
        Felt::from_hex_unchecked(
            "0xaf91f2c71f4a594b1575d258ce82464475c82d8fb244142d0db450491c1b52",
        ),
        Felt::from_hex_unchecked("0x0"),
    );

    let public_input = public_input::get();
    let unsent_commitment = unsent_commitment::get();
    let config = config::get();
    let stark_domains = domains::get();

    assert_eq!(
        stark_commit::<Layout>(
            &mut transcript,
            &public_input,
            &unsent_commitment,
            &config,
            &stark_domains
        )
        .unwrap(),
        commitment::get()
    );

    assert_eq!(
        *transcript.digest(),
        Felt::from_hex_unchecked(
            "0x28f12249c8cba51796d59e7573019ce2b4608c9a8cdeee26e821b0763c69229"
        )
    );

    assert_eq!(*transcript.counter(), Felt::from_hex_unchecked("0x0"))
}
