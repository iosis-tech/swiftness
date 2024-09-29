#!/bin/bash

# Define arrays for the different layouts, hashers, and stones
LAYOUTS=("dex" "recursive" "recursive_with_poseidon" "small" "starknet" "starknet_with_keccak" "dynamic")
HASHERS=("keccak_160_lsb" "keccak_248_lsb" "blake2s_160_lsb" "blake2s_248_lsb")
STONES=("stone5" "stone6")

# Function to build WASM packages
build_wasm_packages() {
    for layout in "${LAYOUTS[@]}"; do
        for hasher in "${HASHERS[@]}"; do
            for stone in "${STONES[@]}"; do
                # Define output directory and features
                output_dir="src/pkg/swiftness_${layout}_${hasher}_${stone}"
                features="${layout},${hasher},${stone}"

                # Print message
                echo "Building WASM package with layout=$layout, hasher=$hasher, stone=$stone"

                # Run wasm-pack build with the specified features and output directory
                wasm-pack build --out-dir "$output_dir" --target web --features "$features" --no-default-features
                rm "$output_dir/.gitignore";
                rm "$output_dir/README.md";
            done
        done
    done
}

# Function to generate import statements
generate_imports() {
    local output_file="generated_imports.ts"
    > "$output_file"  # Clear the file if it exists

    for layout in "${LAYOUTS[@]}"; do
        for hasher in "${HASHERS[@]}"; do
            for stone in "${STONES[@]}"; do
                # Construct package and function names
                pkg_name="swiftness_${layout}_${hasher}_${stone}"
                parse_fn="parse_proof_${pkg_name}"
                verify_fn="verify_proof_${pkg_name}"

                # Generate import statement and append to output file
                echo "import init_${pkg_name}, {" >> "$output_file"
                echo "    parse_proof as ${parse_fn}," >> "$output_file"
                echo "    verify_proof as ${verify_fn}," >> "$output_file"
                echo "} from \"./pkg/${pkg_name}\";" >> "$output_file"
                echo "" >> "$output_file"
            done
        done
    done

    echo "Import statements have been generated and saved to $output_file."
}

# Function to generate the VerifierMap
generate_verifier_map() {
    local output_file="generated_verifier_map.ts"
    > "$output_file"  # Clear the file if it exists

    # Write the initial part of the VerifierMap object
    echo "const verifier_map: VerifierMap = {" >> "$output_file"

    for layout in "${LAYOUTS[@]}"; do
        for hasher in "${HASHERS[@]}"; do
            for stone in "${STONES[@]}"; do
                # Construct the unique identifiers
                map_key="\${Layout.${layout^^}}_\${Commitment.${hasher^^}}_\${Stone.${stone^^}}"
                pkg_name="swiftness_${layout}_${hasher}_${stone}"
                parse_fn="parse_proof_${pkg_name}"
                verify_fn="verify_proof_${pkg_name}"

                # Generate TypeScript entry for each combination
                echo "    [\`${map_key}\`]: [" >> "$output_file"
                echo "        init_${pkg_name}," >> "$output_file"
                echo "        ${parse_fn}," >> "$output_file"
                echo "        ${verify_fn}," >> "$output_file"
                echo "    ]," >> "$output_file"
            done
        done
    done

    # Close the VerifierMap object
    echo "};" >> "$output_file"
    echo "Verifier map has been generated and saved to $output_file."
}

# Main execution
build_wasm_packages
# generate_imports
# generate_verifier_map
