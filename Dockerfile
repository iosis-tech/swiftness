# Use the official Rust image from Docker Hub
FROM rust:1.79.0-slim

# Set the working directory inside the container
WORKDIR /builder

# Copy the project files into the container
COPY . .

# Build the Rust project with optimizations
RUN cargo build --release