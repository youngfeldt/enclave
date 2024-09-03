# Use a multi-stage build to compile Rust for musl target
FROM rust:1.70-slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    musl-tools

# Set up the build environment
WORKDIR /usr/src/enclave_attestation

# Copy source code
COPY . .

# Build the binary for musl
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# Create the final image using scratch as it is minimal
FROM scratch

# Copy the binary
COPY --from=builder /usr/src/enclave_attestation/target/x86_64-unknown-linux-musl/release/enclave_attestation /enclave_attestation

# Command to run the binary
CMD ["/enclave_attestation"]

