use starknet_crypto::Felt;

use crate::{
    fixtures::{config, public_input, unsent_commitment},
    types::StarkProof,
};

// #[test]
// fn test_stark_proof_fibonacci_verify() {
//     let security_bits = Felt::from_hex_unchecked("0x32");

//     let stark_proof = StarkProof {
//         config: config::get(),
//         public_input: public_input::get(),
//         unsent_commitment: unsent_commitment::get(),
//         witness:witness
//     }
// }
