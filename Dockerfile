# Force x86_64 for compatibility with kernel build tools
FROM --platform=linux/amd64 ubuntu:22.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    clang \
    llvm \
    nasm \
    xorriso \
    grub-pc-bin \
    qemu-system-x86 \
    pkg-config \
    libssl-dev \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Install Rustup and the Nightly toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install required Rust components and target for Redox
RUN rustup component add rust-src rustfmt clippy && \
    rustup target add x86_64-unknown-redox

# Create a working directory
WORKDIR /workspace

# Default command
CMD ["/bin/bash"]
