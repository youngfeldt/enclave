# Base image
FROM rust:1.68 as builder

# Install necessary packages
RUN apt-get update && apt-get install -y \
    cmake \
    musl-tools \
    build-essential

# Create an enclave app directory
WORKDIR /app

# Copy Rust source code
COPY . .

# Build the Rust application
RUN cargo build --release

# Build the final image
FROM amazonlinux:2

# Install Nitro Enclaves CLI tools
RUN yum install -y \
    amazon-nitro-enclaves-cli \
    amazon-nitro-enclaves-nsm-cli \
    && yum clean all

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/enclave-main /enclave-main

# Entry point for the Docker container
ENTRYPOINT ["/enclave-main"]
