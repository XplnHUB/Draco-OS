# draco_vision

`draco_vision` provides screen-aware visual intelligence for Draco-OS.

## Functionality
This Rust daemon is responsible for capturing on-screen context so that the OS can respond intelligently to queries like "What am I doing?" or "Is Minecraft running?" by visually analyzing desktop contents.

## Implementation Plan
- Integrate window capture and full desktop screenshot capabilities.
- Implement on-screen content analysis, utilizing local ML models (e.g., via mistral.rs) to parse active windows and content.
- Connect to `draco_ipc` to provide context-aware data back to `draco_voice` for natural language responses.
- Continuously and efficiently run background capture to identify active applications.
