use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub icon: Option<String>,
    pub category: String,
}

pub struct AppLauncher {
    apps: HashMap<String, Application>,
}

impl AppLauncher {
    pub fn new() -> Self {
        let mut launcher = Self {
            apps: HashMap::new(),
        };
        launcher.load_default_apps();
        launcher
    }

    fn load_default_apps(&mut self) {
        // Default applications for Draco-OS
        let default_apps = vec![
            Application {
                name: "Firefox".to_string(),
                command: "firefox".to_string(),
                args: vec![],
                icon: None,
                category: "Web".to_string(),
            },
            Application {
                name: "Terminal".to_string(),
                command: "orbterm".to_string(),
                args: vec![],
                icon: None,
                category: "System".to_string(),
            },
            Application {
                name: "Files".to_string(),
                command: "filemanager".to_string(),
                args: vec![],
                icon: None,
                category: "System".to_string(),
            },
            Application {
                name: "Minecraft".to_string(),
                command: "minecraft".to_string(),
                args: vec![],
                icon: None,
                category: "Games".to_string(),
            },
            Application {
                name: "Settings".to_string(),
                command: "settings".to_string(),
                args: vec![],
                icon: None,
                category: "System".to_string(),
            },
            Application {
                name: "Code".to_string(),
                command: "code".to_string(),
                args: vec![],
                icon: None,
                category: "Development".to_string(),
            },
            Application {
                name: "Calculator".to_string(),
                command: "calculator".to_string(),
                args: vec![],
                icon: None,
                category: "Utility".to_string(),
            },
            Application {
                name: "Text Editor".to_string(),
                command: "texteditor".to_string(),
                args: vec![],
                icon: None,
                category: "Utility".to_string(),
            },
        ];

        for app in default_apps {
            self.apps.insert(app.name.clone(), app);
        }
    }

    pub fn launch_app(&self, app_name: &str) -> Result<(), String> {
        if let Some(app) = self.apps.get(app_name) {
            println!("Launching {} with command: {}", app.name, app.command);
            
            let result = Command::new(&app.command)
                .args(&app.args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();

            match result {
                Ok(_) => {
                    println!("Successfully launched {}", app.name);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to launch {}: {}", app.name, e);
                    Err(format!("Failed to launch {}: {}", app.name, e))
                }
            }
        } else {
            Err(format!("Application '{}' not found", app_name))
        }
    }

    pub fn get_all_apps(&self) -> Vec<&Application> {
        self.apps.values().collect()
    }

    pub fn get_apps_by_category(&self, category: &str) -> Vec<&Application> {
        self.apps
            .values()
            .filter(|app| app.category == category)
            .collect()
    }

    pub fn search_apps(&self, query: &str) -> Vec<&Application> {
        let query_lower = query.to_lowercase();
        self.apps
            .values()
            .filter(|app| {
                app.name.to_lowercase().contains(&query_lower) ||
                app.category.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn is_app_available(&self, app_name: &str) -> bool {
        if let Some(app) = self.apps.get(app_name) {
            // Check if the command exists in the system
            Path::new(&app.command).exists() || 
            which::which(&app.command).is_ok()
        } else {
            false
        }
    }
}
