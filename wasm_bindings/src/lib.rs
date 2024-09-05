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

use swiftness::transform::TransformTo;
use swiftness_proof_parser::parse;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn verify_proof(proof: JsValue) -> Result<JsValue, JsError> {
    // Deserialize input from JsValue to Rust types
    let proof_str: String = proof.as_string().ok_or_else(|| JsError::new("Invalid input"))?;

    // Parse the proof
    let stark_proof = parse(proof_str).map_err(|e| JsError::new(&e.to_string()))?.transform_to();

    // Get security bits and verify
    let security_bits = stark_proof.config.security_bits();
    let (program_hash, output_hash) =
        stark_proof.verify::<Layout>(security_bits).map_err(|e| JsError::new(&e.to_string()))?;

    // Serialize result to JsValue
    let result = serde_json::to_value((program_hash, output_hash))
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(JsValue::from_str(&result.to_string()))
}
