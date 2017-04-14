
extern crate image;

mod math;
mod color;
mod material;
mod scene;
mod figures;

use std::path::Path;
use std::f64;

use image::RgbImage;

use math::{Ray, Vector, I, J, K, O};
use color::*;
use scene::*;
use figures::*;


fn render(scene: Scene, x_res: u32, y_res: u32) -> RgbImage {
    let camera = scene.camera;
    let mut img = image::ImageBuffer::new(x_res, y_res);
    let background_color = BLACK;

    for (i, j, p) in img.enumerate_pixels_mut() {
        let alpha = (i as f64) / (x_res as f64) - 0.5;
        let beta = (j as f64) / (y_res as f64) - 0.5;
        let ray = Ray::new(camera.position,
                           camera.forward + alpha * camera.right - beta * camera.up);
        *p = image::Rgb(background_color.to_u8());
        if let Some(IntersectionPoint { alpha, color, material, n }) = scene.trace(ray) {
            let ambient_color = scene.ambient_light.mix(color).bright(material.ambient);
            let to_light = (scene.spotlight.position - ray.along(alpha)).normalize();
            const EPS: f64 = 1e-5;
            let color = if let None = scene.trace(Ray::new(ray.along(alpha - EPS), to_light)) {
                let diffuse_color = scene.spotlight.color.mix(color).bright((material.diffuse * (to_light * n)).max(0.0));
                let reflected = 2.0 * n * to_light * n - to_light;
                let to_viewer = -1.0 * ray.direction;
                let specular_color = scene.spotlight.color.mix(color).bright((material.specular * reflected * to_viewer).max(0.0));
                ambient_color.add(diffuse_color).add(specular_color)
            } else {
                ambient_color
            };
            *p = image::Rgb(color.to_u8());
        }
    }
    img
}

fn main() {
    let x_res = 1024;
    let y_res = 768;
    let scene = load_scene();
    let img = render(scene, x_res, y_res);
    let _ = img.save(&Path::new("out.png"));
}
