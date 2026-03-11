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
        }
    }

    fn draw_bar(&mut self) {
        self.window.set(BAR_COLOR);

        // Draw Draco Logo / Start Button
        self.window.rect(0, 0, BAR_HEIGHT, BAR_HEIGHT, DRACO_TEAL);
        self.font.render("D", 24.0).draw(&mut self.window, 12, 10, Color::rgb(255, 255, 255));

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
            // Draw dummy apps
            self.font.render("Applications", 20.0).draw(&mut l_win, 20, 20, TEXT_HIGHLIGHT_COLOR);
            
            let apps = ["Firefox", "Terminal", "Files", "Minecraft", "Settings", "Code"];
            for (i, app) in apps.iter().enumerate() {
                let y = 60 + (i as i32 * 40);
                self.font.render(app, 16.0).draw(&mut l_win, 40, y, TEXT_COLOR);
            }
            
            l_win.sync();
            self.launcher_window = Some(l_win);
        } else {
            self.launcher_window = None;
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
