
extern crate image;

mod math;

use std::path::Path;

use image::RgbImage;

use math::Vector;


#[derive(Debug, Clone, Copy)]
struct Ray {
    source: Vector,
    direction: Vector,
}

trait Figure {
    fn intersect(self, ray: Ray) -> bool;
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    center: Vector,
    radius: f64,
}

impl Figure for Sphere {
    fn intersect(self, ray: Ray) -> bool {
        let u = ray.source - self.center;
        let a = ray.direction * ray.direction;
        let b = 2.0 * u * ray.direction;
        let c = u * u - self.radius * self.radius;
        let desc = b * b - 4.0 * a * c;
        desc >= 0.0
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

    for (i, j, p) in img.enumerate_pixels_mut() {
        let alpha = (i as f64) / (x_res as f64) - 0.5;
        let beta = (j as f64) / (y_res as f64) - 0.5;
        let ray = Ray {
            source: camera.position,
            direction: camera.forward + alpha * camera.right + beta * camera.up,
        };
        for obj in &scene.objects {
            *p = if obj.clone().intersect(ray) {
                image::Rgb([255, 255, 255])
            } else {
                image::Rgb([0, 0, 0])
            }
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
                z: 0.0
            },
            forward: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0
            },
            up: Vector {
                x: 0.0,
                y: 3.0,
                z: 0.0
            },
            right: Vector {
                x: 4.0,
                y: 0.0,
                z: 0.0
            }
        },
        objects: vec!(Sphere {
            center: Vector {
                x: 0.0,
                y: 0.0,
                z: 3.0
            },
            radius: 1.0
        })
    }
}

fn main() {
    let x_res = 1024;
    let y_res = 768;
    let scene = load_scene();
    let img = render(&scene, x_res, y_res);
    let _ = img.save(&Path::new("out.png"));
}


#[test]
fn intersect_test() {
    let s = Sphere {
        center: Vector {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        radius: 2.0,
    };
    let forward = Ray {
        source: Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let up = Ray {
        source: Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    };
    assert_eq!(true, s.intersect(forward));
    assert_eq!(false, s.intersect(up))
}