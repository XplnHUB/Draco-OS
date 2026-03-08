# data

The `data` directory stores active, locally-generated data for Draco-OS components.

## Functionality
This directory provides a secure local path for the privacy-oriented services to store sensitive offline data without ever leaving the host machine.

## Implementation Plan
- Save captured user face templates (from `draco_face`) locally.
- Save recorded speaker embeddings/voiceprints (from `draco_voice`) locally.
- Implement optional encryption for the sensitive biometric data residing here.
- Add git-ignoring (e.g., a `.gitignore`) to ensure user data doesn't accidentally get pushed to public repositories.
