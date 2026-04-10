# Draco OS Roadmap

This roadmap outlines the development phases for Draco OS, an AI-native operating environment.

## Phase 1: Foundation (Core & IPC)
*Goal: Establish a stable async backbone and communication layer.*

- [x] **Async Core Daemon (`draco_core`)**: Main event loop and job queue.
- [x] **Modular Architecture**: Clean separation of core, brain, system, and ipc.
- [x] **IPC Layer**: Axum-based HTTP server for internal messaging.
- [x] **CLI Shell (`draco_shell`)**: Basic REPL to interact with the core.
- [ ] **Logging System**: Structured logs for request/response tracking.

## Phase 2: Intelligence & System Awareness
*Goal: Make the daemon aware of its environment and capable of basic intent reflection.*

- [ ] **System Metrics**: Real-time CPU, Memory, and Disk monitoring.
- [ ] **Service Monitoring**: Status tracking for systemctl and Docker.
- [ ] **Intent Engine (V1)**: LLM-based classification (Ollama integration).
- [ ] **Context Builder**: Formatter for system state to be injected into prompts.
- [ ] **System State API**: Endpoints to retrieve real-time system health.

## Phase 3: Execution & Security
*Goal: Allow Draco to safely execute system commands.*

- [ ] **Command Parsing**: Extraction of executable commands from AI responses.
- [ ] **Execution Sandbox**: Secure subprocess wrapper for running commands.
- [ ] **Safety Policies**: Whitelist-based enforcement and argument validation.
- [ ] **Confirmation Flow**: Interactive approval for destructive actions.
- [ ] **Execution Logging**: Detailed audit trails for every system change.

## Phase 4: Memory & Persistence
*Goal: Give Draco a long-term memory and contextual awareness of past interactions.*

- [ ] **Short-Term Context**: Management of active conversation windows.
- [ ] **Long-Term Store**: Persistent storage (JSON/SQLite) for user preferences.
- [ ] **Workspace Registry**: Awareness of projects and directories.
- [ ] **Retriever**: Semantic search for past interactions (optional vector DB).

## Phase 5: Advanced Interfaces & Automation
*Goal: Expand reach via TUI, GUI, and Voice.*

- [ ] **TUI Dashboard**: Real-time terminal interface for metrics and chat.
- [ ] **GUI Interface (`draco_face`)**: Desktop control panel for non-terminal users.
- [ ] **Voice Layer (`draco_voice`)**: Wake-word detection and speech synthesis.
- [ ] **Task Automation**: Cron-like execution of AI-driven maintenance tasks.

## Phase 6: Distribution & Ecosystem
*Goal: Ship Draco as a complete product.*

- [ ] **Packaging**: Creation of `.deb` packages for easy installation.
- [ ] **Systemd Integration**: Full service lifecycle management.
- [ ] **Custom Distribution**: Building a Draco-preinstalled ISO (Ubuntu base).
- [ ] **Plugin System**: Allowing external extensions to the Draco brain.

---

*Last Updated: April 2026*
