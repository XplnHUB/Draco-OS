# draco_system

`draco_system` handles low-level operating system actions and resource monitoring for Draco-OS.

## Functionality
This Rust daemon is responsible for managing system states, monitoring energy and performance metrics, and acting upon commands received (e.g., from `draco_voice` or `draco_shell`).

## Implementation Plan
- Implement functionality to trigger system power states: shutdown, reboot, sleep, and hibernate.
- Collect resource usage metrics: battery percentage, CPU load, and RAM consumption.
- Provide data over IPC to `draco_shell` for rendering in the status bar.
- Manage auto-suspension during idle periods to save battery.
- Respond to system queries like "What's my battery?" via IPC integration with the voice engine.
