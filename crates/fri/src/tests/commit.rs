use starknet_crypto::Felt;
use swiftness_transcript::transcript::Transcript;

use crate::{
    fixtures::{commitment, config, unsent_commitment},
    fri::fri_commit,
};

#[test]
fn test_fri_commit() {
    let mut transcript = Transcript::new_with_counter(
        Felt::from_hex_unchecked(
            "0x3612d68f9f68b263d83b0854b812018fd1b7ba0359d7514fffdabd44f0696e6",
        ),
        Felt::from_hex_unchecked("0x1"),
    );

    let fri_config = config::get();
    let unsent_commitment = unsent_commitment::get();

    assert_eq!(fri_commit(&mut transcript, unsent_commitment, fri_config), commitment::get())
}
