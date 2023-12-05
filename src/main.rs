#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monkey;

use dirs;
use inputbot::{KeybdKey, MouseButton, MouseCursor};
use monkey::*;
use rust_ocr::png_to_text;
use screenshots::Screen;
use std::{
    error::Error,
    ptr,
    sync::{Arc, Mutex},
    thread, time,
};
use winapi::um::winuser::{FindWindowA, SetForegroundWindow};

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
            let mut app = Box::new(SapperApp::new(cc));
            app.spawn_thread();
            app
        }),
    )
}

pub struct SapperApp {
    enabled: Arc<Mutex<bool>>,
    current_money: Arc<Mutex<i32>>,
    upgrade_stage: Arc<Mutex<i32>>,
}

impl SapperApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        SapperApp {
            current_money: Arc::new(Mutex::new(0)),
            enabled: Arc::new(Mutex::new(false)),
            upgrade_stage: Arc::new(Mutex::new(0)),
        }
    }

    // Main logic for the macro
    fn spawn_thread(&mut self) {
        // The macro thread
        let c_enabled = self.enabled.clone();
        let c_current_money = self.current_money.clone();
        let c_upgrade_stage = self.upgrade_stage.clone();

        thread::spawn(move || loop {
            if *c_enabled.lock().unwrap() {
                // Attempt to open up the app
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
                thread::sleep(time::Duration::from_secs(8));

                // endregion

                // Spawn the hero as first action
                spawn_monkey(485, 463, Monkeys::Hero);

                // Start game on double speed
                KeybdKey::SpaceKey.press();
                KeybdKey::SpaceKey.release();
                thread::sleep(time::Duration::from_millis(500));
                KeybdKey::SpaceKey.press();
                KeybdKey::SpaceKey.release();

                // Upgrade check loop
                let screen = Screen::all().unwrap();
                let mut helicopter = MonkeyInstance::new();
                let mut money = 0;
                let mut is_early_defeat = false;

                loop {
                    // Take a screenshot
                    let ocr_money = Self::read_money_data(&screen[0]);
                    println!("Money discovered by OCR: {}", ocr_money);
                    if ocr_money.abs_diff(money) < 1000 && ocr_money != 0 {
                        money = ocr_money; // Sometimes OCR is finnicky
                    } else {
                        thread::sleep(time::Duration::from_secs(1));
                        continue;
                    }

                    // Set current_money
                    let current_money_lock = c_current_money.lock();
                    *(current_money_lock.unwrap()) = money;

                    // Check for upgrade step
                    let upgrade_stage_lock = c_upgrade_stage.lock();
                    let mut upgrade_level = upgrade_stage_lock.unwrap();
                    if Self::attempt_upgrade(&mut helicopter, *upgrade_level, money) {
                        *upgrade_level += 1;
                    }

                    // Exit when upgrades are over
                    if *upgrade_level > 7 {
                        println!("Upgrade section finished.");
                        *upgrade_level = 0;
                        break;
                    }

                    // Repeat every half second
                    thread::sleep(time::Duration::from_millis(500));

                    // Check for victory if it didn't manage to upgrade the last time
                    if *upgrade_level == 7 {
                        // TODO: fix this or something
                        if Self::check_for_victory(&screen[0]) {
                            *upgrade_level = 0;

                            // Click on next button
                            thread::sleep(time::Duration::from_millis(500));
                            left_click(958, 902);

                            // Click on home button
                            thread::sleep(time::Duration::from_millis(500));
                            left_click(725, 845);

                            break;
                        }
                    }
                    // Otherwise, check for the early defeat
                    else if Self::check_for_early_defeat(&screen[0]) {
                        println!("Early defeat found!");
                        is_early_defeat = true;
                        *upgrade_level = 0;
                        break;
                    }
                }

                // Restart loop if early defeat
                if is_early_defeat {
                    thread::sleep(time::Duration::from_secs(1));
                    left_click(625, 800);
                    thread::sleep(time::Duration::from_secs(8));
                    continue;
                }

                // Victory screen check loop
                loop {
                    if Self::check_for_victory(&screen[0]) {
                        break;
                    }
                    thread::sleep(time::Duration::from_secs(8));
                }

                // region: Continue & Reset Game

                // Click on next button
                thread::sleep(time::Duration::from_millis(500));
                left_click(958, 902);

                // Click on continue button
                thread::sleep(time::Duration::from_secs(2));
                left_click(1200, 850);

                // Click on Ok button for freeplay
                thread::sleep(time::Duration::from_secs(2));
                left_click(960, 760);

                // Space to play
                thread::sleep(time::Duration::from_secs(2));
                KeybdKey::SpaceKey.press();
                KeybdKey::SpaceKey.release();
                thread::sleep(time::Duration::from_millis(500));

                // Check for defeat NEXT button
                loop {
                    if Self::check_for_victory(&screen[0]) {
                        break;
                    }
                    thread::sleep(time::Duration::from_secs(8));
                }

                // Click on next button
                thread::sleep(time::Duration::from_secs(1));
                left_click(958, 902);

                // Click on home button
                thread::sleep(time::Duration::from_secs(2));
                left_click(626, 808);

                // endregion

                // Sleep before returning to main page
                thread::sleep(time::Duration::from_secs(8));
            }
        });
    }

    // Returns true if it did upgrade
    fn attempt_upgrade(
        monkey_instance: &mut MonkeyInstance,
        upgrade_level: i32,
        money: i32,
    ) -> bool {
        match upgrade_level {
            0 => {
                // Create Heli (assumes you have the monkey knowledge)
                if money > 1070 {
                    monkey_instance.replace(spawn_monkey(550, 280, Monkeys::Heli));
                    println!("Upgrade 1");
                    return true;
                }
                false
            }
            1 => {
                // Upgrade Top path to 1
                if money > 865 {
                    upgrade_monkey(monkey_instance, vec![UpgradePath::Top]);
                    println!("Upgrade 2");
                    return true;
                }
                false
            }
            2 => {
                // Upgrade top path to 2
                if money > 540 {
                    upgrade_monkey(monkey_instance, vec![UpgradePath::Top]);
                    println!("Upgrade 3");
                    return true;
                }
                false
            }
            3 => {
                // Upgrade top path to 3
                if money > 1890 {
                    upgrade_monkey(monkey_instance, vec![UpgradePath::Top]);
                    println!("Upgrade 4");
                    return true;
                }
                false
            }
            4 => {
                // Upgrade middle path to 1
                if money > 325 {
                    upgrade_monkey(monkey_instance, vec![UpgradePath::Middle]);
                    println!("Upgrade 5");
                    return true;
                }
                false
            }
            5 => {
                // Upgrade middle path to 2
                if money > 650 {
                    upgrade_monkey(monkey_instance, vec![UpgradePath::Middle]);
                    println!("Upgrade 6");
                    return true;
                }
                false
            }
            6 => {
                // Upgrade top path to 4
                if money > 21170 {
                    upgrade_monkey(monkey_instance, vec![UpgradePath::Top]);
                    println!("Upgrade 7");
                    return true;
                }
                false
            }
            7 => {
                // Upgrade top path to 5
                if money > 48600 {
                    upgrade_monkey(monkey_instance, vec![UpgradePath::Top]);
                    println!("Upgrade 8");
                    return true;
                }
                false
            }
            default => false,
        }
    }

    // Uses OCR to read money
    fn read_money_data(screen: &Screen) -> i32 {
        // OCR it
        let mut text = Self::ocr_area(screen, 334, 13, 225, 64).unwrap();

        // Interpret
        text = text
            .replace("$", "")
            .replace(",", "")
            .replace(")", "")
            .replace("'", "")
            .replace(" ", "");
        text.parse().unwrap_or(0)
    }

    // Uses OCR to check for the Victory screen's "NEXT" button
    fn check_for_victory(screen: &Screen) -> bool {
        // Capture the screen
        let screenshot = screen.capture_area(892, 883, 133, 47).unwrap();

        // Get the desktop directory
        let desktop_dir = dirs::desktop_dir().unwrap();

        // Specify the path for saving the screenshot
        let screenshot_path = desktop_dir.join("bloons_ocr.png");

        // Save the screenshot to the desktop
        let _ = screenshot.save(&screenshot_path);

        // OCR it
        let text = Self::ocr_area(screen, 892, 883, 133, 47)
            .unwrap()
            .to_ascii_uppercase()
            .replace(" ", "");
        println!("Victory screen button interpreter found: {}", text);

        // Interpret
        text.contains("NEXT")
    }

    fn check_for_early_defeat(screen: &Screen) -> bool {
        // OCR it
        let text = Self::ocr_area(screen, 574, 873, 109, 42)
            .unwrap()
            .to_ascii_uppercase()
            .replace(" ", "");
        println!("Victory screen button interpreter found: {}", text);

        // Interpret
        text.contains("HOME") || text.contains("UOME")
    }

    fn ocr_area(
        screen: &Screen,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<String, Box<dyn Error>> {
        let screenshot = screen.capture_area(x, y, width, height).unwrap();

        // Get the desktop directory
        let desktop_dir = dirs::desktop_dir().unwrap();

        // Specify the path for saving the screenshot
        let screenshot_path = desktop_dir.join("bloons_ocr.png");

        // Save the screenshot to the desktop
        let _ = screenshot.save(&screenshot_path);

        // OCR it
        png_to_text(screenshot_path)
    }
}

impl eframe::App for SapperApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("BLOONS SAPPER BOT");

            let money_ptr = self.current_money.clone();
            let money: i32 = *(money_ptr.lock()).unwrap();
            ui.label(format!("OCR Money: {}", money));

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
