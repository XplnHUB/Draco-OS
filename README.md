# Draco OS: AI-Native Operating Environment

Draco OS is a production-grade, AI-native operating environment built as a modular intelligence layer on top of Linux. It aims to transform standard computing into an intelligent, proactive system driven by local LLMs.

## 🧠 Core Vision

Unlike traditional operating systems that act as passive resource managers, Draco OS acts as an **AI Control Plane**. It bridges the gap between raw system power and intelligent intent, providing a unified brain for your computer.

## 🏗️ Architecture

Draco is built around a modular daemon-first architecture:

*   **`draco_core`**: The central async orchestration engine. Handles job queues, intent classification, and system awareness.
*   **`draco_shell`**: A high-speed CLI interface for interacting with the core engine.
*   **`draco_vision` / `draco_face`**: Real-time biometric and visual awareness layers.
*   **`draco_voice`**: Speech-to-text and intent routing via audio.

## 🚀 Key Features

*   **Async Core Daemon**: Built in Rust for maximum safety and performance.
*   **Intelligence Layer**: Seamless integration with local LLMs (via Ollama).
*   **System Awareness**: Real-time monitoring of CPU, MEM, and system services.
*   **Secure Execution**: Sandbox-driven command execution with policy enforcement.
*   **Contextual Memory**: Short-term and long-term memory systems for personalized automation.

## 🔧 Getting Started

### Prerequisites

*   Linux (Ubuntu/Debian recommended)
*   Rust (Nightly toolchain)
*   Ollama (running locally)

### Build and Run

1.  **Build the Core Daemon**:
    ```bash
    cd draco/draco_core
    cargo build --release
    ```
2.  **Start the Daemon**:
    ```bash
    ./target/release/draco_core
    ```
3.  **Interact via Shell**:
    ```bash
    cd ../draco_shell
    cargo run
    ```

## 📜 Documentation

*   [Draco Core Daemon Guide](docs/draco_daemon_guide.md)
*   [Feature Roadmap](features.md)

## 🔐 Security & Privacy

Draco OS is designed with a **Local-First** philosophy. All AI processing and system automation data stay on your machine.

---

*Draco OS is currently in active development.*