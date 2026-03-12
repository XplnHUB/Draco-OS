extern crate orbclient;
extern crate orbfont;
extern crate orbimage;
extern crate libredox;

mod theme;
mod system_status;

use orbclient::{Color, EventOption, Renderer, Window, WindowFlag};
use orbfont::Font;
use std::env;
use std::process::Command;
use theme::*;
use system_status::SystemStats;

struct Shell {
    window: Window,
    launcher_window: Option<Window>,
    font: Font,
    stats: SystemStats,
    width: u32,
    height: u32,
    launcher_visible: bool,
    is_locked: bool,
    mouse_x: i32,
    mouse_y: i32,
    search_query: String,
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
        
        Self {
            window,
            launcher_window: None,
            font,
            stats: SystemStats::new(),
            width,
            height,
            launcher_visible: false,
            is_locked: true,
            mouse_x: 0,
            mouse_y: 0,
            search_query: String::new(),
        }
    }

    fn draw_bar(&mut self) {
        self.window.set(BAR_COLOR);
        // Bottom border line for "shadow" effect
        self.window.rect(0, BAR_HEIGHT as i32 - 2, self.width, 2, Color::rgba(0, 0, 0, 100));

        // Draw Draco Logo / Start Button (Pill shaped/accented)
        self.window.rect(0, 0, BAR_HEIGHT + 15, BAR_HEIGHT, DRACO_TEAL);
        self.font.render("D", 28.0).draw(&mut self.window, 22, 10, Color::rgb(255, 255, 255));

        // Draw System Stats with improved layout
        self.stats.update();
        let stats_text = format!(
            "CPU: {}%   RAM: {}%   BAT: {}%",
            self.stats.cpu_usage, self.stats.ram_usage, self.stats.battery_percent
        );
        
        let text_render = self.font.render(&stats_text, FONT_SIZE);
        let clock_str = self.get_time();
        let clock_render = self.font.render(&clock_str, FONT_SIZE + 2.0);
        
        let clock_x = self.width as i32 - clock_render.width() as i32 - 20;
        let stats_x = clock_x - text_render.width() as i32 - 40;

        // Draw separator
        self.window.rect(clock_x - 20, 12, 1, 24, Color::rgba(255, 255, 255, 50));

        text_render.draw(&mut self.window, stats_x, 15, TEXT_COLOR);
        clock_render.draw(&mut self.window, clock_x, 14, TEXT_HIGHLIGHT_COLOR);

        self.window.sync();
    }

    fn draw_lock_screen(&mut self) {
        // Draw a dark frosted overlay effect
        self.window.set(Color::rgba(25, 25, 35, 255)); // solid darker background
        
        let cx = self.width as i32 / 2;
        let cy = self.height as i32 / 2 - 80;

        // "Avatar" placeholder ring
        self.window.circle(cx, cy, 60, DRACO_TEAL);
        self.window.circle(cx, cy, 58, Color::rgb(40, 40, 50));
        self.font.render("User", 24.0).draw(&mut self.window, cx - 25, cy - 12, TEXT_HIGHLIGHT_COLOR);

        let msg = "System Locked";
        let text_render = self.font.render(msg, 32.0);
        text_render.draw(&mut self.window, cx - (text_render.width() as i32 / 2), cy + 90, TEXT_HIGHLIGHT_COLOR);

        let sub_msg = "Waiting for Face or Voice Authentication...";
        let sub_render = self.font.render(sub_msg, 20.0);
        sub_render.draw(&mut self.window, cx - (sub_render.width() as i32 / 2), cy + 140, TEXT_COLOR);
        
        let hint_msg = "[ Press 'U' to mock unlock ]";
        let hint_render = self.font.render(hint_msg, 16.0);
        hint_render.draw(&mut self.window, cx - (hint_render.width() as i32 / 2), cy + 180, Color::rgb(100, 150, 150));
        
        self.window.sync();
    }

    fn update_launcher(&mut self) {
        let mut l_win_opt = self.launcher_window.take();
        if let Some(ref mut l_win) = l_win_opt {
            let l_width = 450;
            l_win.set(Color::rgba(30, 35, 40, 240));
            // Top accent border
            l_win.rect(0, 0, l_width, 4, DRACO_TEAL);

            self.font.render("Applications", 24.0).draw(l_win, 25, 25, TEXT_HIGHLIGHT_COLOR);
            
            // Draw search query box
            l_win.rect(25, 60, l_width - 50, 40, Color::rgba(0, 0, 0, 100));
            l_win.rect(25, 60, l_width - 50, 2, Color::rgba(255, 255, 255, 30)); // Inner top shadow
            
            let mut display_query = self.search_query.clone();
            if display_query.is_empty() {
                self.font.render("Type to search...", 18.0).draw(l_win, 35, 70, Color::rgba(255, 255, 255, 100));
            } else {
                display_query.push('_'); // cursor
                self.font.render(&display_query, 18.0).draw(l_win, 35, 70, TEXT_HIGHLIGHT_COLOR);
            }

            l_win.rect(25, 110, l_width - 50, 1, Color::rgba(255, 255, 255, 30));
            
            let apps = ["Firefox Native", "Draco Terminal", "Files", "Minecraft", "System Settings", "VS Code (WASI)"];
            let mut y = 130;
            for (i, app) in apps.iter().enumerate() {
                if !self.search_query.is_empty() && !app.to_lowercase().contains(&self.search_query.to_lowercase()) {
                    continue;
                }
                
                // Optional: hover/selection mock for the first element
                if i == 0 && self.search_query.is_empty() {
                    l_win.rect(15, y - 5, l_width - 30, 35, Color::rgba(0, 200, 200, 40));
                }
                
                self.font.render(*app, 18.0).draw(l_win, 55, y, TEXT_COLOR);
                // "Icon" placeholder
                l_win.rect(25, y + 2, 16, 16, DRACO_ORANGE);
                
                y += 50;
            }
            
            l_win.sync();
        }
        self.launcher_window = l_win_opt;
    }

    fn toggle_launcher(&mut self) {
        self.launcher_visible = !self.launcher_visible;
        if self.launcher_visible {
            let l_width = 450;
            let l_height = 550;
            let l_win = Window::new_flags(
                10,
                self.height as i32 - BAR_HEIGHT as i32 - l_height as i32 - 10,
                l_width,
                l_height,
                "Draco Launcher",
                &[WindowFlag::Async, WindowFlag::Borderless, WindowFlag::Transparent],
            ).expect("Failed to open launcher window");
            
            self.launcher_window = Some(l_win);
            self.search_query.clear();
            self.update_launcher();
        } else {
            self.launcher_window = None;
            self.search_query.clear();
        }
    }

    fn get_time(&self) -> String {
        "12:45".to_string()
    }

    fn run(&mut self) {
        if self.is_locked {
            self.draw_lock_screen();
        } else {
            self.draw_bar();
        }
        
        'events: loop {
            for event in self.window.events() {
                match event.to_option() {
                    EventOption::Key(key_event) => {
                        if self.is_locked {
                            // Mock receiving an UnlockScreen IPC message
                            if key_event.pressed && key_event.character == 'u' || key_event.character == 'U' {
                                self.is_locked = false;
                                // Force full redraw of the bar to clear the lock screen
                                self.window.set(Color::rgba(0,0,0,0)); 
                                self.draw_bar();
                            }
                        }
                    }
                    EventOption::Mouse(mouse_event) => {
                        self.mouse_x = mouse_event.x;
                        self.mouse_y = mouse_event.y;
                    }
                    EventOption::Button(button_event) => {
                        if !self.is_locked && button_event.left
                            && self.mouse_x < BAR_HEIGHT as i32 
                            && self.mouse_y < BAR_HEIGHT as i32 
                        {
                            self.toggle_launcher();
                        }
                    }
                    EventOption::Quit(_) => break 'events,
                    _ => (),
                }
            }

            if !self.is_locked {
                let mut needs_launcher_redraw = false;
                let mut close_launcher = false;

                if let Some(ref mut l_win) = self.launcher_window {
                    for event in l_win.events() {
                         match event.to_option() {
                            EventOption::Focus(f) => {
                                if !f.focused { }
                            }
                            EventOption::Key(key_event) => {
                                if key_event.pressed {
                                    match key_event.scancode {
                                        orbclient::K_ESC => {
                                            close_launcher = true;
                                        }
                                        orbclient::K_BKSP => {
                                            self.search_query.pop();
                                            needs_launcher_redraw = true;
                                        }
                                        _ => {
                                            let c = key_event.character;
                                            if c != '\0' && c != '\n' && c != '\r' && c != '\x08' {
                                                self.search_query.push(c);
                                                needs_launcher_redraw = true;
                                            }
                                        }
                                    }
                                }
                            }
                            EventOption::Quit(_) => {
                                close_launcher = true;
                            }
                            _ => (),
                         }
                    }
                }
                
                if close_launcher {
                    self.launcher_visible = false;
                    self.launcher_window = None;
                    self.search_query.clear();
                } else if needs_launcher_redraw {
                    self.update_launcher();
                }
                
                self.draw_bar();
            } else {
                self.draw_lock_screen();
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
