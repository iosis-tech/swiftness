use crate::{
    fixtures::{config, unsent_commitment, witness},
    types::StarkProof,
};
use cairovm_verifier_air::{fixtures::public_input, layout::recursive::Layout};
use starknet_crypto::Felt;

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
            "0x603f45d671891116de1e763d11d71d25102ff93707dafc97a8d06e18145baf5"
        )
    );
    assert_eq!(
        output_hash,
        Felt::from_hex_unchecked(
            "0x21e35055ced9a22156eed737abcea133f8406f7f7e78222cf4f0f1271216adb"
        )
    );
}
