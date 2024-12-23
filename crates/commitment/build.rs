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

    macro_rules! assert_unique_feature {
        () => {};
        ($first:tt $(,$rest:tt)*) => {
            $(
                #[cfg(all(feature = $first, feature = $rest))]
                compile_error!(concat!("features \"", $first, "\" and \"", $rest, "\" cannot be used together"));
            )*
            assert_unique_feature!($($rest),*);
        }
    }

    #[rustfmt::skip]
    mod check_hash {
        check_feature_enabled!("keccak_160_lsb", "keccak_248_lsb", "blake2s_160_lsb", "blake2s_248_lsb");
        assert_unique_feature!("keccak_160_lsb", "keccak_248_lsb", "blake2s_160_lsb", "blake2s_248_lsb");
    }
}
