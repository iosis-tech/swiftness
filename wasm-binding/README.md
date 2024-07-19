# Cairo-VM Verifier WASM

![Version](https://img.shields.io/badge/v0.1.1-green?style=flat-square&logo=git&logoColor=white&label=version)
![Continuous Integration](https://img.shields.io/github/actions/workflow/status/iosis-tech/cairovm-verifier/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=Continuous%20Integration)

This is the Rust implementation of the Cairo-VM STARK verifier with layouts. The code is inspired by StarkWare's [Cairo-verifier](https://github.com/starkware-libs/cairo-lang) implementation in Cairo0.

### Install wasm-pack
```sh
cargo install wasm-pack
```

### Build WASM:

```sh
wasm-pack build --workspace --features starknet_with_keccak,blake2s --no-default-features --target wasm32-unknown-unknown
```

## Contributing

Feel free to open issues or submit pull requests to help improve this project.
