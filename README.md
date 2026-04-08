# Draco-OS



**Draco-OS** is a next-generation, AI-first microkernel operating system. Built as a sophisticated fork of [Redox OS](https://redox-os.org), Draco-OS integrates advanced AI capabilities directly into the system architecture, enabling a seamless, voice-dictated, and vision-aware computing experience.

> [!IMPORTANT]
> Draco-OS is currently in **early experimental stage**. It is designed for researchers, AI enthusiasts, and system developers who want to explore the frontier of AI-native operating systems.

---

## Core Philosophy

Traditional operating systems treat AI as an application. Draco-OS treats AI as a **core system service**. 

*   **Voice-Native**: Controlled primarily through natural language. No more typing complex shell commands.
*   **Context-Aware**: The system "sees" what you see, providing real-time code optimizations and workflow assistance.
*   **Privacy-First**: All AI processing (STT, Vision, LLM) runs **locally** on your hardware using `whisper-rs` and `mistral.rs`. No cloud API calls, no data leaks.
*   **Microkernel Security**: Inherits the memory safety and modularity of the Redox microkernel, written entirely in Rust.

---

## Key Features

- **Zero-Touch Interface**: Wake-word detection ("Draco") with ultra-low latency (<500ms).
- **Visual Intelligence**: Real-time framebuffer analysis for context-aware developer assistance.
- **Pure Rust Stack**: From the kernel to the AI daemons, the entire system leverages Rust's safety guarantees.
- **Modular Services**: AI capabilities are isolated in user-space daemons, ensuring kernel stability.
- **High Performance**: Optimized for modern x86_64 architecture with planned ARM support.

---

## System Architecture

```mermaid
graph TD
    User((User)) -->|Voice/Vision| AI_Layer[Draco AI Services]
    AI_Layer -->|Inter-Process Communication| System_Daemons[Redox System Daemons]
    System_Daemons -->|Syscalls| Microkernel[Redox Microkernel]
    Microkernel -->|Drivers| Hardware[CPU/GPU/Audio/Mic]
    
    subgraph "Draco AI Layer"
        V_Daemon[draco_voice: Whisper STT]
        S_Daemon[draco_vision: Mistral.rs Vision]
    end
```

### Technical Stack
*   **Kernel**: Redox Microkernel (MIT)
*   **Filesystem**: RedoxFS
*   **GUI**: Orbital Window Manager
*   **Shell**: Ion Shell (enhanced with `draco://` protocol)
*   **AI Engine**: `whisper-rs` (Voice) & `mistral.rs` (Vision)

---

## Screenshots

<p align="center">
  <img src="assets/screen_login.png" alt="Draco-OS Login" width="45%">
  &nbsp;&nbsp;
  <img src="assets/screen_desktop.png" alt="Draco-OS Desktop UI" width="45%">
</p>

---

## Quick Start

### 1. Prerequisites (Ubuntu/Debian/Pop!_OS)
Ensure you have the necessary build tools and Rust nightly environment:

```bash
sudo apt update && sudo apt install -y \
  build-essential git curl clang llvm nasm xorriso grub-pc-bin qemu-system-x86

# Setup Rust Nightly
rustup default nightly
rustup component add rust-src rustfmt clippy
rustup target add x86_64-unknown-redox
```

### 1.5 Docker Development (Recommended for Mac/Windows or non-sudo Users)
If you are on macOS, Windows, or do not have `sudo` access on your server, use Docker to build the project.

```bash
# Build the dev environment
docker compose build

# Enter the container
docker compose run --rm draco-os bash

# Inside the container, you can run:
make all
```

### 1.6 Publishing to Docker Hub
To use this image on other servers (like your Ubuntu VPS), you can publish it to Docker Hub:

1. **Log in to Docker Hub**:
   ```bash
   docker login
   ```

2. **Tag the image**:
   ```bash
   # Using your username codemaverick143
   docker tag draco-os-draco-os:latest codemaverick143/draco-os:latest
   ```

3. **Push the image**:
   ```bash
   docker push codemaverick143/draco-os:latest
   ```

4. **Pull on the server**:
   ```bash
   # On your Ubuntu server
   docker pull codemaverick143/draco-os:latest
   ```

### 2. Build and Boot
Clone the repository and launch the system in QEMU:

```bash
git clone https://github.com/XplnHUB/Draco-OS.git
cd draco-os
make qemu
```

Once booted, type `draco_voice &` in the terminal to initialize the AI listener.

---

## Development Roadmap

| Phase | Milestone | Description | Status |
| :--- | :--- | :--- | :--- |
| Phase 1 | Foundational Voice | Wake-word detection + basic STT command execution. | Done |
| Phase 2 | Custom Desktop Environment | Developed `draco_shell` with system status and teal UI accents. | Done |
| Phase 3 | Contextual Vision | Framebuffer capture + Mistral.rs vision integration. | In Progress |
| Phase 4 | Neural Optimization | Real-time code refactoring agents within the IDE. | Planned |
| Phase 5 | Hardware Expansion | Native support for Raspberry Pi 5 and ARM64. | Planned |

---

## Contributing

We welcome contributions from the community! Whether it's improving the kernel, optimizing AI models, or enhancing hardware support.

1.  **Fork** the repository.
2.  **Create** your feature branch (`git checkout -b feature/amazing-feature`).
3.  **Commit** your changes (`git commit -m 'Add amazing feature'`).
4.  **Push** to the branch (`git push origin feature/amazing-feature`).
5.  **Open** a Pull Request.

---

## License & Credits

Distributed under the MIT License. See `LICENSE` for more information.

*   **Redox OS**: Special thanks to the [Redox OS team](https://redox-os.org) for the incredible foundation.
*   **AI Libraries**: Powered by `whisper-rs` and `mistral.rs`.

---

<p align="center">
  Built with heart by Arpit Sarang and the Draco-OS Community.
  <br>
  <i>"Say 'Draco' to wake the future of operating systems."</i>
</p>