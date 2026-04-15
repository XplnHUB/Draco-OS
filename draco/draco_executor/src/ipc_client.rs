use draco_ipc::DracoMessage;
use anyhow::{Result, Context};
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct IpcClient {
    addr: String,
}

impl IpcClient {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }

    pub fn send_launch_app(&self, app_name: &str) -> Result<String> {
        let msg = DracoMessage::LaunchApp(app_name.to_string());
        self.send_message(&msg)
    }

    pub fn send_voice_command(&self, command: &str) -> Result<String> {
        let msg = DracoMessage::VoiceCommand(command.to_string());
        self.send_message(&msg)
    }

    pub fn send_system_status_request(&self) -> Result<String> {
        let msg = DracoMessage::SystemStatusRequest;
        self.send_message(&msg)
    }

    pub fn send_shutdown(&self) -> Result<String> {
        let msg = DracoMessage::Shutdown;
        self.send_message(&msg)
    }

    fn send_message(&self, msg: &DracoMessage) -> Result<String> {
        let json = serde_json::to_string(msg)
            .context("Failed to serialize IPC message")?;

        let mut stream = TcpStream::connect(&self.addr)
            .context(format!("Failed to connect to IPC server at {}", self.addr))?;

        stream.write_all(json.as_bytes())
            .context("Failed to send IPC message")?;
        stream.shutdown(std::net::Shutdown::Write)?;

        let mut response = String::new();
        stream.read_to_string(&mut response)
            .context("Failed to read IPC response")?;

        Ok(response)
    }
}
