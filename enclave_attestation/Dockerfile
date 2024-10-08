# Base image
FROM rust:latest as builder

# Install necessary packages
RUN apt-get update && apt-get install -y \
    cmake \
    musl-tools \
    build-essential

# Create an enclave app directory
WORKDIR /app

# Copy Rust source code
COPY . .

RUN rustup update

# Build the Rust application
RUN cargo build --release

# Build the final image
FROM amazonlinux:2023

RUN dnf update -y && \
    dnf install -y \
    aws-nitro-enclaves-cli  gcc git make cmake openssl-devel\
    tar gzip shadow-utils && \
    dnf clean all

RUN git clone https://github.com/aws/aws-nitro-enclaves-cli.git && \
     cd aws-nitro-enclaves-cli && \
     mkdir build && cd build && \
     cmake .. && make && cp bin/nsm-cli /usr/local/bin/

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/enclave_attestation /enclave-main

# Entry point for the Docker container
ENTRYPOINT ["/enclave-main"]
