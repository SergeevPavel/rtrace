

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64
}

pub const METAL: Material = Material {
    ambient: 0.5,
    diffuse: 0.2,
    specular: 0.6
};

pub const PLASTIC: Material = Material {
    ambient: 0.1,
    diffuse: 1.0,
    specular: 0.2
};