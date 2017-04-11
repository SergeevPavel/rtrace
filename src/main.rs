
extern crate image;

mod math;

use std::path::Path;
use std::f64;

use image::RgbImage;

use math::{Vector, I, J, K, O};

#[derive(Debug, Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn to_u8(self) -> [u8; 3] {
        [(255.0 * self.r).round() as u8,
         (255.0 * self.g).round() as u8,
         (255.0 * self.b).round() as u8]
    }
}

const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
};
const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
};
const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

#[derive(Debug, Clone, Copy)]
struct Ray {
    source: Vector,
    direction: Vector,
}

trait Figure {
    fn intersect(&self, ray: Ray) -> Option<(f64, Color)>;
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    color: Color,
    center: Vector,
    radius: f64,
}

#[derive(Debug, Clone, Copy)]
struct ChessBoard {
    colors: (Color, Color),
    o: Vector,
    a: Vector,
    b: Vector,
}

impl Figure for Sphere {
    fn intersect(&self, ray: Ray) -> Option<(f64, Color)> {
        let u = ray.source - self.center;
        let a = ray.direction * ray.direction;
        let b = 2.0 * u * ray.direction;
        let c = u * u - self.radius * self.radius;
        let desc = b * b - 4.0 * a * c;
        if desc >= 0.0 {
            let alpha = (-b - desc.sqrt()) / (2.0 * a);
            if alpha > 0.0 {
                Some((alpha, self.color))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Figure for ChessBoard {
    fn intersect(&self, ray: Ray) -> Option<(f64, Color)> {
        let n = self.a.cross(self.b);
        let gamma = ((self.o - ray.source) * n) / (ray.direction * n);
        if gamma > 0.0 {
            let v = ray.source + gamma * ray.direction;
            let ort_a = n.cross(self.a);
            let ort_b = n.cross(self.b);
            let alpha = v * ort_b / (self.a * ort_b);
            let beta = v * ort_a / (self.b * ort_a);
            let i = alpha.ceil() as u32;
            let j = beta.ceil() as u32;
            if ((i % 2) + (j % 2)) % 2 == 0 {
                Some((gamma, self.colors.0))
            } else {
                Some((gamma, self.colors.1))
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Camera {
    position: Vector,
    forward: Vector,
    up: Vector,
    right: Vector,
}

struct Scene {
    camera: Camera,
    objects: Vec<Box<Figure>>,
}

fn render(scene: Scene, x_res: u32, y_res: u32) -> RgbImage {
    let camera = scene.camera;
    let mut img = image::ImageBuffer::new(x_res, y_res);
    let background_color = BLACK;

    for (i, j, p) in img.enumerate_pixels_mut() {
        let alpha = (i as f64) / (x_res as f64) - 0.5;
        let beta = (j as f64) / (y_res as f64) - 0.5;
        let ray = Ray {
            source: camera.position,
            direction: (camera.forward + alpha * camera.right - beta * camera.up).normalize(),
        };
        let mut min = f64::INFINITY;
        *p = image::Rgb(background_color.to_u8());
        for obj in scene.objects.iter() {
            if obj.intersect(ray).is_some() {
                let (alpha, c) = obj.intersect(ray).unwrap();
                if alpha < min {
                    min = alpha;
                    *p = image::Rgb(c.to_u8())
                }
            }
        }
    }
    img
}

fn load_scene() -> Scene {
    Scene {
        camera: Camera {
            position: O + 5.0 * J,
            forward: 2.5 * K,
            up: 3.0 * J,
            right: 4.0 * I,
        },
        objects: vec![Box::new(Sphere {
                                   center: J + 10.0 * K - 1.5 * I,
                                   color: GREEN,
                                   radius: 1.0,
                               }),
                      Box::new(Sphere {
                                   center: J + 15.0 * K + 1.5 * I,
                                   color: RED,
                                   radius: 1.0,
                               }),
                      Box::new(ChessBoard {
                                   colors: (BLACK, WHITE),
                                   o: O,
                                   a: 3.0 * I,
                                   b: 3.0 * K,
                               })
        ],
    }
}

fn main() {
    let x_res = 1024;
    let y_res = 768;
    let scene = load_scene();
    let img = render(scene, x_res, y_res);
    let _ = img.save(&Path::new("out.png"));
}
