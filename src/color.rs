

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn to_u8(self) -> [u8; 3] {
        [(255.0 * self.r).round() as u8,
         (255.0 * self.g).round() as u8,
         (255.0 * self.b).round() as u8]
    }
}

pub const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
};
pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
};
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};
