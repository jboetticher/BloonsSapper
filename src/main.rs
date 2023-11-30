#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
        Box::new(|cc| Box::new(SapperApp::new(cc))),
    )
}

pub struct SapperApp {
    current_money: i32,
}

impl SapperApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        SapperApp {
            current_money: 0
        }
    }
}

impl eframe::App for SapperApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("BLOONS SAPPER BOT");

            ui.horizontal(|ui| {
                ui.label("You gotta bot");
                ui.label("more text")
            });

            ui.horizontal(|ui| {
                ui.label(format!("OCR Money: {}", self.current_money));
            });

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
