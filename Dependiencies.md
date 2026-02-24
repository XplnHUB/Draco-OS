# Start Building This on Linux

## Install Required System Dependencies

```bash
sudo apt update && sudo apt install -y \
  build-essential \
  git \
  curl \
  clang \
  llvm \
  nasm \
  xorriso \
  grub-pc-bin \
  qemu-system-x86
```

---

## Install Rust (Nightly + Required Components)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

```bash
source ~/.cargo/env
```

```bash
rustup default nightly
```

```bash
rustup component add rust-src rustfmt clippy
```

```bash
rustup target add x86_64-unknown-redox
```

---
