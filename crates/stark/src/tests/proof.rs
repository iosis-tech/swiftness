use crate::{
    fixtures::{config, unsent_commitment, witness},
    types::StarkProof,
};
use starknet_crypto::Felt;
use swiftness_air::{fixtures::public_input, layout::recursive::Layout};

#[test]
fn test_stark_proof_fibonacci_verify() {
    let security_bits: Felt = Felt::from_hex_unchecked("0x32");

    let stark_proof = StarkProof {
        config: config::get(),
        public_input: public_input::get(),
        unsent_commitment: unsent_commitment::get(),
        witness: witness::get(),
    };

    let (program_hash, output_hash) = stark_proof.verify::<Layout>(security_bits).unwrap();
    assert_eq!(
        program_hash,
        Felt::from_hex_unchecked(
            "0x9f6693f4a5610a46b5d71ef573c43bef5f0d111fc1c5e506d509c458a29bae"
        )
    );
    assert_eq!(
        output_hash,
        Felt::from_hex_unchecked(
            "0x615be5409121774c863b7dfefd55d0bcab6d1d09eeecbbc4ee3b88daaa69c81"
        )
    );
}
