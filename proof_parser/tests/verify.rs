use swiftness::{
    config::StarkConfig,
    types::{StarkProof, StarkUnsentCommitment, StarkWitness},
    TransformTo,
};
use swiftness_air::{layout::recursive::Layout, public_memory::PublicInput, trace::Witness};
use swiftness_proof_parser::stark_proof::StarkProof as StarkProofParser;

fn read_proof() -> StarkProof {
    bincode::deserialize::<StarkProofParser>(include_bytes!("proof.bin")).unwrap().transform_to()
}

#[test]
fn test_bytemuck_proof() {
    let stark_proof: StarkProof = read_proof();

    let stark_proof_bytes = bytemuck::bytes_of(&stark_proof);
    let stark_proof_back = bytemuck::from_bytes::<StarkProof>(&stark_proof_bytes);

    assert_eq!(stark_proof, *stark_proof_back);
}

#[test]
fn test_bytemuck_config() {
    let stark_proof: StarkConfig = read_proof().config;

    let stark_proof_bytes = bytemuck::bytes_of(&stark_proof);
    let stark_proof_back = bytemuck::from_bytes::<StarkConfig>(&stark_proof_bytes);

    assert_eq!(stark_proof, *stark_proof_back);
}

#[test]
fn test_bytemuck_public_input() {
    let stark_proof: PublicInput = read_proof().public_input;

    let stark_proof_bytes = bytemuck::bytes_of(&stark_proof);
    let stark_proof_back = bytemuck::from_bytes::<PublicInput>(&stark_proof_bytes);

    assert_eq!(stark_proof, *stark_proof_back);
}

#[test]
fn test_bytemuck_unsent_commitment() {
    let stark_proof: StarkUnsentCommitment = read_proof().unsent_commitment;

    let stark_proof_bytes = bytemuck::bytes_of(&stark_proof);
    let stark_proof_back = bytemuck::from_bytes::<StarkUnsentCommitment>(&stark_proof_bytes);

    assert_eq!(stark_proof, *stark_proof_back);
}

#[test]
fn test_bytemuck_witness() {
    let stark_proof: StarkWitness = read_proof().witness;

    let stark_proof_bytes = bytemuck::bytes_of(&stark_proof);
    let stark_proof_back = bytemuck::from_bytes::<StarkWitness>(&stark_proof_bytes);

    assert_eq!(stark_proof, *stark_proof_back);
}

#[ignore]
#[test]
fn test_verify() {
    let stark_proof = read_proof();

    let transformed_proof = bincode::serialize(&stark_proof).unwrap();
    let stark_proof = bincode::deserialize::<StarkProof>(&transformed_proof).unwrap();

    let security_bits = stark_proof.config.security_bits();

    let (program_hash, _output) = stark_proof.verify::<Layout>(security_bits).unwrap();
    assert_eq!(
        program_hash.to_hex_string(),
        "0x2820cfb261b9ffa9f5fe7af15ff3d4df545154f26bfd4f234d1f6ba18171157"
    );
}

#[test]
fn test_transform_config() {
    let stark_proof = include_bytes!("proof.bin");
    let stark_proof = bincode::deserialize::<StarkProofParser>(stark_proof).unwrap();
    let stark_proof_config: StarkConfig = stark_proof.transform_to().config;
    let transformed_proof = bincode::serialize(&stark_proof_config).unwrap();
    let _stark_proof_config = bincode::deserialize::<StarkConfig>(&transformed_proof).unwrap();
}

#[ignore]
#[test]
fn test_transform_public_input() {
    let stark_proof = include_bytes!("proof.bin");
    let stark_proof = bincode::deserialize::<StarkProofParser>(stark_proof).unwrap();
    let stark_proof_public_input: PublicInput = stark_proof.public_input.transform_to();
    let transformed_proof = bincode::serialize(&stark_proof_public_input).unwrap();
    let _stark_proof_public_input =
        bincode::deserialize::<PublicInput>(&transformed_proof).unwrap();
}

#[test]
fn test_transform_witness() {
    let stark_proof_witness = read_proof().witness;
    let transformed_proof = bincode::serialize(&stark_proof_witness).unwrap();
    let _stark_proof_witness = bincode::deserialize::<Witness>(&transformed_proof).unwrap();
}

#[test]
fn test_transform_unsent_commitment() {
    let stark_proof_unsent_commitment = read_proof().unsent_commitment;
    let transformed_proof = bincode::serialize(&stark_proof_unsent_commitment).unwrap();
    let _stark_proof_unsent_commitment =
        bincode::deserialize::<StarkUnsentCommitment>(&transformed_proof).unwrap();
}
