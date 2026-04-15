use std::fs;
use std::io;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SystemAlert {
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

pub struct SystemStats {
    pub cpu_usage: u32,
    pub ram_usage: u32,
    pub battery_percent: u32,
    pub is_charging: bool,
    pub network_connected: bool,
    pub disk_usage: u32,
    pub temperature: f32,
    alerts: Vec<SystemAlert>,
    last_update: std::time::SystemTime,
}

impl SystemStats {
    pub fn new() -> Self {
        Self {
            cpu_usage: 0,
            ram_usage: 0,
            battery_percent: 100,
            is_charging: false,
            network_connected: false,
            disk_usage: 0,
            temperature: 0.0,
            alerts: Vec::new(),
            last_update: std::time::SystemTime::now(),
        }
    }

    pub fn update(&mut self) {
        let now = std::time::SystemTime::now();
        
        // Update all system metrics
        self.cpu_usage = self.read_cpu().unwrap_or(0);
        self.ram_usage = self.read_ram().unwrap_or(0);
        self.battery_percent = self.read_battery().unwrap_or(100);
        self.is_charging = self.read_battery_status().unwrap_or(false);
        self.network_connected = self.check_network_connection().unwrap_or(false);
        self.disk_usage = self.read_disk_usage().unwrap_or(0);
        self.temperature = self.read_temperature().unwrap_or(0.0);
        
        // Check for alerts and suggestions
        self.check_system_alerts();
        
        self.last_update = now;
    }

    fn read_cpu(&self) -> io::Result<u32> {
        // Try to read from /proc/stat on Linux-like systems
        if let Ok(content) = fs::read_to_string("/proc/stat") {
            // Parse CPU usage from /proc/stat
            let lines: Vec<&str> = content.lines().collect();
            if let Some(cpu_line) = lines.iter().find(|line| line.starts_with("cpu ")) {
                let parts: Vec<u64> = cpu_line
                    .split_whitespace()
                    .skip(1)
                    .filter_map(|s| s.parse().ok())
                    .collect();
                
                if parts.len() >= 4 {
                    let total = parts.iter().sum::<u64>();
                    let idle = parts[3];
                    let usage = if total > 0 {
                        ((total - idle) * 100) / total
                    } else {
                        0
                    };
                    return Ok(usage as u32);
                }
            }
        }
        
        // Fallback to Redox scheme or mock data
        self.read_redox_cpu()
    }

    fn read_redox_cpu(&self) -> io::Result<u32> {
        // Try Redox-specific CPU monitoring
        if let Ok(content) = fs::read_to_string("/scheme/cpu/usage") {
            Ok(content.trim().parse().unwrap_or(0))
        } else {
            // Generate realistic mock CPU usage
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let mut hasher = DefaultHasher::new();
            std::time::SystemTime::now().hash(&mut hasher);
            let hash = hasher.finish();
            Ok((hash % 100) as u32)
        }
    }

    fn read_ram(&self) -> io::Result<u32> {
        // Try to read from /proc/meminfo on Linux-like systems
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            let lines: Vec<&str> = content.lines().collect();
            let mut total_mem = 0u64;
            let mut free_mem = 0u64;
            
            for line in lines {
                if line.starts_with("MemTotal:") {
                    if let Some(num) = line.split_whitespace().nth(1) {
                        total_mem = num.parse().unwrap_or(0);
                    }
                } else if line.starts_with("MemAvailable:") {
                    if let Some(num) = line.split_whitespace().nth(1) {
                        free_mem = num.parse().unwrap_or(0);
                    }
                }
            }
            
            if total_mem > 0 {
                let used = total_mem - free_mem;
                return Ok(((used * 100) / total_mem) as u32);
            }
        }
        
        // Fallback to Redox scheme or mock data
        self.read_redox_ram()
    }

    fn read_redox_ram(&self) -> io::Result<u32> {
        // Try Redox-specific RAM monitoring
        if let Ok(content) = fs::read_to_string("/scheme/memory/usage") {
            Ok(content.trim().parse().unwrap_or(0))
        } else {
            // Generate realistic mock RAM usage
            Ok(45 + (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() % 30) as u32)
        }
    }

    fn read_battery(&self) -> io::Result<u32> {
        // Try to read from /sys/class/power_supply on Linux-like systems
        if let Ok(content) = fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
            Ok(content.trim().parse().unwrap_or(100))
        } else if let Ok(content) = fs::read_to_string("/scheme/battery/capacity") {
            // Redox battery scheme
            Ok(content.trim().parse().unwrap_or(100))
        } else {
            // Mock battery that slowly drains
            let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            Ok(80 - ((time / 60) % 80) as u32)
        }
    }

    fn read_battery_status(&self) -> io::Result<bool> {
        // Try to read battery charging status
        if let Ok(content) = fs::read_to_string("/sys/class/power_supply/BAT0/status") {
            Ok(content.trim().to_lowercase() == "charging")
        } else if let Ok(content) = fs::read_to_string("/scheme/battery/status") {
            Ok(content.trim().to_lowercase() == "charging")
        } else {
            // Mock charging status
            Ok(false)
        }
    }

    fn check_network_connection(&self) -> io::Result<bool> {
        // Check network connectivity by pinging a reliable host
        let output = Command::new("ping")
            .args(&["-c", "1", "-W", "1", "8.8.8.8"])
            .output();
        
        match output {
            Ok(result) => Ok(result.status.success()),
            Err(_) => {
                // Try alternative method
                Command::new("curl")
                    .args(&["-s", "--connect-timeout", "1", "http://example.com"])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
            }
        }
    }

    fn read_disk_usage(&self) -> io::Result<u32> {
        // Try to get disk usage using df command
        if let Ok(output) = Command::new("df").arg("/").output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                let lines: Vec<&str> = output_str.lines().collect();
                if lines.len() > 1 {
                    let parts: Vec<&str> = lines[1].split_whitespace().collect();
                    if parts.len() >= 5 {
                        if let Some(usage_str) = parts.get(4) {
                            if let Some(usage) = usage_str.trim_end_matches('%').parse().ok() {
                                return Ok(usage);
                            }
                        }
                    }
                }
            }
        }
        
        // Mock disk usage
        Ok(60)
    }

    fn read_temperature(&self) -> io::Result<f32> {
        // Try to read CPU temperature
        if let Ok(content) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
            if let Ok(temp_millidegrees) = content.trim().parse::<i32>() {
                return Ok(temp_millidegrees as f32 / 1000.0);
            }
        }
        
        // Mock temperature
        Ok(45.0 + (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() % 20) as f32)
    }

    fn check_system_alerts(&mut self) {
        let now = std::time::SystemTime::now();
        
        // Clear old alerts (older than 5 minutes)
        self.alerts.retain(|alert| {
            now.duration_since(alert.timestamp).unwrap_or_default().as_secs() < 300
        });
        
        // Battery alerts
        if self.battery_percent < 20 && !self.is_charging {
            self.add_alert(
                format!("Low battery: {}% remaining", self.battery_percent),
                AlertSeverity::Warning,
            );
        }
        
        if self.battery_percent < 10 && !self.is_charging {
            self.add_alert(
                format!("Critical battery: {}% remaining - charge soon!", self.battery_percent),
                AlertSeverity::Critical,
            );
        }
        
        // CPU alerts
        if self.cpu_usage > 90 {
            self.add_alert(
                format!("High CPU usage: {}% - consider closing applications", self.cpu_usage),
                AlertSeverity::Warning,
            );
        }
        
        // RAM alerts
        if self.ram_usage > 85 {
            self.add_alert(
                format!("High RAM usage: {}% - system may become slow", self.ram_usage),
                AlertSeverity::Warning,
            );
        }
        
        // Temperature alerts
        if self.temperature > 70.0 {
            self.add_alert(
                format!("High temperature: {:.1}°C - system may throttle", self.temperature),
                AlertSeverity::Warning,
            );
        }
        
        // Network alerts
        if !self.network_connected {
            self.add_alert(
                "No network connection".to_string(),
                AlertSeverity::Info,
            );
        }
        
        // Disk space alerts
        if self.disk_usage > 90 {
            self.add_alert(
                format!("Low disk space: {}% used", self.disk_usage),
                AlertSeverity::Critical,
            );
        }
    }

    fn add_alert(&mut self, message: String, severity: AlertSeverity) {
        let now = std::time::SystemTime::now();
        // Check if we already have a similar alert to avoid spam
        let similar_exists = self.alerts.iter().any(|alert| {
            alert.message == message && 
            now.duration_since(alert.timestamp).unwrap_or_default().as_secs() < 60
        });
        
        if !similar_exists {
            self.alerts.push(SystemAlert {
                message,
                severity,
                timestamp: now,
            });
        }
    }

    pub fn get_alerts(&self) -> &[SystemAlert] {
        &self.alerts
    }

    pub fn get_critical_alerts(&self) -> Vec<&SystemAlert> {
        self.alerts.iter()
            .filter(|alert| matches!(alert.severity, AlertSeverity::Critical))
            .collect()
    }

    pub fn get_warnings(&self) -> Vec<&SystemAlert> {
        self.alerts.iter()
            .filter(|alert| matches!(alert.severity, AlertSeverity::Warning))
            .collect()
    }

    pub fn get_smart_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if self.cpu_usage > 80 {
            suggestions.push("Consider closing resource-intensive applications".to_string());
        }
        
        if self.ram_usage > 75 {
            suggestions.push("Free up memory by closing unused applications".to_string());
        }
        
        if self.battery_percent < 30 && !self.is_charging {
            suggestions.push("Connect charger to preserve battery life".to_string());
        }
        
        if self.disk_usage > 80 {
            suggestions.push("Clean up disk space to maintain performance".to_string());
        }
        
        if self.temperature > 65.0 {
            suggestions.push("Ensure proper ventilation to reduce temperature".to_string());
        }
        
        suggestions
    }
}
