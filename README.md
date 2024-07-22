# Swiftness CairoVM Verifier

![Version](https://img.shields.io/badge/v0.0.3-green?style=flat-square&logo=git&logoColor=white&label=version)
![Continuous Integration](https://img.shields.io/github/actions/workflow/status/iosis-tech/swiftness/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=Continuous%20Integration)

This is the Rust implementation of the Cairo-VM STARK verifier with layouts. The code is inspired by StarkWare's [Cairo-verifier](https://github.com/starkware-libs/cairo-lang) implementation in Cairo0.

### Verify example proof:

```sh
cargo run --release --bin swiftness --features std,starknet_with_keccak,keccak --no-default-features -- --proof examples/proofs/starknet_with_keccak/cairo0_example_proof.json 
```

## Run Tests

```sh
cargo test
```

### Install wasm-pack
```sh
cargo install wasm-pack
```

### Build WASM:

```sh
cd wasm-binding && wasm-pack build --target web --workspace --features std,starknet_with_keccak,blake2s --no-default-features
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
  - [x] NPM packages published
    - [swiftness](https://www.npmjs.com/package/swiftness-dex-blake2s) - layout: `dex` - hash: `blake2s`
    - [swiftness](https://www.npmjs.com/package/swiftness-dex-keccak) - layout: `dex` - hash: `keccak`
    - [swiftness](https://www.npmjs.com/package/swiftness-recursive-blake2s) - layout: `recursive` - hash: `blake2s`
    - [swiftness](https://www.npmjs.com/package/swiftness-recursive-keccak) - layout: `recursive` - hash: `keccak`
    - [swiftness](https://www.npmjs.com/package/swiftness-recursive-with-poseidon-blake2s) - layout: `recursive_with_poseidon` - hash: `blake2s`
    - [swiftness](https://www.npmjs.com/package/swiftness-recursive-with-poseidon-keccak) - layout: `recursive_with_poseidon` - hash: `keccak`
    - [swiftness](https://www.npmjs.com/package/swiftness-small-blake2s) - layout: `small` - hash: `blake2s`
    - [swiftness](https://www.npmjs.com/package/swiftness-small-keccak) - layout: `small` - hash: `keccak`
    - [swiftness](https://www.npmjs.com/package/swiftness-starknet-blake2s) - layout: `starknet` - hash: `blake2s`
    - [swiftness](https://www.npmjs.com/package/swiftness-starknet-keccak) - layout: `starknet` - hash: `keccak`
    - [swiftness](https://www.npmjs.com/package/swiftness-starknet-with-keccak-blake2s) - layout: `starknet_with_keccak` - hash: `blake2s`
    - [swiftness](https://www.npmjs.com/package/swiftness-starknet-with-keccak-keccak) - layout: `starknet_with_keccak` - hash: `keccak`

## Contributing

Feel free to open issues or submit pull requests to help improve this project.
