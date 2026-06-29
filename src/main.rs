#![feature(generic_const_exprs)]
use eframe::NativeOptions;
use egui::pos2;
use matrix::matrix::Matrix;
use matrix::vector::Vector;
use std::f32::consts::PI;

enum Pos {
    Pos2(Vector<f32, 2>),
    Pos3(Vector<f32, 3>),
    Pos4(Vector<f32, 4>),
}
macro_rules! deg_to_rad {
    ($deg:expr) => {
        ($deg) * (PI / 180.)
    };
}

fn create_projection_matrix(
    aspect_ratio: f32,
    angle: f32,
    far_plane: f32,
    near_plane: f32,
) -> Matrix<f32, 4, 4>
where
    [(); 4 * 4]:,
{
    let width_scaler = 1f32 / (angle / 2f32).tan();
    let depth_scaler = far_plane / (far_plane - near_plane);
    Matrix::from([
        [width_scaler / aspect_ratio, 0., 0., 0.],
        [0., width_scaler, 0., 0.],
        [0., 0., depth_scaler, 1.],
        [0., 0., -near_plane * depth_scaler, 0.],
    ])
}

fn project(position: &Vector<f32, 3>, proj_mat: &Matrix<f32, 4, 4>) -> Vector<f32, 3>
where
    [(); 4 * 4]:,
{
    let pos4 = Vector::from([position.data[0], position.data[1], position.data[2], 1.]);
    let vec = proj_mat.mul_vec(&pos4);
    Vector::from([
        vec.data[0] / vec.data[3],
        vec.data[1] / vec.data[3],
        vec.data[2] / vec.data[3],
    ])
}

fn main() {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800., 600.]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "World",
        native_options,
        Box::new(|_cc| Ok(Box::new(World::default()))),
    );
}

struct World {
    vertices: Vec<Vector<f32, 3>>,
    edges: Vec<(usize, usize)>,
    projection_matrix: Matrix<f32, 4, 4>,
    t: f32,
}

impl Default for World {
    fn default() -> Self {
        let z_offset = 13.0;
        let vertices = vec![
            Vector::from([-0.5, -0.5, z_offset - 0.5]),
            Vector::from([0.5, -0.5, z_offset - 0.5]),
            Vector::from([0.5, 0.5, z_offset - 0.5]),
            Vector::from([-0.5, 0.5, z_offset - 0.5]),
            Vector::from([-0.5, -0.5, z_offset + 0.5]),
            Vector::from([0.5, -0.5, z_offset + 0.5]),
            Vector::from([0.5, 0.5, z_offset + 0.5]),
            Vector::from([-0.5, 0.5, z_offset + 0.5]),
        ];
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];

        World {
            vertices,
            edges,
            projection_matrix: create_projection_matrix(800. / 600., deg_to_rad!(90.), 1000., 0.1),
            t: 0.,
        }
    }
}

fn rotate(vect: &mut Vector<f32, 3>, rad_x: f32, rad_z: f32) {
    let rot_mat_z = Matrix::from([
        [rad_z.cos(), -rad_z.sin(), 0.],
        [rad_z.sin(), rad_z.cos(), 0.],
        [0., 0., 1.],
    ]);

    let rot_mat_x = Matrix::from([
        [1., 0., 0.],
        [0., rad_x.cos(), -rad_x.sin()],
        [0., rad_x.sin(), rad_x.cos()],
    ]);

    *vect = rot_mat_z.mul_vec(&rot_mat_x.mul_vec(vect));
}

impl eframe::App for World {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let rect = ui.max_rect();
            let width = rect.width();
            let height = rect.height();

            let painter = ui.painter();
            let to_screen = |ndc: Vector<f32, 3>| {
                let x = rect.min.x + (ndc.data[0] + 1.) * 0.5 * width;
                let y = rect.min.y + (1. - ndc.data[1]) * 0.5 * height;
                pos2(x, y)
            };
            let projected: Vec<egui::Pos2> = self
                .vertices
                .iter()
                .map(|vert| to_screen(project(vert, &self.projection_matrix)))
                .collect();

            let stroke = egui::Stroke::new(1., egui::Color32::KHAKI);
            for &(start, end) in &self.edges {
                painter.line_segment([projected[start], projected[end]], stroke);
            }
        });
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let z_offset = 13.0;
        self.t += 0.00001;
        for vect in &mut self.vertices {
            vect.data[2] -= z_offset;
            rotate(vect, self.t, self.t);
            vect.data[2] += z_offset;
        }
        ctx.request_repaint();
    }
}
