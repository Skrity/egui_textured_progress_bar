#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::Color32;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Progress Bars",
        options,
        Box::new(|_cc| {
            _cc.egui_ctx.set_pixels_per_point(2.0);
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    progress: f32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add(egui::DragValue::new(&mut self.progress));
            let progress = self.progress / 100.0;
            ui.add(
                egui_textured_progress_bar::ProgressBar::new(progress)
                    .text("Basic")
                    .animate(true)
                    .fill(Color32::GREEN),
            );
            ui.add(
                egui_textured_progress_bar::ProgressBar::new(progress)
                    .text("With texture")
                    .animate(true)
                    .fill(Color32::YELLOW)
                    .texture(progress_bar_texture(ui)),
            );
            ui.add(
                egui_textured_progress_bar::ProgressBar::new(progress)
                    .text("With texture + custom corners")
                    .animate(true)
                    .corner_radius(5)
                    .fill(Color32::LIGHT_RED)
                    .texture(progress_bar_texture(ui)),
            );
            ui.add(
                egui_textured_progress_bar::ProgressBar::new(progress)
                    .animate(true)
                    .fill(Color32::LIGHT_YELLOW)
                    .text("XDD")
                    .show_percentage()
                    .texture(progress_bar_texture(ui)),
            );
            ui.add(
                egui_textured_progress_bar::ProgressBar::new(progress)
                    .text("Different")
                    .animate(true)
                    .fill(Color32::LIGHT_GREEN)
                    .corner_radius(7)
                    .texture(progress_bar_texture(ui)),
            );
            ui.add(
                egui_textured_progress_bar::ProgressBar::new(progress)
                    .text("Non animated")
                    .animate(false)
                    .fill(Color32::LIGHT_BLUE)
                    .text("XDD")
                    .corner_radius(7)
                    .texture(progress_bar_texture(ui)),
            );
        });
    }
}
/// Texture for progress bar
fn progress_bar_texture(ui: &eframe::egui::Ui) -> &eframe::egui::TextureHandle {
    static TEXTURE: std::sync::OnceLock<eframe::egui::TextureHandle> = std::sync::OnceLock::new();
    TEXTURE.get_or_init(|| {
        ui.load_texture(
            "progress_bar_texture",
            eframe::egui::ColorImage::from_rgba_unmultiplied(
                [20, 20],
                include_bytes!("texture.rgba"),
            ),
            eframe::egui::TextureOptions::LINEAR_REPEAT,
        )
    })
}
