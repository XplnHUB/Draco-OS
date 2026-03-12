# Draco-OS Development TODO

## Phase 1: Core OS Foundation & Build System
- [x] Set up Redox OS base environment and build system.
- [x] Implement custom OS branding (Draco-OS boot screen, system name).
- [x] Establish basic user identity and auto-login structures.
- [x] Architect the IPC messaging system for Rust-based microservices.
- [x] Implement the Draco-OS kernel.

## Phase 2: Desktop GUI & Shell
- [x] Develop the Draco desktop shell application (Rust).
- [x] Implement a clean, modern status bar (battery, network, usage stats).
- [x] Build the app grid and launcher UI.
- [x] Add aesthetic polish (rounded corners, shadows, animations, Pop-style theme).

## Phase 3: Identity & Biometric Security (Local)
- [x] Integrate local webcam capture for face registration.
- [x] Implement local face authentication service (`Draco-face`).
- [x] Develop voiceprint registration module.
- [x] Implement local voice authentication service (`Draco-voice`).
- [x] Build the secure lock/unlock system tying into the biometric services.

## Phase 4: Voice Interface & Command Engine
- [ ] Integrate always-listening wake-word detection ("Draco").
- [ ] Build the natural language command parser.
- [ ] Implement system control commands (shutdown, reboot, sleep, status queries).
- [ ] Implement app launcher commands (Minecraft, Firefox, Code).
- [ ] Implement file-system action commands (open, search).+--

## Phase 5: Screen-Aware Intelligence
- [ ] Develop the screenshot capture service (`Draco-vision`).
- [ ] Implement on-screen content analysis (detect active windows/apps).
- [ ] Enable context-aware voice responses based on screen state.

## Phase 6: System Optimization & Final Polish
- [ ] Implement idle auto-suspend and resource optimization logic.
- [ ] Add sound effect feedback for voice interactions.
- [ ] (Optional) Add local Text-to-Speech (TTS) for voice feedback.
- [ ] Perform final system integration and performance testing.
