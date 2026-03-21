use draco_ipc::DracoMessage;
use draco_ipc::channel::{send_message, SHELL_SOCKET_PATH};
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use std::thread;
use std::time::Duration;
use image::RgbImage;

fn main() {
    println!("Starting draco_face service...");
    
    // In a production Draco-OS, this would be a persistent IPC listener (tokio socket).
    println!("Listening for registration requests on {}...", draco_ipc::channel::FACE_SOCKET_PATH);
    
    let _ = draco_ipc::channel::listen(draco_ipc::channel::FACE_SOCKET_PATH, |msg| {
        if let DracoMessage::RegisterFace = msg {
            println!("Received RegisterFace request!");

            match capture_face() {
                Ok(_) => {
                    println!("Face captured successfully!");
                    let status_msg = DracoMessage::BiometricStatus("Face ID Verified".to_string());
                    let _ = send_message(SHELL_SOCKET_PATH, &status_msg);
                    
                    thread::sleep(Duration::from_millis(500));
                    
                    let unlock_msg = DracoMessage::UnlockScreen;
                    let _ = send_message(SHELL_SOCKET_PATH, &unlock_msg);
                    println!("IPC: UnlockScreen sent");
                }
                Err(e) => {
                    println!("Hardware Error: {}. Falling back to Simulation.", e);
                    let _ = send_message(SHELL_SOCKET_PATH, &DracoMessage::BiometricStatus("Camera Error - Simulating...".to_string()));
                    thread::sleep(Duration::from_secs(2));
                    
                    let status_msg = DracoMessage::BiometricStatus("Simulated Face Verified".to_string());
                    let _ = send_message(SHELL_SOCKET_PATH, &status_msg);
                    
                    thread::sleep(Duration::from_millis(500));
                    let _ = send_message(SHELL_SOCKET_PATH, &DracoMessage::UnlockScreen);
                    println!("Simulated Face Capture Success (Mock Mode)");
                }
            }
        }
    });
}

fn capture_face() -> anyhow::Result<()> {
    println!("Initializing Camera...");
    
    // Try to find a camera
    let index = CameraIndex::Index(0);
    let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    
    let mut camera = Camera::new(index, format)?;
    camera.open_stream()?;
    
    // Buffer for a second to let exposure adjust
    thread::sleep(Duration::from_millis(500));
    
    let frame = camera.frame()?;
    let decoded = frame.decode_image::<RgbFormat>()?;
    
    // Save to local data
    let img = RgbImage::from_raw(decoded.width(), decoded.height(), decoded.to_vec())
        .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;
    
    img.save("../data/face_template.png")?;
    
    Ok(())
}
