use cairovm_verifier_transcript::transcript::Transcript;
use starknet_crypto::Felt;

use crate::{
    table::{self, commit::table_commit},
    vector,
};

#[test]
fn test_table_commitment_commit() {
    let mut transcript = Transcript::new_with_counter(
        Felt::from_hex("0x1b9182dce9dc1169fcd00c1f8c0b6acd6baad99ce578370ead5ca230b8fb8c6")
            .unwrap(),
        Felt::from_hex("0x1").unwrap(),
    );

    let unsent_commitment =
        Felt::from_hex("0x1e9b0fa29ebe52b9c9a43a1d44e555ce42da3199370134d758735bfe9f40269")
            .unwrap();

    let config = table::config::Config {
        n_columns: Felt::from_hex("0x4").unwrap(),
        vector: vector::config::Config {
            height: Felt::from_hex("0x9").unwrap(),
            n_verifier_friendly_commitment_layers: Felt::from_hex("0x64").unwrap(),
        },
    };

    assert!(
        table_commit(&mut transcript, unsent_commitment, config.to_owned())
            == table::types::Commitment {
                config: config.to_owned(),
                vector_commitment: vector::types::Commitment {
                    config: config.vector,
                    commitment_hash: unsent_commitment
                },
            }
    );

    assert!(
        *transcript.digest()
            == Felt::from_hex("0x1abd607dab09dede570ed131d9df0a1997e33735b11933c45dc84353df84259")
                .unwrap(),
    );
    assert!(*transcript.counter() == Felt::from_hex("0x0").unwrap());
}
