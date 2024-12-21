fn main() {
    macro_rules! check_feature_enabled {
        ($($feature:literal),*) => {
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
    mod check_hash {
        check_feature_enabled!("keccak", "blake2s");
        check_feature_conflict!("keccak", "blake2s");
        check_feature_conflict!("blake2s", "keccak");
    }
}
