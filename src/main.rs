use std::fmt::format;
use std::time::Instant;

use eframe::egui;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Kalman Visualizer",
        options,
        Box::new(|_cc| Ok(Box::<KalmanVisualizer>::default())),
    )
}

#[derive(Default)]
struct KalmanVisualizer {
    selected_tab: usize,
}

impl KalmanVisualizer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for KalmanVisualizer {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Uçuş Bilgisayarı");
                ui.separator();
                ui.selectable_value(&mut self.selected_tab, 0, "3D View");
                ui.selectable_value(&mut self.selected_tab, 1, "Charts");
                ui.selectable_value(&mut self.selected_tab, 2, "Logs");
            });
        });
        egui::Panel::left("left_panel")
            .resizable(true)
            .default_size(200.)
            .show(ui, |ui| {
                ui.label("Filter Controls");
                ui.separator();
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(format!(
                "Tab: {}",
                ["3D View", "Charts", "Logs"][self.selected_tab]
            ));
        });
    }
}
