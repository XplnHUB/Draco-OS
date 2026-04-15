use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use draco_ipc::DracoMessage;

pub struct IpcServer {
    // Simplified for now - will be expanded later
}

impl IpcServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        println!("IPC Server listening on 127.0.0.1:8080");

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            if let Err(e) = handle_client(stream) {
                                eprintln!("Error handling client: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Connection failed: {}", e);
                    }
                }
            }
        });

        Ok(())
    }
}

fn handle_client(
    mut stream: TcpStream,
) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break, // Connection closed
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Try to parse as DracoMessage
                match serde_json::from_str::<DracoMessage>(line) {
                    Ok(message) => {
                        let response = handle_message(message);
                        if let Some(resp) = response {
                            stream.write_all(resp.as_bytes())?;
                            stream.write_all(b"\n")?;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse message: {}", e);
                        let error_response = format!(r#"{{"error": "Invalid message format: {}"}}"#, e);
                        stream.write_all(error_response.as_bytes())?;
                        stream.write_all(b"\n")?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn handle_message(
    message: DracoMessage,
) -> Option<String> {
    match message {
        DracoMessage::LaunchApp(app_name) => {
            // For now, just acknowledge the launch request
            // Full integration will be added later
            Some(format!(r#"{{"success": true, "message": "Launch request received for {}", "note": "Full integration pending"}}"#, app_name))
        }
        DracoMessage::SystemStatusRequest => {
            // This would be handled by the system monitoring component
            Some(r#"{"type": "system_status_response", "cpu": 0, "ram": 0, "battery": 0}"#.to_string())
        }
        DracoMessage::VoiceCommand(command) => {
            // Handle voice commands by parsing and executing them
            let command_lower = command.to_lowercase();
            
            if command_lower.contains("launch") || command_lower.contains("open") {
                // Extract app name from command
                let words: Vec<&str> = command_lower.split_whitespace().collect();
                if let Some(app_name) = words.get(1) {
                    Some(format!(r#"{{"success": true, "message": "Voice command parsed: launch {} (pending integration)"}}"#, app_name))
                } else {
                    Some(r#"{"success": false, "error": "Could not parse app name from voice command"}"#.to_string())
                }
            } else {
                Some(format!(r#"{{"success": false, "error": "Unknown voice command: {}"}}"#, command))
            }
        }
        DracoMessage::Log(message) => {
            println!("IPC Log: {}", message);
            None
        }
        DracoMessage::Shutdown => {
            println!("Shutdown command received via IPC");
            Some(r#"{"success": true, "message": "Shutdown initiated"}"#.to_string())
        }
        _ => {
            Some(r#"{"success": false, "error": "Message type not supported by shell"}"#.to_string())
        }
    }
}
