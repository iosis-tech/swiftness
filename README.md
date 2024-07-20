# Swiftness CairoVM Verifier

![Version](https://img.shields.io/badge/v0.0.3-green?style=flat-square&logo=git&logoColor=white&label=version)
![Continuous Integration](https://img.shields.io/github/actions/workflow/status/iosis-tech/cairovm-verifier/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=Continuous%20Integration)

This is the Rust implementation of the Cairo-VM STARK verifier with layouts. The code is inspired by StarkWare's [Cairo-verifier](https://github.com/starkware-libs/cairo-lang) implementation in Cairo0.

### Verify example proof:

```sh
cargo run --release --bin swiftness_cli --features starknet_with_keccak,keccak --no-default-features -- --proof examples/proofs/starknet_with_keccak/cairo0_example_proof.json 
```

## Run Tests

```sh
cargo test
```

## Features

- **Usage:**

  - [x] CLI support for easy interaction

- **Implemented Layouts:**

  - [x] dex
  - [x] recursive
  - [x] recursive_with_poseidon
  - [x] small
  - [x] starknet
  - [x] starknet_with_keccak

- **Web Support:**
  - [x] Wasm Support
  - [x] NPM package

## Contributing

Feel free to open issues or submit pull requests to help improve this project.
