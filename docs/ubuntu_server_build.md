# Building Draco-OS on Ubuntu Server

This guide provides step-by-step instructions for building Draco-OS on a clean Ubuntu Server (20.04, 22.04, or 24.04).

## 1. System Requirements

*   **OS**: Ubuntu Server (64-bit)
*   **RAM**: Minimum 8GB (16GB recommended for faster compilation)
*   **Disk**: At least 50GB of free space
*   **Internet**: Required for downloading dependencies and Rust toolchains

## 2. Install Base Dependencies

Update your system and install the essential build tools:

```bash
sudo apt update
sudo apt install -y build-essential git curl clang llvm nasm xorriso grub-pc-bin qemu-system-x86

# Install 'just' (command runner) - Required for COSMIC components
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | sudo bash -s -- --to /usr/local/bin
```

## 3. Install Rust (Nightly)

Draco-OS requires the Rust nightly toolchain and specific components.

```bash
# Install rustup if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Setup Nightly and Components
rustup default nightly
rustup component add rust-src rustfmt clippy
rustup target add x86_64-unknown-redox
```

## 4. Clone and Bootstrap

Clone the repository and run the Redox bootstrap script to install cross-compilation toolchains and additional libraries.

```bash
git clone https://github.com/XplnHUB/Draco-OS.git
cd Draco-OS

# Run the native bootstrap script (This will take some time and might prompt for password)
./redox/native_bootstrap.sh -e qemu
```

> [!NOTE]
> The `-e qemu` flag ensures that QEMU and its related dependencies are configured.

## 5. Build Draco-OS

Once the environment is set up, you can build the entire system.

```bash
# Build Draco AI services and the Redox kernel
make all
```

## 6. Running Draco-OS on a Server

Since a server typically lacks a physical display, you can run Draco-OS in QEMU with VNC access or in non-graphic mode.

### Option A: VNC Access (Recommended for GUI testing)
```bash
# Launch with VNC on port 5900 (display :0)
cd redox
make qemu QEMUFLAGS="-vnc :0"
```
You can then connect using any VNC client to `<your-server-ip>:5900`.

### Option B: Non-Graphic (Serial Console)
If you only need terminal access and want to skip the GUI:
```bash
cd redox
make qemu QEMUFLAGS="-nographic"
```

## Troubleshooting

- **Memory Issues**: If the build fails due to OOM, increase swap space or use `cargo build -j 2` to limit parallel jobs.
- **Missing Toolchains**: Ensure `source "$HOME/.cargo/env"` was run or restart your shell.
- **FUSE Errors**: Building the filesystem image requires FUSE. Ensure the `fuse` or `fuse3` package is installed and your user is part of the `fuse` group if necessary.
