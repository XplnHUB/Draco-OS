# config

The `config` directory stores the default configuration files for Draco-OS.

## Functionality
This directory contains user-configurable `.toml`, `.json`, or `.yaml` files that define settings for the various services (e.g., UI theme colors, custom voice commands, resource thresholds).

## Implementation Plan
- Create a central `draco_config.toml` that `draco_core` parses.
- Implement optional encryption for sensitive configuration data.
- Allow users to drop new configurations or plugin settings into this folder.
