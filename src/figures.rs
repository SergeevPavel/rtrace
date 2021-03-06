
use color::Color;
use material::*;
use math::{Vector, Ray};

pub struct IntersectionPoint {
    pub alpha: f64,
    pub color: Color,
    pub material: Material,
    pub n: Vector
}

impl IntersectionPoint {
    pub fn new(alpha: f64, color: Color, material: Material, n: Vector) -> IntersectionPoint {
        IntersectionPoint {
            alpha: alpha,
            color: color,
            material: material,
            n: n
        }
    }
}

pub trait Figure {
    fn intersect(&self, ray: Ray) -> Option<IntersectionPoint>;
    fn material(&self) -> Material;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub color: Color,
    pub center: Vector,
    pub radius: f64,
    pub material: Material
}

#[derive(Debug, Clone, Copy)]
pub struct ChessBoard {
    pub colors: (Color, Color),
    pub o: Vector,
    pub a: Vector,
    pub b: Vector,
    pub material: Material
}

impl Figure for Sphere {
    fn intersect(&self, ray: Ray) -> Option<IntersectionPoint> {
        let u = ray.source - self.center;
        let a = ray.direction * ray.direction;
        let b = 2.0 * u * ray.direction;
        let c = u * u - self.radius * self.radius;
        let desc = b * b - 4.0 * a * c;
        if desc >= 0.0 {
            let alpha = (-b - desc.sqrt()) / (2.0 * a);
            if alpha > 0.0 {
                let p = ray.along(alpha);
                let n = (p - self.center).normalize();
                Some(IntersectionPoint::new(alpha, self.color, self.material, n))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }
}

impl Figure for ChessBoard {
    fn intersect(&self, ray: Ray) -> Option<IntersectionPoint> {
        let n = self.a.cross(self.b).normalize();
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
                Some(IntersectionPoint::new(gamma, self.colors.0, self.material, n))
            } else {
                Some(IntersectionPoint::new(gamma, self.colors.1, self.material, n))
            }
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }
}