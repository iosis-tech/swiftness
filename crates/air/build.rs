fn main() {
    if cfg!(feature = "dynamic") {
        /*
           MIN_STACK_SIZE:
           8 MB:   8388608
           16 MB:  16777216
           32 MB:  33554432
           64 MB:  67108864
        */
        const MIN_STACK_SIZE: usize = 16_777_216;
        println!("cargo:rerun-if-env-changed=RUST_MIN_STACK");

        let rust_min_stack: usize = std::env::var("RUST_MIN_STACK")
            .expect("Environment variable RUST_MIN_STACK must be set. Set it to at least 16777216.")
            .parse()
            .expect("RUST_MIN_STACK must be a valid integer.");

        if rust_min_stack < MIN_STACK_SIZE {
            panic!(
                "RUST_MIN_STACK must be at least {}. Current value: {}.",
                MIN_STACK_SIZE, rust_min_stack
            );
        }

        macro_rules! check_feature_enabled {
                ($($feature:literal),*) => {
                    // Ensure at least one feature is enabled
                    #[cfg(not(any($(feature = $feature),*)))]
                    compile_error!(concat!(
                        "At least one feature must be enabled: ",
                        $(concat!("`", $feature, "`, ")),*
                    ));
                };
            }

        macro_rules! check_feature_conflict {
                ($feature:literal, $($all_features:expr),*) => {
                    $(
                        #[cfg(all(feature = $feature, feature = $all_features))]
                        compile_error!(concat!(
                            "Conflicting features detected: `", $feature, "` and `", $all_features, "` cannot be enabled together."
                        ));
                    )*
                };
            }

        #[rustfmt::skip]
        mod check_layout {
            check_feature_enabled!("dex", "recursive", "recursive_with_poseidon", "small", "starknet", "starknet_with_keccak", "dynamic");
            check_feature_conflict!("dex", "recursive", "recursive_with_poseidon", "small", "starknet", "starknet_with_keccak", "dynamic");
            check_feature_conflict!("recursive", "dex", "recursive_with_poseidon", "small", "starknet", "starknet_with_keccak", "dynamic");
            check_feature_conflict!("recursive_with_poseidon", "dex", "recursive", "small", "starknet", "starknet_with_keccak", "dynamic");
            check_feature_conflict!("small", "dex", "recursive", "recursive_with_poseidon", "starknet", "starknet_with_keccak", "dynamic");
            check_feature_conflict!("starknet", "dex", "recursive", "recursive_with_poseidon", "small", "starknet_with_keccak", "dynamic");
            check_feature_conflict!("starknet_with_keccak", "dex", "recursive", "recursive_with_poseidon", "small", "starknet", "dynamic");
            check_feature_conflict!("dynamic", "dex", "recursive", "recursive_with_poseidon", "small", "starknet", "starknet_with_keccak");
        }

        #[rustfmt::skip]
        mod check_stone {
            check_feature_enabled!("stone5", "stone6");
            check_feature_conflict!("stone5", "stone6");
            check_feature_conflict!("stone6", "stone5");
        }
    }
}
