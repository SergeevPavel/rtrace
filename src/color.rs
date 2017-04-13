

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn bright(self, v: f64) -> Color {
        Color {
            r: self.r * v,
            g: self.g * v,
            b: self.b * v
        }
    }

    pub fn mix(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b
        }
    }

    pub fn add(self, other: Color) -> Color {
        Color {
            r: (self.r + other.r).min(1.0),
            g: (self.g + other.g).min(1.0),
            b: (self.b + other.b).min(1.0)
        }
    }

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
