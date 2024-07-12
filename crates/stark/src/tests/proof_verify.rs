use cairovm_verifier_air::tests::public_input;
use starknet_crypto::Felt;

use crate::types::StarkProof;

use super::{config, unsent_commitment, witness};

#[test]
fn test_stark_proof_fibonacci_verify() {
    let security_bits: Felt = Felt::from_hex_unchecked("0x32");

    let stark_proof = StarkProof {
        config: config::get(),
        public_input: public_input::get(),
        unsent_commitment: unsent_commitment::get(),
        witness: witness::get(),
    };

    stark_proof.verify(security_bits).unwrap();
}
