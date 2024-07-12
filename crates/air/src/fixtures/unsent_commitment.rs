use starknet_crypto::Felt;

use crate::trace::UnsentCommitment;

pub fn get() -> UnsentCommitment {
    UnsentCommitment {
        original: Felt::from_hex_unchecked(
            "0x2a588e8517b956684162e05e373dc6891146c1853c82d3984fbc707ae937972",
        ),
        interaction: Felt::from_hex_unchecked(
            "0x7171ffc67e24fcbb2a7d1acd6244fa91c54dff15c96ca26d193907b716ce2c5",
        ),
    }
}
