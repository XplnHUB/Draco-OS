use draco_ipc::DracoMessage;
use std::thread;
use std::time::Duration;

/// Central system core for Draco OS.
/// Mocks receiving biometric events and coordinates system state.
fn main() {
    println!("Starting draco_core daemon...");
    
    let mut is_locked = true;
    
    // In reality, this would listen on a Redox channel or Unix socket
    // and process incoming `DracoMessage` enums.
    
    // Simulated event loop:
    loop {
        // Mock incoming valid face detection after 10 seconds of being locked
        if is_locked {
            println!("draco_core [LOCKED]: waiting for face or voice...");
            thread::sleep(Duration::from_secs(10));
            
            // Received a mocked IPC message from draco_face
            let msg = DracoMessage::FaceDetected(true);
            
            match msg {
                DracoMessage::FaceDetected(true) | DracoMessage::VoiceDetected(true) => {
                    println!("draco_core: Valid biometrics received. Unlocking system.");
                    is_locked = false;
                    
                    // We'd send this IPC message to `draco_shell` here
                    let out_msg = DracoMessage::UnlockScreen;
                    println!("draco_core: Broadcast -> {:?}", out_msg);
                }
                _ => {}
            }
        } else {
             println!("draco_core [UNLOCKED]: System running normally...");
             thread::sleep(Duration::from_secs(60));
        }
    }
}
