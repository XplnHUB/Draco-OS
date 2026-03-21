# Draco-OS System Prompt & Context

## Project Overview
**Draco-OS** is a next-generation, AI-first microkernel operating system. Built as a sophisticated fork of [Redox OS](https://redox-os.org), it integrates advanced AI capabilities directly into the system architecture to enable a seamless, voice-dictated, and vision-aware computing experience.

## Core Philosophy
1. **Voice-Native:** Controlled primarily through natural language (wake-word: "Draco").
2. **Context-Aware:** The system utilizes screen and camera vision to assist the user.
3. **Privacy-First:** **100% Local AI processing.** No cloud API calls, no third parties. (Uses `whisper-rs`, `mistral.rs`, and local facial recognition).
4. **Rust First:** Built on the Redox microkernel, ensuring strict memory safety and modularity. All core daemons are written in Rust.

## Technology Stack
- **Kernel / Base:** Redox Microkernel (Rust), Relibc
- **Language:** Rust (Stable/Nightly depending on component)
- **GUI Desktop:** `draco_shell` (Pop-OS inspired teal/orange theme, floating status bar)
- **Communication:** Local IPC (Channels/Sockets) between system daemons.
- **AI/ML:** Rust bindings for Whisper (`whisper-rs`), Mistral (`mistral.rs`), local OpenCV/burn/tract.

## System Architecture (Microservices)
Draco-OS is composed of modular Rust daemons that communicate over IPC:
- `draco_core`: Manages system state, user identity, power states, and forwards actions.
- `draco_shell`: The GUI desktop environment, status bar, and app launcher.
- `draco_voice`: Always-listening microphone daemon, wake-word detection, NLP parsing, and local voice authentication.
- `draco_face`: Camera access and face detection/authentication.
- `draco_vision`: Screen capture and visual context analyzer.

## Guidelines for AI Coding Assistants (Claude, Cursor, Copilot, etc.)
When generating code or proposing architectural changes for Draco-OS, strictly adhere to the following rules:

1. **Rust Best Practices:** Write idiomatic, memory-safe Rust code. Handle errors gracefully using `Result` and `Option`. Never use `unwrap()` in production daemons unless failure is genuinely unrecoverable.
2. **Redox / Unix Compatibility:** Be mindful that this is a Unix-like Redox OS environment. Avoid Linux-specific APIs (e.g., specific `ioctl`s or `/proc` interfaces) unless supported by Redox/Relibc.
3. **IPC First:** Daemons should be completely decoupled. If `draco_voice` needs to update the UI, it sends an IPC message to `draco_shell`.
4. **No Cloud:** Never suggest importing SDKs or crates that rely on external cloud APIs (e.g., OpenAI, AWS, Google Cloud). All AI features must run on local inferences.
5. **Performance:** The AI daemons run in the background. Keep CPU and RAM footprints minimal. Optimize tensor operations and utilize idle auto-suspend architectures.
6. **Aesthetics:** When working on `draco_shell`, ensure the UI is modern, uses smooth animations, rounded corners, and adheres to the teal/orange aesthetic scheme.

---
*Note: This file is intended to provide system-level context to LLMs and automation tools interacting with the Draco-OS codebase. Point your AI assistant to this file to ensure it aligns with the project's architecture and constraints.*
