// Test file to verify Application Control Layer functionality
use crate::app_launcher::AppLauncher;
use crate::window_manager::WindowManager;
use crate::system_status::SystemStats;

pub fn test_app_launcher() {
    println!("Testing App Launcher...");
    
    let launcher = AppLauncher::new();
    let apps = launcher.get_all_apps();
    
    println!("Available applications:");
    for app in apps {
        println!("  - {} ({})", app.name, app.category);
        let available = launcher.is_app_available(&app.name);
        println!("    Available: {}", available);
    }
    
    // Test search functionality
    let search_results = launcher.search_apps("web");
    println!("Search results for 'web':");
    for app in search_results {
        println!("  - {}", app.name);
    }
    
    println!("App Launcher test completed.\n");
}

pub fn test_window_manager() {
    println!("Testing Window Manager...");
    
    let mut wm = WindowManager::new();
    
    // Register some test windows
    let window1 = wm.register_window("Test App 1".to_string(), 1234, 100, 100, 800, 600);
    let window2 = wm.register_window("Test App 2".to_string(), 5678, 200, 200, 1024, 768);
    
    println!("Window count: {}", wm.get_window_count());
    println!("Visible window count: {}", wm.get_visible_window_count());
    
    // Test focus management
    wm.set_window_focus(window1, true);
    if let Some(focused) = wm.get_focused_window() {
        println!("Focused window: {}", focused.title);
    }
    
    // Test window switching
    if let Some(next_id) = wm.switch_to_next_window() {
        if let Some(next_window) = wm.get_window(next_id) {
            println!("Switched to: {}", next_window.title);
        }
    }
    
    // Test window operations
    wm.minimize_window(window1);
    println!("Visible windows after minimizing: {}", wm.get_visible_window_count());
    
    wm.restore_window(window1);
    println!("Visible windows after restoring: {}", wm.get_visible_window_count());
    
    // Test window closing
    let _ = wm.close_window(window2);
    println!("Window count after closing: {}", wm.get_window_count());
    
    println!("Window Manager test completed.\n");
}

pub fn test_system_monitoring() {
    println!("Testing System Monitoring...");
    
    let mut stats = SystemStats::new();
    
    // Update system metrics
    stats.update();
    
    println!("Current system stats:");
    println!("  CPU Usage: {}%", stats.cpu_usage);
    println!("  RAM Usage: {}%", stats.ram_usage);
    println!("  Battery: {}% (Charging: {})", stats.battery_percent, stats.is_charging);
    println!("  Network: {}", if stats.network_connected { "Connected" } else { "Disconnected" });
    println!("  Disk Usage: {}%", stats.disk_usage);
    println!("  Temperature: {:.1}°C", stats.temperature);
    
    // Test alerts
    let alerts = stats.get_alerts();
    println!("Active alerts: {}", alerts.len());
    for alert in alerts {
        let severity = match alert.severity {
            crate::system_status::AlertSeverity::Info => "INFO",
            crate::system_status::AlertSeverity::Warning => "WARN",
            crate::system_status::AlertSeverity::Critical => "CRIT",
        };
        println!("  [{}] {}", severity, alert.message);
    }
    
    // Test smart suggestions
    let suggestions = stats.get_smart_suggestions();
    println!("Smart suggestions: {}", suggestions.len());
    for suggestion in suggestions {
        println!("  - {}", suggestion);
    }
    
    println!("System Monitoring test completed.\n");
}

pub fn run_all_tests() {
    println!("=== Draco-OS Application Control Layer Tests ===\n");
    
    test_app_launcher();
    test_window_manager();
    test_system_monitoring();
    
    println!("=== All tests completed ===");
}
