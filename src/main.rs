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
    left_width: f32,
    filter_gain: f32,
    show_trajectory: bool,
    status: String,
}

impl KalmanVisualizer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            left_width: 200.0,
            filter_gain: 0.1,
            show_trajectory: true,
            status: "Ready".to_string(),
        }
    }
}

impl eframe::App for KalmanVisualizer {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel")
            .resizable(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("ft_kalman");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(&self.status);
                    });
                });
            });
        egui::Panel::bottom("bottom_panel")
            .resizable(true)
            .default_size(400.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("IMU 100HZ!");
                    ui.separator();
                    ui.label("GPS active!");
                    ui.separator();
                    ui.label("Filter: LKF");
                    ui.separator();
                });
            });
        egui::Panel::left("left_panel")
            .default_size(220.)
            .resizable(true)
            .show(ui, |ui| {
                ui.heading("Controls");
                ui.separator();

                ui.add(egui::Slider::new(&mut self.filter_gain, 0.0..=1.0).text("Process Noise"));
                ui.checkbox(&mut self.show_trajectory, "Show Trajectory");
                ui.separator();
                egui::CollapsingHeader::new("Advanced")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.label("Q Matrix Tuning");
                        ui.label("R Matrix Tuning");
                    });
            });
        egui::CentralPanel::default().show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

            let rect = response.rect;

            painter.rect_filled(rect, 50.0, egui::Color32::from_rgb(15, 15, 20));

            let stroke = egui::Stroke::new(0.5, egui::Color32::from_rgb(80, 80, 100));

            let step = 40.0;
            let mut x = rect.left();

            while x < rect.right() {
                painter.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    stroke,
                );
                x += step;
            }
            let mut y = rect.top();

            while y < rect.bottom() {
                painter.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    stroke,
                );
                y += step;
            }

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "3D Viewport",
                egui::FontId::proportional(16.0),
                egui::Color32::from_rgb(80, 80, 100),
            );
            if response.dragged() {
                self.status = format!(
                    "drag Δ ({:.1}, {:.1})",
                    response.drag_delta().x,
                    response.drag_delta().y
                );
            }
        });
    }
}
