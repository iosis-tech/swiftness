use alloc::vec::Vec;
use num_bigint::{BigInt, TryFromBigIntError};
use starknet_crypto::{pedersen_hash, Felt};

use crate::consts::{FELT_0, FELT_3};

pub fn compute_program_hash(
    memory: &[Felt],
    initial_pc: Felt,
    initial_fp: Felt,
) -> Result<Felt, TryFromBigIntError<BigInt>> {
    let program: Vec<&Felt> = memory
        .iter()
        .skip(initial_pc.to_bigint().try_into()?)
        .step_by(2)
        .take((initial_fp - FELT_3).to_bigint().try_into()?)
        .collect();

    let hash = program.iter().fold(FELT_0, |acc, &e| pedersen_hash(&acc, e));
    let program_hash = pedersen_hash(&hash, &Felt::from(program.len()));

    Ok(program_hash)
}
