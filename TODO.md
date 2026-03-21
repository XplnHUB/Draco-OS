# Draco-OS: Pro-Level Development Roadmap (TODO)

This roadmap outlines the technical engineering steps required to transition Draco-OS from its experimental state to the pro-level vision defined in `complete.md`.

---

## Phase 1: Core OS & Build Infrastructure (Foundational)
- [x] **Redox Base Stability:** Ensure Redox kernel boots in QEMU and native hardware.
- [x] **Custom Branding:** Inject "Draco-OS" into `bootloader`, `init`, and `kernel` logs.
- [x] **Identity System:** Implement basic user creation and auto-login logic.
- [x] **IPC Foundation:** Establish the `unix-socket` or `redox-channel` based messaging protocol between daemons.

## Phase 2: Draco Shell & GUI (The Desktop)
- [x] **Orbital Integration:** Build `draco-shell` using Rust (Slint or custom toolkit).
- [x] **Teal/Orange Theme:** Implement the Pop-style aesthetic (rounded corners, shadows).
- [x] **Status Bar:** Real-time metrics for CPU, RAM, Battery, and AI Daemon status.
- [x] **App Launcher:** Implement grid-based icon navigation and window management.

## Phase 3: Local Biometric Security
- [x] **Face Registration:** Implement `draco-face` for secure, local face-point capture.
- [x] **Face Auth:** Hook local OpenCV/inferencing into the login/unlock flow.
- [x] **Voice Registration:** Implement local voiceprint embedding capture.
- [x] **Voice Auth:** Create the `draco-voice` authentication server.

---

## Phase 4: Voice Interface & NLP Engine (In Progress)
- [ ] **Wake-Word Detection:**
  - [ ] Implement `draco-voice` listener using a lightweight Rust-native model (e.g., `porcupine` alternative or `rust-nn`).
  - [ ] Optimize for sub-500ms latency on `x86_64`.
- [ ] **Speech-to-Text (Local STT):**
  - [ ] Integrate `whisper-rs` for locally-hosted transcription.
  - [ ] Implement buffer-streaming for real-time interaction.
- [ ] **NLP Command Parser:**
  - [ ] Map transcriptions to specific system intents (e.g., `"open {app}"`, `"shutdown"`, `"search {query}"`).
  - [ ] Implement fuzzy matching for command robustness.
- [ ] **IPC Action Routing:**
  - [ ] Route intents from `draco-voice` -> `draco_core`.
  - [ ] Implement callback loops for command confirmation.

## Phase 5: Vision & Contextual Awareness
- [ ] **Frame Recovery (`draco-vision`):**
  - [ ] Implement secure framebuffer capture for specific window handles or global desktop.
- [ ] **Vision Inference:**
  - [ ] Integrate `mistral.rs` (Vision model) for local analysis of captured screen data.
  - [ ] Implement "Context Queries" (e.g., "What game is this?").
- [ ] **System State Tracking:**
  - [ ] Track active window titles and PIDs to provide context to the NLP engine.

## Phase 6: System Optimization & Power Management
- [ ] **Idle Auto-Suspend:**
  - [ ] Implement system-wide timers to trigger `sleep`/`hibernate` after inactivity.
  - [ ] Integrate `draco-face` to detect user presence before suspending.
- [ ] **Neural Throttle:**
  - [ ] Dynamically lower the polling rate of `draco-voice` and `draco-face` during battery-low states.
  - [ ] Ensure AI daemons yield priority when high-performance apps (e.g., Minecraft) are focused.
- [ ] **Local TTS:**
  - [ ] Integrate a lightweight Rust-based TTS (Text-to-Speech) for audible system feedback.

## Phase 7: App Ecosystem & Integration
- [ ] **Native App Hooks:**
  - [ ] Create `draco-api` for applications (Minecraft, Firefox) to register custom voice commands.
  - [ ] Implement generic window-controls (resize, move, focus) via voice.
- [ ] **Developer SDK:**
  - [ ] Document the IPC protocol for 3rd-party microservice plugins.
  - [ ] Provide boilerplate Rust templates for "Draco-Ready" applications.

## Phase 8: Final QA & Hardening
- [ ] **Privacy Audit:** Verify 100% zero-network-egress for all AI components.
- [ ] **Performance Benchmarking:** Target sub-10% CPU usage for background idling.
- [ ] **Regression Testing:** Ensure Redox kernel updates do not break Draco microservices.
