use cairovm_verifier_air::layout::recursive;
use cairovm_verifier_proof_parser::parse;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_verify(proof: JsValue) -> Result<JsValue, JsValue> {
    // Deserialize input from JsValue to Rust types
    let proof_str: String = proof.as_string().ok_or_else(|| JsValue::from_str("Invalid input"))?;

    // Parse the proof
    let stark_proof = parse(proof_str).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Get security bits and verify
    let security_bits = stark_proof.config.security_bits();
    let (program_hash, output_hash) = stark_proof
        .verify::<recursive::Layout>(security_bits)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Serialize result to JsValue
    let result = serde_json::to_value((program_hash, output_hash))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&result.to_string()))
}
