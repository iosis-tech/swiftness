# Cairo-VM Verifier

![Version](https://img.shields.io/badge/v0.1.1-green?style=flat-square&logo=git&logoColor=white&label=version)
![Continuous Integration](https://img.shields.io/github/actions/workflow/status/iosis-tech/cairovm-verifier/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=Continuous%20Integration)

This is the Rust implementation of the Cairo-VM STARK verifier with layouts. The code is inspired by StarkWare's [Cairo-verifier](https://github.com/starkware-libs/cairo-lang) implementation in Cairo0.

## Build Project

```sh
cargo build
```

## Run Tests

```sh
cargo test
```

## Features

- **Usage:**
  - [ ] CLI support for easy interaction

- **Implemented Layouts:**
  - [x] dex
  - [x] recursive
  - [x] recursive_with_poseidon
  - [ ] small
  - [ ] starknet
  - [ ] starknet_with_keccak

- **WASM Support:**
  - [ ] Wasm bindings for integration with web applications
  - [ ] Example Cairo-VM STARK verifier web client

## Contributing

Feel free to open issues or submit pull requests to help us improve this project.