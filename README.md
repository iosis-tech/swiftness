# Swiftness CairoVM Verifier

<div align="center">

![Version](https://img.shields.io/badge/v0.0.8-green?style=flat-square&logo=git&logoColor=white&label=version)
![Continuous Integration](https://img.shields.io/github/actions/workflow/status/iosis-tech/swiftness/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=Continuous%20Integration)

[![Crates.io Version](https://img.shields.io/crates/v/swiftness?style=flat-square&logo=lootcrate)](https://crates.io/crates/swiftness)
[![docs.rs](https://img.shields.io/docsrs/swiftness?style=flat-square&logo=docsdotrs)](https://docs.rs/swiftness/)
[![hub.docker](https://img.shields.io/docker/pulls/okm165/swiftness?style=flat-square&logo=docker&logoColor=white&label=docker)](https://hub.docker.com/repository/docker/okm165/swiftness/tags)

</div>

Swiftness is a Rust implementation of the Cairo-VM STARK verifier with layouts, inspired by StarkWare's [Cairo-verifier](https://github.com/starkware-libs/cairo-lang) in Cairo0.

## Getting Started

### Verify an Example Proof

1. **Install `swiftness`:**

   ```sh
   cargo install -f --path cli/ --features starknet_with_keccak,keccak_160_lsb --no-default-features
   ```

2. **Verify the proof:**

   Ensure you use a proof corresponding to the layout and hash used to build the binary.

   ```sh
   swiftness --proof examples/proofs/starknet_with_keccak/cairo0_example_proof.json
   ```

3. **Local Run:**

   ```sh
   cd cli && cargo run --release --bin swiftness --features starknet_with_keccak,keccak_160_lsb --no-default-features -- --proof ../examples/proofs/starknet_with_keccak/cairo0_example_proof.json
   ```

## Running Tests

```sh
cargo test
```

## WebAssembly (WASM) Setup

1. **Install `wasm-pack`:**

   ```sh
   cargo install wasm-pack
   ```

2. **Build WASM:**

   ```sh
   cd wasm_bindings && wasm-pack build --target web --features recursive_with_poseidon,blake2s_248_lsb --no-default-features
   ```

## Features

### Usage

- [x] CLI

### Implemented Layouts

- [x] dex
- [x] recursive
- [x] recursive_with_poseidon
- [x] small
- [x] starknet
- [x] starknet_with_keccak

### Commitment Hashes

- [x] keccak_160_lsb
- [x] keccak_248_lsb
- [x] blake2s_160_lsb
- [x] blake2s_248_lsb

### Web Support

- [x] WASM support
- [x] [Web Demo](https://demo.swiftness.iosis.tech/)
- [x] NPM packages:
  - [swiftness-dex-blake2s](https://www.npmjs.com/package/swiftness-dex-blake2s)
  - [swiftness-dex-keccak](https://www.npmjs.com/package/swiftness-dex-keccak)
  - [swiftness-recursive-blake2s](https://www.npmjs.com/package/swiftness-recursive-blake2s)
  - [swiftness-recursive-keccak](https://www.npmjs.com/package/swiftness-recursive-keccak)
  - [swiftness-recursive-with-poseidon-blake2s](https://www.npmjs.com/package/swiftness-recursive-with-poseidon-blake2s)
  - [swiftness-recursive-with-poseidon-keccak](https://www.npmjs.com/package/swiftness-recursive-with-poseidon-keccak)
  - [swiftness-small-blake2s](https://www.npmjs.com/package/swiftness-small-blake2s)
  - [swiftness-small-keccak](https://www.npmjs.com/package/swiftness-small-keccak)
  - [swiftness-starknet-blake2s](https://www.npmjs.com/package/swiftness-starknet-blake2s)
  - [swiftness-starknet-keccak](https://www.npmjs.com/package/swiftness-starknet-keccak)
  - [swiftness-starknet-with-keccak-blake2s](https://www.npmjs.com/package/swiftness-starknet-with-keccak-blake2s)
  - [swiftness-starknet-with-keccak-keccak](https://www.npmjs.com/package/swiftness-starknet-with-keccak-keccak)

### Other Platforms Support

- [x] Bare Armv7-R, Big Endian - `armebv7r-none-eabi`
- [x] Bare RISC-V (RV64IMAC ISA) - `riscv64imac-unknown-none-elf`
- [x] Bare ARM64, softfloat - `aarch64-unknown-none-softfloat`
- [x] Bare ARM64, hardfloat - `aarch64-unknown-none`

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve this project.
