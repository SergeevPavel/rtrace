
extern crate image;

mod math;

use std::path::Path;

use image::RgbImage;

use math::Vector;

#[derive(Debug, Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn to_u8(self) -> [u8; 3] {
        [(255.0 * self.r).round() as u8, (255.0 * self.g).round() as u8, (255.0 * self.b).round() as u8]
    }
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    source: Vector,
    direction: Vector,
}

trait Figure {
    fn intersect(self, ray: Ray) -> Option<Color>;
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    color: Color,
    center: Vector,
    radius: f64,
}

struct ChessBoard {
    colors: (Color, Color),
    o: Vector,
    a: Vector,
    b: Vector
}

impl Figure for Sphere {
    fn intersect(self, ray: Ray) -> Option<Color> {
        let u = ray.source - self.center;
        let a = ray.direction * ray.direction;
        let b = 2.0 * u * ray.direction;
        let c = u * u - self.radius * self.radius;
        let desc = b * b - 4.0 * a * c;
        if desc >= 0.0 {
            let alpha = (-b + desc.sqrt()) / (2.0 * a);
            if alpha > 0.0 {
                Some(self.color)
            } else {
                None
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
    objects: Vec<Sphere>,
}

fn render(scene: &Scene, x_res: u32, y_res: u32) -> RgbImage {
    let camera = scene.camera;
    let mut img = image::ImageBuffer::new(x_res, y_res);
    let background_color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0
    };

    for (i, j, p) in img.enumerate_pixels_mut() {
        let alpha = (i as f64) / (x_res as f64) - 0.5;
        let beta = (j as f64) / (y_res as f64) - 0.5;
        let ray = Ray {
            source: camera.position,
            direction: camera.forward + alpha * camera.right + beta * camera.up,
        };
        for obj in &scene.objects {
            let c = obj.clone().intersect(ray).unwrap_or(background_color);
            *p = image::Rgb(c.to_u8())
        }
    }
    img
}

fn load_scene() -> Scene {
    Scene {
        camera: Camera {
            position: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            forward: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            up: Vector {
                x: 0.0,
                y: 3.0,
                z: 0.0,
            },
            right: Vector {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
        },
        objects: vec![Sphere {
                          center: Vector {
                              x: 0.0,
                              y: 0.0,
                              z: 3.0,
                          },
                          color: Color {
                              r: 0.0,
                              g: 1.0,
                              b: 0.0,
                          },
                          radius: 1.0,
                      }],
    }
}

fn main() {
    let x_res = 1024;
    let y_res = 768;
    let scene = load_scene();
    let img = render(&scene, x_res, y_res);
    let _ = img.save(&Path::new("out.png"));
}
