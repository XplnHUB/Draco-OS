use draco_ipc::DracoMessage;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting draco_face service...");
    println!("(Simulating webcam capture and face detection)");

    // In a real app, we'd initialize `nokhwa` here and start capturing frames.
    // We'd also listen on a socket/channel for `DracoMessage::RegisterFace`.
    
    // For now, we simulate sending a FaceDetected(true) message after 10 seconds 
    // to mock a successful login/unlock event.
    thread::sleep(Duration::from_secs(10));
    
    let msg = DracoMessage::FaceDetected(true);
    println!("Simulated detection! Sending IPC message: {:?}", msg);
    
    // Keep the process alive
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
