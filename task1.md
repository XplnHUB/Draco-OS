# Draco-OS Task: Implement Voice Command Pipeline (MVP)

## Objective

Build a complete end-to-end voice command pipeline that allows users to control the OS using natural speech.

The system should:

* Listen for voice input
* Convert speech to text
* Parse intent from text
* Execute system-level actions via IPC or direct control
* Provide optional feedback

This is the core interaction layer of Draco-OS and must be reliable, fast, and modular.

---

## Scope (MVP Only — No Overengineering)

Implement the following pipeline:

wake word (optional placeholder) → speech-to-text → intent parser → command executor

---

## Requirements

### 1. Voice Input (Speech-to-Text)

* Use whisper-rs for transcription
* Accept microphone input OR pre-recorded audio (for testing)
* Output clean text string

Example:
Input: "open firefox"
Output: "open firefox"

---

### 2. Intent Parser (Deterministic)

* Do NOT use heavy AI models initially
* Use keyword matching / regex-based parsing

Define enum:

enum Intent {
OpenApp(String),
CloseActiveApp,
GetSystemInfo(String),
Shutdown,
Unknown
}

Map examples:

* "open firefox" → OpenApp("firefox")
* "close this" → CloseActiveApp
* "check ram" → GetSystemInfo("ram")
* "shutdown system" → Shutdown

---

### 3. Command Execution Layer

Implement executor:

fn execute_intent(intent: Intent) {
match intent {
Intent::OpenApp(app) => launch_app(app),
Intent::CloseActiveApp => close_active_window(),
Intent::GetSystemInfo(info) => fetch_system_info(info),
Intent::Shutdown => shutdown_system(),
Intent::Unknown => log_error("Unknown command"),
}
}

Constraints:

* Use existing Application Control Layer where possible
* Avoid shell hacks — use system-level APIs or IPC

---

### 4. IPC Integration (If Available)

* Send structured messages via existing IPC system
* Format messages as JSON

Example:
{
"type": "LaunchApp",
"payload": "firefox"
}

---

### 5. Feedback System (Basic)

* Print logs OR simple audio feedback

Example:

* "Opening Firefox"
* "Closing active window"

---

## Architecture

Modules to create:

* draco_voice (handles audio + STT)
* draco_intent (parsing logic)
* draco_executor (executes commands)
* draco_ipc (existing, integrate here)

Ensure modular separation.

---

## Deliverables

* Working Rust modules for:

  * voice input
  * intent parsing
  * execution layer

* CLI test command:
  cargo run -- "open firefox"

* End-to-end working flow:
  Voice → Text → Intent → Action

---

## Success Criteria

* At least 5 commands working:

  * open firefox
  * open terminal
  * close this
  * check ram
  * shutdown

* Response time < 1 second (excluding STT)

* Clean modular code (extensible for AI later)

---

## Constraints

* Keep it simple
* No heavy ML inference beyond whisper
* No UI needed
* Focus on functionality over perfection

---

## Bonus (Optional)

* Add logging for debugging
* Add command history
* Add fallback for unknown commands

---

## Goal

By completing this task, Draco-OS should support basic voice-driven system control — marking the transition from a traditional OS to an AI-native interface.

