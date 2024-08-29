# Swiftness CairoVM Verifier

<div align="center">

![Version](https://img.shields.io/badge/v0.0.8-green?style=flat-square&logo=git&logoColor=white&label=version)
![Continuous Integration](https://img.shields.io/github/actions/workflow/status/iosis-tech/swiftness/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=Continuous%20Integration)

[![Crates.io Version](https://img.shields.io/crates/v/swiftness?style=flat-square&logo=lootcrate)](https://crates.io/crates/swiftness)
[![docs.rs](https://img.shields.io/docsrs/swiftness?style=flat-square&logo=docsdotrs)](https://docs.rs/swiftness/)
[![hub.docker](https://img.shields.io/docker/pulls/okm165/swiftness?style=flat-square&logo=docker&logoColor=white&label=docker)](https://hub.docker.com/repository/docker/okm165/swiftness/tags)

</div>

Swiftness is a Rust implementation of the Cairo-VM STARK verifier with layouts, inspired by StarkWare's [Cairo-verifier](https://github.com/starkware-libs/cairo-lang) in Cairo0.

## Example Usage

```js
import init, { verify_proof } from "swiftness-{layout}-{commitment hash}";

async function run(proof_json) {
  await init(); // Initialize the Wasm module
  try {
    const [programHash, programOutput] = JSON.parse(
      await verify_proof(proof_json),
    );
  } catch (err) {
    console.error(`Verification failed: ${err}`);
  }
}
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve this project.
