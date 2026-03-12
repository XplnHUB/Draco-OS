use draco_ipc::DracoMessage;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting draco_voice service...");
    println!("(Simulating microphone capture and voice detection)");

    // In a real app, we'd initialize `cpal` here and start listening for audio chunks.
    // We'd pass audio through whisper-rs or a lighter embedding model.
    // We'd also listen on a socket/channel for `DracoMessage::RegisterVoice`.
    
    // For now, we simulate sending a VoiceDetected(true) message after 15 seconds 
    // to mock a successful login/unlock event via voice.
    thread::sleep(Duration::from_secs(15));
    
    let msg = DracoMessage::VoiceDetected(true);
    println!("Simulated voice detection! Sending IPC message: {:?}", msg);
    
    // Keep the process alive
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
