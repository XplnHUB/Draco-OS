extern crate orbclient;
extern crate orbfont;
extern crate orbimage;
extern crate libredox;

mod theme;
mod system_status;
mod app_launcher;
mod window_manager;
mod ipc_server;
mod test_app_control;

use orbclient::{Color, EventOption, Renderer, Window, WindowFlag};
use orbfont::Font;
use std::env;
use std::process::Command;
use theme::*;
use system_status::SystemStats;
use app_launcher::AppLauncher;
use window_manager::WindowManager;
use ipc_server::IpcServer;

struct Shell {
    window: Window,
    launcher_window: Option<Window>,
    font: Font,
    stats: SystemStats,
    app_launcher: AppLauncher,
    window_manager: WindowManager,
    ipc_server: IpcServer,
    width: u32,
    height: u32,
    launcher_visible: bool,
    selected_app_index: usize,
}

impl Shell {
    fn new() -> Self {
        let (width, height) = orbclient::get_display_size().expect("Failed to get display size");
        let window = Window::new_flags(
            0,
            height as i32 - BAR_HEIGHT as i32,
            width,
            BAR_HEIGHT,
            "Draco Shell",
            &[WindowFlag::Async, WindowFlag::Borderless, WindowFlag::Transparent],
        ).expect("Failed to open shell window");

        let font = Font::find(None, None, None).expect("Failed to find font");
        let app_launcher = AppLauncher::new();
        let window_manager = WindowManager::new();
        
        Self {
            window,
            launcher_window: None,
            font,
            stats: SystemStats::new(),
            app_launcher,
            window_manager,
            ipc_server: IpcServer::new(), // Will be initialized after struct creation
            width,
            height,
            launcher_visible: false,
            selected_app_index: 0,
        }
    }

    fn draw_bar(&mut self) {
        self.window.set(BAR_COLOR);

        // Draw Draco Logo / Start Button
        self.window.rect(0, 0, BAR_HEIGHT, BAR_HEIGHT, DRACO_TEAL);
        self.font.render("D", 24.0).draw(&mut self.window, 12, 10, Color::rgb(255, 255, 255));

        // Draw Window Count
        let window_count = self.window_manager.get_visible_window_count();
        let window_text = format!("Windows: {}", window_count);
        let window_render = self.font.render(&window_text, FONT_SIZE);
        let window_x = self.width - 400;
        window_render.draw(&mut self.window, window_x as i32, 15, TEXT_HIGHLIGHT_COLOR);

        // Draw System Stats
        self.stats.update();
        let stats_text = format!(
            "CPU: {}% | RAM: {}% | BAT: {}%",
            self.stats.cpu_usage, self.stats.ram_usage, self.stats.battery_percent
        );
        
        let text_render = self.font.render(&stats_text, FONT_SIZE);
        let x = self.width - text_render.width() - 20;
        text_render.draw(&mut self.window, x as i32, 15, TEXT_COLOR);

        // Draw Clock
        let time_str = self.get_time();
        let time_render = self.font.render(&time_str, FONT_SIZE);
        time_render.draw(&mut self.window, (x - time_render.width() - 40) as i32, 15, TEXT_HIGHLIGHT_COLOR);

        self.window.sync();
    }

