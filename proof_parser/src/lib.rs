pub mod annotations;
pub mod builtins;
pub mod json_parser;
pub mod layout;
pub mod stark_proof;

pub use stark_proof::*;

pub fn parse<I: AsRef<str>>(input: I) -> anyhow::Result<stark_proof::StarkProof> {
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input.as_ref())?;
    stark_proof::StarkProof::try_from(proof_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dex() {
        let input = include_str!("../../examples/proofs/dex/cairo0_stone5_example_proof.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
        stark_proof::StarkProof::try_from(proof_json).unwrap();
    }

    #[test]
    fn test_parse_dynamic() {
        let input = include_str!("../../examples/proofs/dynamic/cairo0_stone6_example_proof.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
        stark_proof::StarkProof::try_from(proof_json).unwrap();
    }

    #[test]
    fn test_parse_recursive() {
        let input =
            include_str!("../../examples/proofs/recursive/cairo0_stone5_example_proof.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
        stark_proof::StarkProof::try_from(proof_json).unwrap();
    }

    #[test]
    fn test_parse_recursive_with_poseidon() {
        let input = include_str!(
            "../../examples/proofs/recursive_with_poseidon/cairo0_stone5_example_proof.json"
        );
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
        stark_proof::StarkProof::try_from(proof_json).unwrap();
    }

    #[test]
    fn test_parse_small() {
        let input = include_str!("../../examples/proofs/small/cairo0_stone5_example_proof.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
        stark_proof::StarkProof::try_from(proof_json).unwrap();
    }

    #[test]
    fn test_parse_starknet() {
        let input = include_str!("../../examples/proofs/starknet/cairo0_stone5_example_proof.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
        stark_proof::StarkProof::try_from(proof_json).unwrap();
    }

    #[test]
    fn test_parse_starknet_with_keccak() {
        let input = include_str!(
            "../../examples/proofs/starknet_with_keccak/cairo0_stone5_example_proof.json"
        );
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
        stark_proof::StarkProof::try_from(proof_json).unwrap();
    }
}
