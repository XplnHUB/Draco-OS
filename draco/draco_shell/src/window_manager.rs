use std::collections::HashMap;
use std::process::{Command, Child};
use std::sync::{Arc, Mutex};
use orbclient::Window;

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub id: u32,
    pub title: String,
    pub process_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub is_focused: bool,
    pub is_minimized: bool,
}

pub struct WindowManager {
    windows: Arc<Mutex<HashMap<u32, WindowInfo>>>,
    next_window_id: u32,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Arc::new(Mutex::new(HashMap::new())),
            next_window_id: 1,
        }
    }

    pub fn register_window(&mut self, title: String, process_id: u32, x: i32, y: i32, width: u32, height: u32) -> u32 {
        let window_id = self.next_window_id;
        self.next_window_id += 1;

        let window_info = WindowInfo {
            id: window_id,
            title,
            process_id,
            x,
            y,
            width,
            height,
            is_focused: false,
            is_minimized: false,
        };

        let mut windows = self.windows.lock().unwrap();
        windows.insert(window_id, window_info);
        window_id
    }

    pub fn unregister_window(&self, window_id: u32) -> bool {
        let mut windows = self.windows.lock().unwrap();
        windows.remove(&window_id).is_some()
    }

    pub fn get_window(&self, window_id: u32) -> Option<WindowInfo> {
        let windows = self.windows.lock().unwrap();
        windows.get(&window_id).cloned()
    }

    pub fn get_all_windows(&self) -> Vec<WindowInfo> {
        let windows = self.windows.lock().unwrap();
        windows.values().cloned().collect()
    }

    pub fn get_focused_window(&self) -> Option<WindowInfo> {
        let windows = self.windows.lock().unwrap();
        windows.values().find(|w| w.is_focused).cloned()
    }

    pub fn set_window_focus(&self, window_id: u32, focused: bool) -> bool {
        let mut windows = self.windows.lock().unwrap();
        if let Some(window) = windows.get_mut(&window_id) {
            window.is_focused = focused;
            true
        } else {
            false
        }
    }

    pub fn minimize_window(&self, window_id: u32) -> bool {
        let mut windows = self.windows.lock().unwrap();
        if let Some(window) = windows.get_mut(&window_id) {
            window.is_minimized = true;
            true
        } else {
            false
        }
    }

    pub fn restore_window(&self, window_id: u32) -> bool {
        let mut windows = self.windows.lock().unwrap();
        if let Some(window) = windows.get_mut(&window_id) {
            window.is_minimized = false;
            true
        } else {
            false
        }
    }

    pub fn close_window(&self, window_id: u32) -> Result<(), String> {
        if let Some(window_info) = self.get_window(window_id) {
            // Try to gracefully close the process
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                let _ = Command::new("kill")
                    .args(&["-TERM", &window_info.process_id.to_string()])
                    .output();
            }
            
            // Remove from window registry
            self.unregister_window(window_id);
            Ok(())
        } else {
            Err(format!("Window {} not found", window_id))
        }
    }

    pub fn switch_to_next_window(&self) -> Option<u32> {
        let windows = self.windows.lock().unwrap();
        let mut window_list: Vec<&WindowInfo> = windows.values()
            .filter(|w| !w.is_minimized)
            .collect();
        
        if window_list.len() <= 1 {
            return None;
        }

        // Find current focused window
        let current_focused = window_list.iter().position(|w| w.is_focused);
        
        let next_index = match current_focused {
            Some(index) => (index + 1) % window_list.len(),
            None => 0,
        };

        let next_window_id = window_list[next_index].id;
        drop(windows);
        
        // Set focus to next window
        self.set_window_focus(next_window_id, true);
        
        // Unfocus others
        for window in self.get_all_windows() {
            if window.id != next_window_id {
                self.set_window_focus(window.id, false);
            }
        }

        Some(next_window_id)
    }

    pub fn get_window_count(&self) -> usize {
        let windows = self.windows.lock().unwrap();
        windows.len()
    }

    pub fn get_visible_window_count(&self) -> usize {
        let windows = self.windows.lock().unwrap();
        windows.values().filter(|w| !w.is_minimized).count()
    }
}
