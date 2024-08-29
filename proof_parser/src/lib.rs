mod annotations;
mod builtins;
mod conversion;
mod json_parser;
mod layout;
mod stark_proof;
mod utils;

use crate::{json_parser::ProofJSON, stark_proof::StarkProof};
use std::convert::TryFrom;
extern crate clap;
extern crate num_bigint;
extern crate regex;
extern crate serde;
use swiftness_stark::types::StarkProof as StarkProofFromVerifier;

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
    fn test_parse_dex() {
        let input = include_str!("../../examples/proofs/dex/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }

    #[test]
    fn test_parse_dynamic() {
        let input = include_str!("../../examples/proofs/dynamic/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }

    #[test]
    fn test_parse_recursive() {
        let input = include_str!("../../examples/proofs/recursive/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }

    #[test]
    fn test_parse_recursive_with_poseidon() {
        let input =
            include_str!("../../examples/proofs/recursive_with_poseidon/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }

    #[test]
    fn test_parse_small() {
        let input = include_str!("../../examples/proofs/small/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }

    #[test]
    fn test_parse_starknet() {
        let input = include_str!("../../examples/proofs/starknet/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }

    #[test]
    fn test_parse_starknet_with_keccak() {
        let input =
            include_str!("../../examples/proofs/starknet_with_keccak/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }
}
