# draco_voice

`draco_voice` is the completely offline, API-free voice interface for Draco-OS.

## Functionality
This service continuously listens for the wake word "Draco" and processes natural language strings into actionable commands. It also handles voice authentication by verifying the user's voiceprint.

## Implementation Plan
- Integrate an always-on wake-word detection engine tailored to the word "Draco".
- Translate recorded voice commands into parsed OS and app intents (e.g., "open Minecraft", "Shutdown").
- Capture a brief sample of the user's voice for registration, storing a local speaker embedding.
- Check the speaker embedding during interactions to authenticate the user and prevent impersonation.
- Connect to `draco_ipc` to send recognized commands to `draco_shell` for visual feedback, and `draco_system` or app launchers to execute the request.
