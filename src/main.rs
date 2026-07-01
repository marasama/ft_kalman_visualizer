#![feature(generic_const_exprs)]
use eframe::NativeOptions;
use egui::{pos2, Color32, Key};
use matrix::matrix::funcs::inverse::identity_matrix;
use matrix::matrix::Matrix;
use matrix::vector::funcs::cross::cross_product;
use matrix::vector::Vector;
use std::f32::consts::PI;
use std::fs::File;
use std::io::{self, BufRead};
use std::isize;

macro_rules! deg_to_rad {
    ($deg:expr) => {
        ($deg) * (PI / 180.)
    };
}

macro_rules! mat_rot {
    ($axis:ident, $vec:expr, $angle:expr) => {
        {
            let (s, c) = ($angle.sin(), $angle.cos());
            let mat = mat_rot!(@internal $axis, s, c);
            mat.mul_vec($vec)
        }
    };

    (@internal X, $s:expr, $c:expr) => {
        Matrix::from([
            [1., 0., 0.],
            [0., $c, -$s],
            [0., $s, $c],
        ])
    };
    (@internal Y, $s:expr, $c:expr) => {
        Matrix::from([
            [$c, 0., $s],
            [0., 1., 0.],
            [-$s, 0., $c],
        ])
    };
    (@internal Z, $s:expr, $c:expr) => {
        Matrix::from([
            [$c, -$s, 0.],
            [$s, $c, 0.],
            [0., 0., 1.],
        ])
    };
}

fn read_obj_file(
    file_name: &str,
) -> io::Result<(Vec<Vector<f32, 3>>, Vec<Vector<f32, 3>>, Vec<[usize; 3]>)> {
    let obj_file = File::open(file_name)?;
    let reader = std::io::BufReader::new(obj_file);

    let mut vertices: Vec<Vector<f32, 3>> = Vec::new();
    let mut normals: Vec<Vector<f32, 3>> = Vec::new();
    let mut faces: Vec<[usize; 3]> = Vec::new();

    for line_unwrapped in reader.lines() {
        let line = line_unwrapped?;
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let coordinates: Vec<f32> = parts
                    .map(|p| p.parse::<f32>().expect("Invalid float value in file!"))
                    .collect();
                if coordinates.len() >= 3 {
                    vertices.push(Vector::from([
                        coordinates[0],
                        coordinates[1],
                        coordinates[2],
                    ]));
                }
            }
            Some("vn") => {
                let coordinates: Vec<f32> = parts
                    .map(|p| p.parse::<f32>().expect("Invalid float value in file!"))
                    .collect();
                if coordinates.len() >= 3 {
                    normals.push(Vector::from([
                        coordinates[0],
                        coordinates[1],
                        coordinates[2],
                    ]));
                }
            }
            Some("f") => {
                let indices: Vec<usize> = parts
                    .map(|p| {
                        let v_str = p.split('/').next().unwrap();
                        let idx: isize = v_str.parse().expect("Invalid usize value in file!");
                        if idx > 0 {
                            (idx - 1) as usize
                        } else {
                            (vertices.len() as isize + idx) as usize
                        }
                    })
                    .collect();
                if indices.len() >= 3 {
                    faces.push([indices[0], indices[1], indices[2]]);
                }
            }
            _ => {}
        }
    }

    Ok((vertices, normals, faces))
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
        [0., 0., depth_scaler, -near_plane * depth_scaler],
        [0., 0., 1., 0.],
    ])
}

