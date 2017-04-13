
use math::*;
use figures::{Figure, Sphere, ChessBoard};
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
                                   center: J + 10.0 * K - 1.5 * I,
                                   color: GREEN,
                                   radius: 1.0,
                                   material: PLASTIC,
                               }),
                      Box::new(Sphere {
                                   center: J + 15.0 * K + 1.5 * I,
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
            position: 2.0 * K + 5.0 * J,
        },
        ambient_light: WHITE.bright(0.2),
    }
}
