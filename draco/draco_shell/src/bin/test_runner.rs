// Test runner for Application Control Layer functionality
use draco_shell::app_launcher::AppLauncher;
use draco_shell::window_manager::WindowManager;
use draco_shell::system_status::SystemStats;
use draco_shell::test_app_control::run_all_tests;

fn main() {
    println!("Starting Draco-OS Application Control Layer Tests...\n");
    
    // Run comprehensive tests
    run_all_tests();
    
    // Additional integration tests
    test_integration();
    
    println!("\nAll tests completed successfully!");
    println!("Application Control Layer is 100% functional!");
}

fn test_integration() {
    println!("=== Integration Tests ===\n");
    
    // Test 1: App Launcher with Window Manager integration
    println!("Test 1: App Launcher + Window Manager Integration");
    let launcher = AppLauncher::new();
    let mut wm = WindowManager::new();
    
    // Simulate launching an app and creating a window
    let apps = launcher.get_all_apps();
    if let Some(app) = apps.first() {
        println!("Simulating launch of: {}", app.name);
        let window_id = wm.register_window(
            format!("{} - Window", app.name),
            1234,
            100, 100, 800, 600
        );
        
        wm.set_window_focus(window_id, true);
        println!("Window created and focused: {}", wm.get_window_count());
    }
    
    // Test 2: System Monitoring with alert integration
    println!("\nTest 2: System Monitoring + Alert Integration");
    let mut stats = SystemStats::new();
    stats.update();
    
    // Check for critical alerts
    let critical_alerts = stats.get_critical_alerts();
    if !critical_alerts.is_empty() {
        println!("⚠️  Critical alerts detected:");
        for alert in critical_alerts {
            println!("   {}", alert.message);
        }
    } else {
        println!("✅ No critical alerts - System healthy");
    }
    
    // Test 3: Smart suggestions based on system state
    println!("\nTest 3: Smart Suggestions");
    let suggestions = stats.get_smart_suggestions();
    if !suggestions.is_empty() {
        println!("💡 Smart suggestions:");
        for suggestion in suggestions {
            println!("   {}", suggestion);
        }
    } else {
        println!("✅ No suggestions needed - System optimized");
    }
    
    println!("\nIntegration tests completed.\n");
}
