# draco_face

`draco_face` is the facial recognition and authentication service for Draco-OS.

## Functionality
This service continuously monitors the system's webcam (or other camera) to provide secure face authentication and presence detection. 

## Implementation Plan
- Construct a Rust-based, 100% local facial recognition pipeline.
- Implement face registration: capture a photo and generate a locally-stored face template.
- Implement face authentication: authenticate the user at boot or screen unlock without sending any data to a cloud API.
- Continuously check user presence to lock the screen automatically if the user walks away.
- Only allow full access (voice commands, apps, settings) if the registered user's face is detected or if manually unlocked via another secure method.
