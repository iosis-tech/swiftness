use cairovm_verifier_air::layout::recursive::{NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND};
use cairovm_verifier_fri::tests::queries;

use crate::verify::stark_verify;

use super::{commitment, domains, witness};

#[test]
pub fn test_stark_verify() {
    let queries = queries::get();
    let commitment = commitment::get();
    let witness = witness::get();
    let stark_domains = domains::get();

    stark_verify(
        NUM_COLUMNS_FIRST as usize,
        NUM_COLUMNS_SECOND as usize,
        &queries,
        commitment,
        &witness,
        &stark_domains,
    )
    .unwrap()
}