    fn toggle_launcher(&mut self) {
        self.launcher_visible = !self.launcher_visible;
        if self.launcher_visible {
            let l_width = 400;
            let l_height = 500;
            let mut l_win = Window::new_flags(
                20,
                self.height as i32 - BAR_HEIGHT as i32 - l_height as i32 - 10,
                l_width,
                l_height,
                "Draco Launcher",
                &[WindowFlag::Async, WindowFlag::Borderless, WindowFlag::Transparent],
            ).expect("Failed to open launcher window");
            
            l_win.set(BAR_COLOR);
            self.font.render("Applications", 20.0).draw(&mut l_win, 20, 20, TEXT_HIGHLIGHT_COLOR);
            
            let apps = self.app_launcher.get_all_apps();
            for (i, app) in apps.iter().enumerate() {
                let y = 60 + (i as i32 * 40);
                let color = if i == self.selected_app_index {
                    TEXT_HIGHLIGHT_COLOR
                } else {
                    TEXT_COLOR
                };
                
                // Draw app availability indicator
                let indicator_color = if self.app_launcher.is_app_available(&app.name) {
                    Color::rgb(0, 255, 0) // Green for available
                } else {
                    Color::rgb(255, 0, 0) // Red for unavailable
                };
                l_win.rect(25, y + 5, 10, 10, indicator_color);
                
                self.font.render(&app.name, 16.0).draw(&mut l_win, 40, y, color);
                self.font.render(&format!("({})", app.category), 12.0).draw(&mut l_win, 200, y + 4, Color::rgb(150, 150, 150));
            }
            
            l_win.sync();
            self.launcher_window = Some(l_win);
        } else {
            self.launcher_window = None;
            self.selected_app_index = 0;
        }
    }

    fn get_time(&self) -> String {
        "12:45".to_string()
    }

    fn run(&mut self) {
        self.draw_bar();
        
        'events: loop {
            for event in self.window.events() {
                match event.to_option() {
                    EventOption::Mouse(mouse_event) => {
                        if mouse_event.y < BAR_HEIGHT as i32 && mouse_event.x < BAR_HEIGHT as i32 {
                            if mouse_event.left {
                                self.toggle_launcher();
                            }
                        }
                    }
                    EventOption::Quit(_) => break 'events,
                    _ => (),
                }
            }

            if let Some(ref mut l_win) = self.launcher_window {
                for event in l_win.events() {
                     match event.to_option() {
                        EventOption::Mouse(mouse_event) => {
                            if mouse_event.left {
                                // Check which app was clicked
                                let apps = self.app_launcher.get_all_apps();
                                for (i, app) in apps.iter().enumerate() {
                                    let y = 60 + (i as i32 * 40);
                                    if mouse_event.y >= y && mouse_event.y <= y + 30 &&
                                       mouse_event.x >= 40 && mouse_event.x <= 350 {
                                        // Launch the app
                                        if let Err(e) = self.app_launcher.launch_app(&app.name) {
                                            eprintln!("Launch error: {}", e);
                                        }
                                        self.toggle_launcher(); // Close launcher after launch
                                        break;
                                    }
                                }
                            }
                        }
                        EventOption::Key(key_event) => {
                            if key_event.pressed {
                                match key_event.scancode {
                                    // Up arrow
                                    72 => {
                                        if self.selected_app_index > 0 {
                                            self.selected_app_index -= 1;
                                            self.toggle_launcher(); // Redraw
                                            self.toggle_launcher();
                                        }
                                    }
                                    // Down arrow
                                    80 => {
                                        let apps_count = self.app_launcher.get_all_apps().len();
                                        if self.selected_app_index < apps_count - 1 {
                                            self.selected_app_index += 1;
                                            self.toggle_launcher(); // Redraw
                                            self.toggle_launcher();
                                        }
                                    }
                                    // Enter key
                                    28 => {
                                        let apps = self.app_launcher.get_all_apps();
                                        if let Some(app) = apps.get(self.selected_app_index) {
                                            if let Err(e) = self.app_launcher.launch_app(&app.name) {
                                                eprintln!("Launch error: {}", e);
                                            }
                                            self.toggle_launcher(); // Close launcher after launch
                                        }
                                    }
                                    // Escape key
                                    1 => {
                                        self.toggle_launcher(); // Close launcher
                                    }
                                    _ => (),
                                }
                            }
                        }
                        EventOption::Focus(f) => {
                            if !f.focused {
                                // self.toggle_launcher(); // Close on blur
                            }
                        }
                        _ => (),
                     }
                }
            }
            
            self.draw_bar();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
