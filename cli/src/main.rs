pub mod transform;

use clap::Parser;
use std::path::PathBuf;
pub use swiftness_proof_parser::*;
pub use swiftness_stark::*;
pub use transform::TransformTo;

#[cfg(feature = "dex")]
use swiftness_air::layout::dex::Layout;
#[cfg(feature = "dynamic")]
use swiftness_air::layout::dynamic::Layout;
#[cfg(feature = "recursive")]
use swiftness_air::layout::recursive::Layout;
#[cfg(feature = "recursive_with_poseidon")]
use swiftness_air::layout::recursive_with_poseidon::Layout;
#[cfg(feature = "small")]
use swiftness_air::layout::small::Layout;
#[cfg(feature = "starknet")]
use swiftness_air::layout::starknet::Layout;
#[cfg(feature = "starknet_with_keccak")]
use swiftness_air::layout::starknet_with_keccak::Layout;

#[derive(Parser)]
#[command(author, version, about)]
struct CairoVMVerifier {
    /// Path to proof JSON file
    #[clap(short, long)]
    proof: PathBuf,
    /// Whether to print the pedersen hash of the output instead of the full output.
    #[clap(short = 'o', long, num_args=0..=1, default_missing_value = "true")]
    print_output_hash: Option<bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CairoVMVerifier::parse();
    let stark_proof = parse(std::fs::read_to_string(cli.proof)?)?.transform_to();
    let security_bits = stark_proof.config.security_bits();
    let (program_hash, program_output) = stark_proof.verify::<Layout>(security_bits)?;

    println!("program hash: {:#x}", program_hash);
    if let Some(true) = cli.print_output_hash {
        let hash = starknet_core::crypto::compute_hash_on_elements(&program_output);
        println!("program output hash: {:#x}", hash);
    } else {
        println!("program output: {:x?}", program_output);
    }

    Ok(())
}
