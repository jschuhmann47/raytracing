use std::ops::{Add, Mul};

// Values between 0 and 1
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }
    pub fn to_line(&self) -> String {
        format!("{} {} {}\n", self.x, self.y, self.z)
    }

    pub fn to_u8_range(&self) -> Self {
        Self {
            x: (self.x * 255.999).trunc(),
            y: (self.y * 255.999).trunc(),
            z: (self.z * 255.999).trunc(),
        }
    }

    pub fn scalar_mul(&self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
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
