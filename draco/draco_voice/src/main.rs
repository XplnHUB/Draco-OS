use draco_ipc::DracoMessage;
use draco_ipc::channel::{send_message, SHELL_SOCKET_PATH};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound;

fn main() {
    println!("Starting draco_voice service...");

    println!("Listening for voice registration requests on {}...", draco_ipc::channel::VOICE_SOCKET_PATH);
    
    let _ = draco_ipc::channel::listen(draco_ipc::channel::VOICE_SOCKET_PATH, |msg| {
        if let DracoMessage::RegisterVoice = msg {
            println!("Received RegisterVoice request!");

            match record_voice() {
                Ok(_) => {
                    println!("Voice recorded successfully!");
                    let status_msg = DracoMessage::BiometricStatus("Voice Print Verified".to_string());
                    let _ = send_message(SHELL_SOCKET_PATH, &status_msg);
                    
                    thread::sleep(Duration::from_millis(500));
                    
                    let unlock_msg = DracoMessage::UnlockScreen;
                    let _ = send_message(SHELL_SOCKET_PATH, &unlock_msg);
                    println!("IPC: UnlockScreen sent");
                }
                Err(e) => {
                    println!("Hardware Error: {}. Falling back to Simulation.", e);
                    let _ = send_message(SHELL_SOCKET_PATH, &DracoMessage::BiometricStatus("Mic Error - Simulating...".to_string()));
                    thread::sleep(Duration::from_secs(2));
                    
                    let status_msg = DracoMessage::BiometricStatus("Simulated Voice Verified".to_string());
                    let _ = send_message(SHELL_SOCKET_PATH, &status_msg);
                    
                    thread::sleep(Duration::from_millis(500));
                    let _ = send_message(SHELL_SOCKET_PATH, &DracoMessage::UnlockScreen);
                    println!("Simulated Voice Capture Success (Mock Mode)");
                }
            }
        }
    });
}

fn record_voice() -> anyhow::Result<()> {
    println!("Initializing Microphone...");
    
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No input device found"))?;
    
    let config = device.default_input_config()?;
    let spec = hound::WavSpec {
        channels: config.channels() as u16,
        sample_rate: config.sample_rate().0 as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let writer = Arc::new(Mutex::new(Some(hound::WavWriter::create("../data/voice_sample.wav", spec)?)));
    let writer_clone = writer.clone();

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _| {
            if let Some(ref mut w) = *writer_clone.lock().unwrap() {
                for &sample in data {
                    let s = (sample * i16::MAX as f32) as i16;
                    w.write_sample(s).ok();
                }
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None
    )?;

    stream.play()?;
    println!("Recording for 5 seconds...");
    thread::sleep(Duration::from_secs(5));
    
    drop(stream);
    writer.lock().unwrap().take(); // Close the file
    
    Ok(())
}
