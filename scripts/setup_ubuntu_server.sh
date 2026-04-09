#!/usr/bin/env bash

# Draco-OS Ubuntu Server Setup Script
# This script automates the installation of dependencies, Rust, and the build system.

set -e

echo "--- Starting Draco-OS Setup for Ubuntu Server ---"

# 1. Update and install base dependencies
echo "Installing base system dependencies..."
sudo apt update
sudo apt install -y build-essential git curl clang llvm nasm xorriso grub-pc-bin qemu-system-x86 fuse3 libfuse3-dev

# 2. Install Rust Nightly
if ! command -v rustup &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust already installed, updating..."
    rustup update
fi

echo "Setting up Rust nightly toolchain..."
rustup default nightly
rustup component add rust-src rustfmt clippy
rustup target add x86_64-unknown-redox

# 3. Install 'just' via binary installer (to avoid nix compilation issues)
if ! command -v just &> /dev/null; then
    echo "Installing 'just' command runner..."
    mkdir -p "$HOME/.local/bin"
    curl --proto '=https' --tlsv1.2 -sSf https://just.sh/install.sh | bash -s -- --to "$HOME/.local/bin"
    
    # Add to current PATH and profile
    export PATH="$PATH:$HOME/.local/bin"
    if ! grep -q ".local/bin" "$HOME/.bashrc"; then
        echo 'export PATH="$PATH:$HOME/.local/bin"' >> "$HOME/.bashrc"
    fi
else
    echo "'just' is already installed."
fi

# 4. Bootstrap Redox Build System
if [ -d "redox" ]; then
    echo "Bootstrapping Redox build system..."
    ./redox/native_bootstrap.sh -e qemu
else
    echo "Error: Directory 'redox' not found. Please run this script from the root of the Draco-OS repository."
    exit 1
fi

echo "--- Setup Complete! ---"
echo "You can now build Draco-OS by running: make all"
echo "Note: You might need to restart your shell or run 'source ~/.bashrc' to pick up the new PATH."
