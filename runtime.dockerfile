# Use the official Rust image from Docker Hub
FROM rust:1.79.0-slim

# Set the working directory inside the container
WORKDIR /swiftness

# Copy the project files into the container
COPY . .

# Build the Rust project with optimizations
RUN cargo build --release

# Define build arguments for features
ARG FEATURES="starknet_with_keccak,blake2s"

# Use the build arguments in the cargo install command
RUN cargo install -f --path cli/ --features $FEATURES --no-default-features