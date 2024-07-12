use crate::fixtures::config;
use starknet_crypto::Felt;

#[test]
fn test_fri_config() {
    let fri_config = config::get();
    let log_n_cosets = Felt::from_hex("0x2").unwrap();
    let n_verifier_friendly_commitment_layers = Felt::from_hex("0x64").unwrap();
    let log_expected_input_degree = Felt::from_hex("0x12").unwrap();

    assert_eq!(
        fri_config.validate(log_n_cosets, n_verifier_friendly_commitment_layers).unwrap(),
        log_expected_input_degree
    );
}
