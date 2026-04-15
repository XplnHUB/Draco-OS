use draco_intent::Intent;
use crate::ipc_client::IpcClient;
use crate::system_info::SystemInfoProvider;
use tracing::{info, warn, error};

#[derive(Debug)]
pub enum ExecutionResult {
    Success(String),
    Failed(String),
    NotImplemented(String),
}

impl std::fmt::Display for ExecutionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionResult::Success(msg) => write!(f, "✅ {}", msg),
            ExecutionResult::Failed(msg) => write!(f, "❌ {}", msg),
            ExecutionResult::NotImplemented(msg) => write!(f, "⚠️ {}", msg),
        }
    }
}

pub struct CommandExecutor {
    ipc: IpcClient,
    sys_info: SystemInfoProvider,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            ipc: IpcClient::new("127.0.0.1:8080"),
            sys_info: SystemInfoProvider::new(),
        }
    }

    pub fn with_ipc_addr(addr: &str) -> Self {
        Self {
            ipc: IpcClient::new(addr),
            sys_info: SystemInfoProvider::new(),
        }
    }

    pub fn execute(&self, intent: Intent) -> ExecutionResult {
        info!("Executing intent: {}", intent);
        match intent {
            Intent::OpenApp(app) => self.launch_app(&app),
            Intent::CloseActiveApp => self.close_active(),
            Intent::GetSystemInfo(info) => self.get_sys_info(&info),
            Intent::Shutdown => self.shutdown(),
            Intent::Reboot => self.reboot(),
            Intent::Sleep => self.sleep(),
            Intent::SwitchApp(app) => self.switch_app(&app),
            Intent::VolumeUp => self.volume_up(),
            Intent::VolumeDown => self.volume_down(),
            Intent::ToggleMute => self.toggle_mute(),
            Intent::LockScreen => self.lock_screen(),
            Intent::Unknown(raw) => {
                warn!("Unknown command: {}", raw);
                ExecutionResult::Failed(format!("Unknown command: {}", raw))
            }
        }
    }

    fn launch_app(&self, app: &str) -> ExecutionResult {
        info!("Launching application: {}", app);
        match self.ipc.send_launch_app(app) {
            Ok(resp) => ExecutionResult::Success(format!("Launched {} — {}", app, resp)),
            Err(e) => {
                error!("IPC launch failed for {}: {}", app, e);
                ExecutionResult::Failed(format!("Failed to launch {}: {}", app, e))
            }
        }
    }

    fn close_active(&self) -> ExecutionResult {
        info!("Closing active application");
        ExecutionResult::NotImplemented("CloseActiveApp requires window manager integration".into())
    }

    fn get_sys_info(&self, info_type: &str) -> ExecutionResult {
        info!("Fetching system info: {}", info_type);
        let result = match info_type {
            "ram" => self.sys_info.ram_usage(),
            "cpu" => self.sys_info.cpu_usage(),
            "battery" => self.sys_info.battery_status(),
            "disk" => self.sys_info.disk_usage(),
            "network" => self.sys_info.network_status(),
            "temperature" => self.sys_info.temperature(),
            _ => return ExecutionResult::Failed(format!("Unknown info type: {}", info_type)),
        };
        ExecutionResult::Success(result)
    }

    fn shutdown(&self) -> ExecutionResult {
        info!("Shutdown requested");
        ExecutionResult::NotImplemented("Shutdown requires system-level privileges".into())
    }

    fn reboot(&self) -> ExecutionResult {
        info!("Reboot requested");
        ExecutionResult::NotImplemented("Reboot requires system-level privileges".into())
    }

    fn sleep(&self) -> ExecutionResult {
        info!("Sleep requested");
        ExecutionResult::NotImplemented("Sleep requires system-level privileges".into())
    }

    fn switch_app(&self, app: &str) -> ExecutionResult {
        info!("Switching to app: {}", app);
        ExecutionResult::NotImplemented(format!("SwitchApp({}) requires window manager", app))
    }

    fn volume_up(&self) -> ExecutionResult {
        ExecutionResult::NotImplemented("Volume control not yet implemented".into())
    }

    fn volume_down(&self) -> ExecutionResult {
        ExecutionResult::NotImplemented("Volume control not yet implemented".into())
    }

    fn toggle_mute(&self) -> ExecutionResult {
        ExecutionResult::NotImplemented("Mute toggle not yet implemented".into())
    }

    fn lock_screen(&self) -> ExecutionResult {
        ExecutionResult::NotImplemented("Screen lock not yet implemented".into())
    }
}

impl Default for CommandExecutor {
    fn default() -> Self { Self::new() }
}
