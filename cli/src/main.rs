use std::path::PathBuf;

#[cfg(feature = "dex")]
use cairovm_verifier_air::layout::dex::Layout;
#[cfg(feature = "recursive")]
use cairovm_verifier_air::layout::recursive::Layout;
#[cfg(feature = "recursive_with_poseidon")]
use cairovm_verifier_air::layout::recursive_with_poseidon::Layout;
#[cfg(feature = "small")]
use cairovm_verifier_air::layout::small::Layout;
#[cfg(feature = "starknet")]
use cairovm_verifier_air::layout::starknet::Layout;
#[cfg(feature = "starknet_with_keccak")]
use cairovm_verifier_air::layout::starknet_with_keccak::Layout;

use cairovm_verifier_proof_parser::parse;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct CairoVMVerifier {
    /// Path to proof JSON file
    #[clap(short, long)]
    proof: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CairoVMVerifier::parse();
    let stark_proof = parse(std::fs::read_to_string(cli.proof)?)?;
    let security_bits = stark_proof.config.security_bits();
    let result = stark_proof.verify::<Layout>(security_bits)?;
    println!("{:?}", result);
    Ok(())
}
