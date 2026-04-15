use std::fs;
use std::process::Command;

struct CpuTimes { idle: u64, total: u64 }

pub struct SystemInfoProvider;

impl SystemInfoProvider {
    pub fn new() -> Self { Self }

    pub fn ram_usage(&self) -> String {
        let (t, a) = self.read_meminfo();
        if t == 0 { return "RAM info unavailable".into(); }
        let u = t - a;
        format!("RAM: {}% used ({}MB / {}MB)", (u*100)/t, u/1024, t/1024)
    }

    pub fn cpu_usage(&self) -> String {
        match self.read_cpu_usage() { Some(v) => format!("CPU: {:.1}% usage", v), None => "CPU info unavailable".into() }
    }

    pub fn battery_status(&self) -> String {
        let (pct, ch) = self.read_battery();
        format!("Battery: {}% ({})", pct, if ch { "Charging" } else { "Discharging" })
    }

    pub fn disk_usage(&self) -> String {
        Command::new("df").arg("-h").arg("/").output().ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| s.lines().nth(1).map(|l| l.to_string()))
            .map(|l| { let p: Vec<&str>=l.split_whitespace().collect();
                if p.len()>=5 { format!("Disk: {} of {} ({})",p[2],p[1],p[4]) } else { "Disk info unavailable".into() }})
            .unwrap_or_else(|| "Disk info unavailable".into())
    }

    pub fn network_status(&self) -> String {
        let up = Command::new("ping").args(["-c","1","-W","1","8.8.8.8"]).output().map(|o|o.status.success()).unwrap_or(false);
        if up { "Network: Connected" } else { "Network: Disconnected" }.into()
    }

    pub fn temperature(&self) -> String {
        self.read_temp().map(|t| format!("Temperature: {:.1}°C", t))
            .unwrap_or_else(|| "Temperature info unavailable".into())
    }

    fn read_meminfo(&self) -> (u64, u64) {
        let c = fs::read_to_string("/proc/meminfo").unwrap_or_default();
        let (mut t, mut a) = (0u64, 0u64);
        for l in c.lines() {
            if l.starts_with("MemTotal:") { t = l.split_whitespace().nth(1).and_then(|v|v.parse().ok()).unwrap_or(0); }
            if l.starts_with("MemAvailable:") { a = l.split_whitespace().nth(1).and_then(|v|v.parse().ok()).unwrap_or(0); }
        }
        (t, a)
    }

    fn read_cpu_times(&self) -> Option<CpuTimes> {
        let c = fs::read_to_string("/proc/stat").ok()?;
        let f: Vec<u64> = c.lines().next()?.split_whitespace().skip(1).filter_map(|x|x.parse().ok()).collect();
        let idle = f.get(3).copied().unwrap_or(0) + f.get(4).copied().unwrap_or(0);
        Some(CpuTimes { idle, total: f.iter().sum() })
    }

    fn read_cpu_usage(&self) -> Option<f64> {
        let t1 = self.read_cpu_times()?;
        std::thread::sleep(std::time::Duration::from_millis(200));
        let t2 = self.read_cpu_times()?;
        let di = t2.idle.saturating_sub(t1.idle) as f64;
        let dt = t2.total.saturating_sub(t1.total) as f64;
        if dt == 0.0 { return None; }
        Some((1.0 - di/dt) * 100.0)
    }

    fn read_battery(&self) -> (u32, bool) {
        for b in &["BAT0","BAT1"] {
            let base = format!("/sys/class/power_supply/{}", b);
            let pct: u32 = fs::read_to_string(format!("{}/capacity",base)).ok()
                .and_then(|s|s.trim().parse().ok()).unwrap_or(0);
            let st = fs::read_to_string(format!("{}/status",base)).unwrap_or_default();
            if pct > 0 { return (pct, st.contains("Charging") || st.contains("Full")); }
        }
        (0, false)
    }

    fn read_temp(&self) -> Option<f64> {
        fs::read_to_string("/sys/class/thermal/thermal_zone0/temp").ok()
            .and_then(|s| s.trim().parse::<f64>().ok())
            .map(|m| m / 1000.0)
    }
}
