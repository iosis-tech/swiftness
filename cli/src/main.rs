use std::path::PathBuf;

use cairovm_verifier_air::layout::recursive::RecursiveLayout;
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
    let result = stark_proof.verify::<RecursiveLayout>(security_bits)?;
    println!("{:?}", result);
    Ok(())
}
