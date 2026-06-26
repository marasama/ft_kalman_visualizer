use eframe::egui;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Kalman Visualizer",
        options,
        Box::new(|cc| Ok(Box::<KalmanVisualizer>::default())),
    )
}

struct KalmanVisualizer {
    name: String,
    age: u32,
}

impl Default for KalmanVisualizer {
    fn default() -> Self {
        Self {
            name: String::from("Zort"),
            age: 31,
        }
    }
}

impl KalmanVisualizer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for KalmanVisualizer {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("DENEME");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your Name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            if ui.button("Decrement").clicked() {
                self.age -= 1;
            }
            ui.label(format!("Hello {} -> your age {}", self.name, self.age));
        });
    }
}
