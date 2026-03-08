# draco_ipc

`draco_ipc` is the inter-process communication library for Draco-OS.

## Functionality
Draco-OS relies on a modular architecture where different features (voice, vision, shell, system) are isolated into separate Rust services. This crate provides the infrastructure for these services to talk to each other efficiently and securely on the local machine.

## Implementation Plan
- Implement local message passing using Unix domain sockets or shared memory channels.
- Define a structured message protocol (e.g., using Serde or Protobuf) for serializing commands, events, and data between core services.
- Provide a clear API for services to publish events (e.g., "wake word detected") and subscribe to specific event types.
- Ensure all communication stays local to the machine, enforcing the "no-cloud" and "no external API" requirements of Draco-OS.