fn project(
    position: &Vector<f32, 3>,
    view_mat: &Matrix<f32, 4, 4>,
    proj_mat: &Matrix<f32, 4, 4>,
) -> Vector<f32, 3>
where
    [(); 4 * 4]:,
{
    let pos4 = Vector::from([position.data[0], position.data[1], position.data[2], 1.]);

    let view_pos = view_mat.mul_vec(&pos4);
    let vec = proj_mat.mul_vec(&view_pos);

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

struct Camera {
    pos: Vector<f32, 3>,
    target: Vector<f32, 3>,
    up: Vector<f32, 3>,
}

struct World {
    vertices: Vec<Vector<f32, 3>>,
    normals: Vec<Vector<f32, 3>>,
    projection_matrix: Matrix<f32, 4, 4>,
    view_matrix: Matrix<f32, 4, 4>,
    camera: Camera,
    t: f32,
    faces: Vec<[usize; 3]>,
}

impl Default for World {
    fn default() -> Self {
        let (vertices, normals, faces) = read_obj_file("teapot.obj").unwrap();

        let camera = Camera {
            pos: Vector::from([0., 0., -10.]),
            target: Vector::from([0., 0., 0.]),
            up: Vector::from([0., 1., 0.]),
        };

        World {
            vertices,
            normals,
            projection_matrix: create_projection_matrix(800. / 600., deg_to_rad!(90.), 1000., 0.1),
            view_matrix: identity_matrix::<f32, 4>(),
            camera,
            t: 0.01,
            faces,
        }
    }
}

fn create_view_matrix(camera: &Camera) -> Matrix<f32, 4, 4>
where
    [(); 4 * 4]:,
{
    // Forward
    let c = camera.target.sub_vec_ref(&camera.pos).normalize().unwrap();
    // Right
    let a = cross_product(&camera.up, &c);
    // Up
    let b = cross_product(&c, &a);

    let t = &camera.pos;

    Matrix::from([
        [a.data[0], a.data[1], a.data[2], -t.dot(&a)],
        [b.data[0], b.data[1], b.data[2], -t.dot(&b)],
        [c.data[0], c.data[1], c.data[2], -t.dot(&c)],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

impl eframe::App for World {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let rect = ui.max_rect();
            let width = rect.width();
            let height = rect.height();
            self.projection_matrix =
                create_projection_matrix(width / height, deg_to_rad!(90.), 1000., 0.1);

            self.view_matrix = create_view_matrix(&self.camera);
            let painter = ui.painter();
            let to_screen = |ndc: Vector<f32, 3>| {
                let x = rect.min.x + (ndc.data[0] + 1.) * 0.5 * width;
                let y = rect.min.y + (1. - ndc.data[1]) * 0.5 * height;
                pos2(x, y)
            };
            let projected: Vec<egui::Pos2> = self
                .vertices
                .iter()
                .map(|vert| to_screen(project(vert, &self.view_matrix, &self.projection_matrix)))
                .collect();

            let stroke = egui::Stroke::new(0., egui::Color32::KHAKI);
            //for &(start, end) in &self.edges {
            //    painter.line_segment([projected[start], projected[end]], stroke);
            //}
            let normals: Vec<Vector<f32, 3>> = self
                .faces
                .iter()
                .map(|face| {
                    cross_product(
                        &self.vertices[face[2]].sub_vec_ref(&self.vertices[face[0]]),
                        &self.vertices[face[1]].sub_vec_ref(&self.vertices[face[0]]),
                    )
                    .normalize()
                    .unwrap()
                })
                .collect();

            let mut faces_depth: Vec<(usize, f32)> = self
                .faces
                .iter()
                .enumerate()
                .map(|(i, face)| {
                    let z0 = self.vertices[face[0]].data[2];
                    let z1 = self.vertices[face[1]].data[2];
                    let z2 = self.vertices[face[2]].data[2];

                    (i, (z0 + z1 + z2) / 3.0)
                })
                .collect();

            faces_depth.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let brightness: Vec<f32> = normals
                .iter()
                .map(|norm| norm.dot(&Vector::from([0., 0., 1.]).normalize().unwrap()))
                .collect();
            for (i, _face) in faces_depth {
                let path = egui::Shape::convex_polygon(
                    vec![
                        projected[self.faces[i][0]],
                        projected[self.faces[i][1]],
                        projected[self.faces[i][2]],
                    ],
                    Color32::from_rgb(
                        (brightness[i] * 255.) as u8,
                        (brightness[i] * 255.) as u8,
                        (brightness[i] * 255.) as u8,
                    ),
                    stroke,
                );
                painter.add(path);
            }
        });
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let radius = -30.;
        //self.camera.pos.data[0] = radius * self.t.sin();
        //self.camera.pos.data[1] = -radius * self.t.cos();
        //self.t += 0.02;
        let speed = 0.5;
        ctx.input(|k| {
            if k.key_down(egui::Key::W) {
                self.camera.pos.data[2] -= speed;
                self.camera.target.data[2] -= speed;
            }
            if k.key_down(egui::Key::S) {
                self.camera.pos.data[2] += speed;
                self.camera.target.data[2] += speed;
            }
            if k.key_down(egui::Key::A) {
                self.camera.target.data[0] -= speed;
                self.camera.pos.data[0] -= speed;
            }
            if k.key_down(egui::Key::D) {
                self.camera.target.data[0] += speed;
                self.camera.pos.data[0] += speed;
            }
            if k.key_down(egui::Key::E) {
                self.camera.target.data[1] -= speed;
                self.camera.pos.data[1] -= speed;
            }
            if k.key_down(egui::Key::Q) {
                self.camera.target.data[1] += speed;
                self.camera.pos.data[1] += speed;
            }
            if k.key_down(egui::Key::Space) {
                self.camera.pos = Vector::from([0., 10., -10.]);
                self.camera.target = Vector::from([0., 0., 0.]);
            }
        });
        ctx.request_repaint();
    }
}
