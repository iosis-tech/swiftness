use crate::{
    fixtures::{commitment, queries, witness},
    fri::fri_verify,
};

use super::*;

#[test]
fn test_fri_verify() {
    let queries = queries::get();
    let commitment = commitment::get();
    let decommitment = decommit::get();
    let withness = witness::get();

    fri_verify(&queries, commitment, decommitment, withness).unwrap();
}
