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
    }
}
