# Swiftness Proof Parser CairoVM Verifier

## Overview
The Swiftness Proof Parser is a JSON proof parser designed for the [Stone Prover](https://github.com/starkware-libs/stone-prover). It processes the JSON output from the Stone Prover and converts it into the [StarkProof](https://github.com/iosis-tech/swiftness/blob/main/proof_parser/src/stark_proof.rs#L5-L11) struct.

## Requirements
To use this parser, ensure that the Stone Prover is executed with the `--generate_annotations` flag enabled. For additional details, refer to the [reference code](https://github.com/iosis-tech/swiftness/blob/main/examples/proofs/generate.py#L134-L140).

## Configuration

### Example Configuration
An example configuration for the Stone Prover is available [here](https://github.com/iosis-tech/swiftness/blob/main/examples/proofs/cpu_air_params.json).

### Important Notes
1. **Verifier-Friendly Channel Updates**
   Ensure that the Stone Prover is configured with the latest channel updates enabled by adding the following to your configuration:
   ```json
   "verifier_friendly_channel_updates": true
   ```
   See [example configuration](https://github.com/iosis-tech/swiftness/blob/main/examples/proofs/cpu_air_params.json#L26).

2. **First FRI Step Requirement**
   The first element of the FRI step list must be set to zero for compatibility with the verifier. See the [reference](https://github.com/iosis-tech/swiftness/blob/main/examples/proofs/cpu_air_params.json#L13).

## Usage
After generating the proof JSON output with proper annotations, use this parser to produce a `StarkProof` struct.