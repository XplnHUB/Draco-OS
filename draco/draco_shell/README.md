# draco_shell

`draco_shell` is the unified desktop shell and GUI environment for Draco-OS.

## Functionality
This is the primary user interface component of the operating system, providing a visual desktop shell built in Rust. It interacts with the other background daemon services (voice, vision, system) via IPC to surface details like battery, system resources, face lock status, and voice recognition state.

## Implementation Plan
- Develop a full, customizable desktop shell featuring a status bar (battery, network, voice/face icons) and an application grid.
- Integrate modern UI conventions such as smooth window transitions, rounded corners, blur effects, and custom theming.
- Act as the central visual state manager. When an app is opened via a voice command, `draco_shell` is responsible for launching the GUI components or directing window management.
- Expose visual feedback (like a short animation or sound) when Draco's wake word is recognized or a command is running.
