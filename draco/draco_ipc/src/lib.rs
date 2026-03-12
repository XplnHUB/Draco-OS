use serde::{Serialize, Deserialize};

/// Core messages that Draco-OS services send back and forth over local IPC.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DracoMessage {
    /// A voice command has been parsed and is ready to execute (sent by draco_voice)
    VoiceCommand(String),
    
    /// User facial recognition authentication status (sent by draco_face to draco_core)
    FaceDetected(bool),

    /// User voice recognition authentication status (sent by draco_voice to draco_core)
    VoiceDetected(bool),

    /// Request to register a new face profile (sent by shell/core to draco_face)
    RegisterFace,

    /// Request to register a new voice profile (sent by shell/core to draco_voice)
    RegisterVoice,

    /// Status of a biometric registration process
    BiometricStatus(String),
    
    /// Command to lock the screen UI (sent by core to shell)
    LockScreen,

    /// Command to unlock the screen UI (sent by core to shell)
    UnlockScreen,
    
    /// Request basic system analytics (sent by draco_shell or others to draco_core)
    SystemStatusRequest,
    
    /// Response to SystemStatusRequest containing system analytics
    SystemStatusResponse {
        cpu_percent: u8,
        ram_percent: u8,
        battery_percent: u8,
    },
    
    /// Request to launch a graphical application (handled by draco_shell)
    LaunchApp(String),
    
    /// Informational or error logs
    Log(String),
    
    /// Stop/shutdown signal
    Shutdown,
}

/// Helper structures and tools to create standard IPC over Unix sockets or TCP/Redox channels.
pub mod channel {
    // This is a placeholder for async networking logic. In Redox, standard sockets and chan:
    // are common, but tokio is available. We will refine this once we begin the actual services.
}
