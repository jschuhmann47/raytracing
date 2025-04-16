use std::ops::{Add, Mul, Sub};

// Values between 0 and 1
#[derive(Clone, Copy)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }
    pub fn to_line(self) -> String {
        format!("{} {} {}\n", self.x, self.y, self.z)
    }

    pub fn to_u8_range(self) -> Self {
        Self {
            x: (self.x * 255.999).trunc(),
            y: (self.y * 255.999).trunc(),
            z: (self.z * 255.999).trunc(),
        }
    }

    pub fn scalar_mul(self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    pub fn normalize(self) -> Self {
        let length = self.length();
        Self { x: self.x / length, y: self.y / length, z: self.z /length }
    }

    fn length(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul for Vector3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    } 
}

pub fn random_color() -> Vector3D {
    Vector3D { x: 0.1, y: 0.2, z: 0.3 }
}