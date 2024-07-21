#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod annotations;
mod builtins;
mod conversion;
mod json_parser;
mod layout;
mod stark_proof;
mod utils;

use crate::{json_parser::ProofJSON, stark_proof::StarkProof};
use alloc::string::String;
use cairovm_verifier_stark::types::StarkProof as StarkProofFromVerifier;

pub fn parse(input: String) -> anyhow::Result<StarkProofFromVerifier> {
    let proof_json = serde_json::from_str::<ProofJSON>(&input)?;
    let stark_proof = StarkProof::try_from(proof_json)?;
    let stark_proof_verifier: StarkProofFromVerifier = stark_proof.into();
    Ok(stark_proof_verifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../../examples/proofs/recursive/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }
}
