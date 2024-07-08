use crate::fri::fri_verify;

use super::*;

#[test]
fn test_fri_verify() {
    let queries = cairovm_verifier_common::queries::get();
    let commitment = commit::get();
    let decommitment = decommit::get();
    let withness = witness::get();

    fri_verify(queries, commitment, decommitment, withness).unwrap();
}
