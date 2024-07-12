use cairovm_verifier_air::fixtures::interaction_elements;
use starknet_crypto::Felt;

use crate::{fixtures::*, oods::verify_oods};

// #[test]
// fn test_verify_oods() {
//     let public_input = public_input::get();
//     let interaction_elements = interaction_elements::get();
//     let mask_values = oods_values::get();
//     let constraint_coefficients = constraint_coefficients::get();

//     verify_oods(
//         &mask_values,
//         interaction_elements,
//         &public_input,
//         &constraint_coefficients,
//         Felt::from_hex_unchecked(
//             "0x47148421d376a8ca07af1e4c89890bf29c90272f63b16103646397d907281a8",
//         ),
//         Felt::from_hex_unchecked("0x40000"),
//         Felt::from_hex_unchecked(
//             "0x4768803ef85256034f67453635f87997ff61841e411ee63ce7b0a8b9745a046",
//         ),
//     );
// }
