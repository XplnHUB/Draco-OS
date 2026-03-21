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

/// Helper structures and tools to create standard IPC over Unix sockets.
pub mod channel {
    use super::DracoMessage;
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::io::{Read, Write};
    use serde_json;

    pub const SHELL_SOCKET_PATH: &str = "/tmp/draco_shell.sock";
    pub const FACE_SOCKET_PATH: &str = "/tmp/draco_face.sock";
    pub const VOICE_SOCKET_PATH: &str = "/tmp/draco_voice.sock";

    pub fn send_message(path: &str, msg: &DracoMessage) -> std::io::Result<()> {
        let mut stream = UnixStream::connect(path)?;
        let serialized = serde_json::to_string(msg).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        stream.write_all(serialized.as_bytes())?;
        stream.write_all(b"\n")?;
        Ok(())
    }

    pub fn listen<F>(path: &str, mut handler: F) -> std::io::Result<()> 
    where F: FnMut(DracoMessage) 
    {
        let _ = std::fs::remove_file(path);
        let listener = UnixListener::bind(path)?;
        
        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => {
                    let mut buffer = String::new();
                    if let Ok(_) = s.read_to_string(&mut buffer) {
                        for line in buffer.lines() {
                            if let Ok(msg) = serde_json::from_str::<DracoMessage>(line) {
                                handler(msg);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("IPC Error: {}", e),
            }
        }
        Ok(())
    }
}
