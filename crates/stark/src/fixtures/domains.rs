use cairovm_verifier_air::domains::StarkDomains;
use starknet_crypto::Felt;

pub fn get() -> StarkDomains {
    StarkDomains {
        log_eval_domain_size: Felt::from_hex_unchecked("0x14"),
        eval_domain_size: Felt::from_hex_unchecked("0x100000"),
        eval_generator: Felt::from_hex_unchecked(
            "0x594beafca8a00d9581d81caee93dc85c727c9af7fc4c648e3d47b998574e81f",
        ),
        log_trace_domain_size: Felt::from_hex_unchecked("0x12"),
        trace_domain_size: Felt::from_hex_unchecked("0x40000"),
        trace_generator: Felt::from_hex_unchecked(
            "0x4768803ef85256034f67453635f87997ff61841e411ee63ce7b0a8b9745a046",
        ),
    }
}
