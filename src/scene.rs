
use std::f64;

use math::*;
use figures::{Figure, Sphere, ChessBoard, IntersectionPoint};
use color::*;
use material::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vector,
    pub forward: Vector,
    pub up: Vector,
    pub right: Vector,
}

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<Figure>>,
    pub spotlight: Spotlight,
    pub ambient_light: Color,
}

impl Scene {
    pub fn trace(&self, ray: Ray) -> Option<IntersectionPoint> {
        let mut min = f64::INFINITY;
        let mut result: Option<IntersectionPoint> = None;
        for obj in self.objects.iter() {
            if obj.intersect(ray).is_some() {
                let ip = obj.intersect(ray).unwrap();
                let IntersectionPoint { alpha, color, material, n } = ip;
                if alpha < min {
                    min = alpha;
                    result = Some(ip);
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spotlight {
    pub color: Color,
    pub position: Vector,
}

pub fn load_scene() -> Scene {
    Scene {
        camera: Camera {
            position: 5.0 * J,
            forward: 2.5 * K,
            up: 3.0 * J,
            right: 4.0 * I,
        },
        objects: vec![Box::new(Sphere {
                                   center: 2.0 * J + 13.0 * K - 1.5 * I,
                                   color: GREEN,
                                   radius: 2.0,
                                   material: METAL,
                               }),
                      Box::new(Sphere {
                                   center: J + 10.0 * K + 1.5 * I,
                                   color: RED,
                                   radius: 1.0,
                                   material: METAL
                               }),
                      Box::new(ChessBoard {
                                   colors: (BLACK, WHITE),
                                   o: O,
                                   a: 3.0 * I,
                                   b: 3.0 * K,
                                   material: PLASTIC

                               })],
        spotlight: Spotlight {
            color: WHITE,
            position: 3.0 * I + 8.0 * K + 5.0 * J,
        },
        ambient_light: WHITE.bright(0.2),
    }
}
