use std::ops::{Add, Mul, Sub, Div};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn length(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn normalize(self) -> Vector {
        self / self.length()
    }

    pub fn cross(self, rhs: Vector) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

pub const O: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0
};

pub const I: Vector = Vector {
    x: 1.0,
    y: 0.0,
    z: 0.0
};

pub const J: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0
};

pub const K: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 1.0
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub source: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(source: Vector, direction: Vector) -> Ray {
        Ray {source: source, direction: direction.normalize()}
    }

    pub fn along(self, alpha: f64) -> Vector {
        self.source + alpha * self.direction
    }
}

#[test]
fn vector_test() {
    let a = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vector {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    let c = Vector {
        x: 1.0,
        y: 0.0,
        z: 0.0
    };
    assert!(a + b ==
        Vector {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        });
    assert!(2.0 * a ==
        Vector {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        });
    assert!(a * b == 12.0);
    assert!(c.length() == 1.0);
    assert!(c.normalize() == c);
}