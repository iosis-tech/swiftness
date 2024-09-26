# Use the official Rust image from Docker Hub
FROM rust:1.79.0-slim

# Set the working directory inside the container
WORKDIR /swiftness

# Copy the project files into the container
COPY . .

# Define environment variables for the different layouts, hashers, and stone versions
ENV LAYOUTS="dex recursive recursive_with_poseidon small starknet starknet_with_keccak dynamic"
ENV HASHERS="keccak_160_lsb keccak_248_lsb blake2s_160_lsb blake2s_248_lsb"
ENV STONES="stone5 stone6"

# Create a bash script for building and renaming all variants
RUN bash -c 'for layout in $LAYOUTS; do \
    for hasher in $HASHERS; do \
        for stone in $STONES; do \
            echo "Building swiftness verifier with layout=$layout, hasher=$hasher, stone=$stone"; \
            cargo install -f --path cli/ --no-default-features --features "${layout},${hasher},${stone}"; \
            mv /usr/local/cargo/bin/swiftness /usr/local/bin/swiftness_${layout}_${hasher}_${stone}; \
        done; \
    done; \
done'
