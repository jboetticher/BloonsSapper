#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monkey;

use std::{
    sync::{Arc, Mutex},
    thread, time, ptr
};
use winapi::um::winuser::{FindWindowA, SetForegroundWindow};
use inputbot::{KeybdKey, MouseButton, MouseCursor};
use monkey::*;

fn main() -> eframe::Result<()> {
    println!("Starting the Bloons Sapper BOT!");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Bloons Sapper Bot",
        native_options,
        Box::new(|cc| {
            let app = Box::new(SapperApp::new(cc));
            app.spawn_thread();
            app
        }),
    )
}

pub struct SapperApp {
    enabled: Arc<Mutex<bool>>,
    current_money: i32,
}

impl SapperApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        SapperApp {
            current_money: 0,
            enabled: Arc::new(Mutex::new(false)),
        }
    }

    // Main logic for the macro
    fn spawn_thread(&self) {
        // The macro thread
        let c_enabled = self.enabled.clone();
        thread::spawn(move || loop {
            if *c_enabled.lock().unwrap() {
                // Attempt to open up the
                let bloons_window_title = "BloonsTD6";
                unsafe {
                    // Find the window by its title
                    let window_handle =
                        FindWindowA(ptr::null_mut(), bloons_window_title.as_ptr() as *const i8);

                    if window_handle != ptr::null_mut() {
                        // Bring the window to the foreground
                        SetForegroundWindow(window_handle);
                    } else {
                        println!("Window not found");
                        thread::sleep(time::Duration::from_millis(30000));
                        continue;
                    }
                }

                thread::sleep(time::Duration::from_secs(5));

                // region: Enter New Game

                left_click(830, 930);
                thread::sleep(time::Duration::from_secs(2));
                for _ in 0..3 {
                    left_click(580, 980);
                    thread::sleep(time::Duration::from_millis(100));
                }

                left_click(535, 560);
                thread::sleep(time::Duration::from_secs(1));
                left_click(1290, 540);
                thread::sleep(time::Duration::from_secs(1));
                left_click(955, 735);
                thread::sleep(time::Duration::from_secs(10));

                // endregion
            
                spawn_monkey(485, 463, Monkeys::Hero);
            }
        });
    }
}

impl eframe::App for SapperApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("BLOONS SAPPER BOT");

            ui.label(format!("OCR Money: {}", self.current_money));

            let c_enabled = self.enabled.clone();
            let lock = c_enabled.lock();
            let mut is_enabled = lock.unwrap();
            let button_label = match *is_enabled {
                true => "Disable Bot",
                false => "Enable Bot",
            };

            if ui.button(button_label).clicked() {
                // Enables / Disables the macro
                *is_enabled = !*is_enabled;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/jboetticher/BloonsSapper",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
