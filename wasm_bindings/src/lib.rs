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

use serde_json::json;
use swiftness::{transform::TransformTo, types::StarkProof as StarkProofVerifier};
use swiftness_proof_parser::parse;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_proof(stark_proof: JsValue) -> Result<JsValue, JsError> {
    // Deserialize input from JsValue to Rust types
    let proof_str: String = stark_proof
        .as_string()
        .ok_or_else(|| JsError::new("Failed to convert input to string. Expected a valid string representation of the proof"))?;

    // Parse the proof
    let stark_proof: StarkProofVerifier = parse(proof_str)
        .map_err(|e| JsError::new(&format!("Error parsing proof: {}", e)))?
        .transform_to();

    // Serialize result to JsValue
    let result = serde_json::to_value(stark_proof)
        .map_err(|e| JsError::new(&format!("Failed to serialize proof to JSON: {}", e)))?;

    Ok(JsValue::from_str(&result.to_string()))
}

#[wasm_bindgen]
pub fn verify_proof(stark_proof: JsValue) -> Result<JsValue, JsError> {
    // Deserialize proof from JsValue
    let proof_str = stark_proof.as_string().ok_or_else(|| {
        JsError::new(
            "Input is not a valid string. Expected a JSON string representation of the proof",
        )
    })?;

    let stark_proof: StarkProofVerifier = serde_json::from_str(&proof_str)
        .map_err(|e| JsError::new(&format!("Error deserializing proof: {}", e)))?;

    // Get security bits and verify
    let security_bits = stark_proof.config.security_bits();
    let (program_hash, output_hash) = stark_proof
        .verify::<Layout>(security_bits)
        .map_err(|e| JsError::new(&format!("Verification failed: {}", e)))?;

    // Serialize result to JsValue
    let result = serde_json::to_value(json!({
        "program_hash": program_hash,
        "output_hash": output_hash,
    }))
    .map_err(|e| {
        JsError::new(&format!("Failed to serialize verification result to JSON: {}", e))
    })?;

    Ok(JsValue::from_str(&result.to_string()))
}
